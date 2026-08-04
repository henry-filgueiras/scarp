use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use scarp::cli::{ArtifactTarget, Cli, Collection, Command, ProposalCommand};
use scarp::error::Error;
use scarp::read::Status;
use scarp::{artifact, doctor, fortune, proposal, read, repo, transition};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli.command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", error.render());
            ExitCode::from(error.exit_code())
        }
    }
}

/// Dispatch a parsed command.
fn run(command: &Command) -> Result<(), Error> {
    match command {
        Command::Init => init(),
        Command::New {
            collection,
            title,
            sprint,
            body_file,
            json,
        } => new_artifact(
            *collection,
            title,
            sprint.as_deref(),
            body_file.as_deref(),
            *json,
        ),
        Command::Proposal(command) => proposal_command(command),
        Command::List {
            collection,
            json,
            active,
        } => list(*collection, *json, *active),
        Command::Show { reference, json } => show(reference, *json),
        Command::Doctor { json } => doctor(*json),
        Command::Close {
            reference,
            resolved_by,
            body_file,
        } => close(reference, resolved_by.as_deref(), body_file.as_deref()),
        Command::Reopen { reference } => {
            transition(reference, "reopen", Collection::Dragon, Status::Open)
        }
        Command::Adopt {
            reference,
            adopted_by,
        } => adopt(reference, adopted_by.as_deref()),
        Command::Reject { reference } => {
            transition(reference, "reject", Collection::Idea, Status::Rejected)
        }
        Command::Fortune => fortune(),
        Command::Resolve { references, json } => resolve(references, *json),
        Command::Completions { shell } => {
            completions(*shell);
            Ok(())
        }
    }
}

/// Emit a completion script for `shell` on stdout. Generation is pure
/// (no repository discovery): the script is derived entirely from the
/// clap command definition, so it works outside any Scarp repository.
fn completions(shell: clap_complete::Shell) {
    use clap::CommandFactory;
    let mut command = Cli::command();
    clap_complete::generate(shell, &mut command, "scarp", &mut std::io::stdout());
}

/// Scan one command-line collection into the shared read model.
fn scan(root: &std::path::Path, collection: Collection) -> Result<Vec<read::Artifact>, Error> {
    match collection {
        Collection::Dragon => read::scan(root, &read::DRAGON),
        Collection::Idea => read::scan(root, &read::IDEA),
        Collection::Decision => read::scan(root, &read::DECISION),
        Collection::Log => read::scan(root, &read::LOG),
        Collection::Principle => read::scan(root, &read::PRINCIPLE),
        Collection::Maintenance => read::scan(root, &read::MAINTENANCE),
        Collection::Sprint => read::scan_sprints(root),
        Collection::Task => read::scan_tasks(root),
    }
}

/// Convert a command-line artifact target into a read-model selector.
fn selector(target: &ArtifactTarget) -> read::Selector<'_> {
    match target {
        ArtifactTarget::Reference(reference) => read::Selector::Sequence(reference.sequence),
        ArtifactTarget::Id(id) => read::Selector::Id(id),
    }
}

/// The transition-verb guidance for one collection, used when a verb is
/// applied to a reference outside its lifecycle.
fn verb_guidance(collection: Collection) -> &'static str {
    match collection {
        Collection::Dragon => "dragons close and reopen: use `scarp close` or `scarp reopen`",
        Collection::Idea => "ideas adopt or reject: use `scarp adopt` or `scarp reject`",
        Collection::Decision => "decisions have no lifecycle verbs; they are permanent records",
        Collection::Log => {
            "logs have no lifecycle verbs; a log records something that already happened"
        }
        Collection::Principle => {
            "principles have no lifecycle verbs; a principle advises until a later record supersedes it"
        }
        Collection::Maintenance => "maintenance items close: use `scarp close`",
        Collection::Sprint => "sprints close: use `scarp close`",
        Collection::Task => "tasks close: use `scarp close`",
    }
}

/// The lifecycle refusal for decisions (task 32): truthful guidance naming
/// the operation, the artifact, and why decisions have no such transition —
/// never a generic parse or not-found error.
fn decision_refusal(verb: &str, display: &str) -> Error {
    Error::InvalidInvocation {
        message: format!(
            "cannot {verb} `{display}`: it is a decision, and decisions have \
             no lifecycle transitions — an accepted decision is a permanent \
             record, and a changed position is recorded as a new decision \
             that supersedes it"
        ),
    }
}

/// Best-effort probe: does a cleanly parsed decision claim this stable id?
/// A failing decision scan answers no — the probe only upgrades a would-be
/// not-found diagnostic into the truthful decision refusal and must never
/// block another collection's lifecycle.
fn id_names_a_decision(root: &std::path::Path, id: &str) -> bool {
    read::scan(root, &read::DECISION)
        .map(|decisions| decisions.iter().any(|d| d.summary.id == id))
        .unwrap_or(false)
}

/// Transition one artifact between lifecycle states and render the outcome.
///
/// Each transition verb belongs to one collection's lifecycle; a reference
/// into another collection is refused with the verbs that do apply, rather
/// than resolved into a surprising rewrite.
fn transition(
    target: &ArtifactTarget,
    verb: &str,
    collection: Collection,
    to: Status,
) -> Result<(), Error> {
    if let ArtifactTarget::Reference(reference) = target
        && reference.collection != collection
    {
        if reference.collection == Collection::Decision {
            return Err(decision_refusal(verb, &target.to_string()));
        }
        return Err(Error::InvalidInvocation {
            message: format!(
                "`{target}` is a {} reference; {}",
                reference.collection,
                verb_guidance(reference.collection)
            ),
        });
    }
    let root = repo::discover(&cwd()?)?;
    if let ArtifactTarget::Id(id) = target
        && id_names_a_decision(&root, id)
    {
        return Err(decision_refusal(verb, id));
    }
    let done = transition::transition(
        &root,
        match collection {
            Collection::Dragon => &read::DRAGON,
            Collection::Idea => &read::IDEA,
            Collection::Decision => &read::DECISION,
            Collection::Log => &read::LOG,
            Collection::Principle => &read::PRINCIPLE,
            Collection::Maintenance => &read::MAINTENANCE,
            Collection::Sprint => &read::SPRINT,
            Collection::Task => &read::TASK,
        },
        selector(target),
        &target.to_string(),
        to,
    )?;
    render_transition(&done);
    Ok(())
}

/// `scarp adopt`: an idea transition that may carry its `adopted-by`
/// provenance in the same invocation.
fn adopt(target: &ArtifactTarget, adopted_by: Option<&str>) -> Result<(), Error> {
    if let ArtifactTarget::Reference(reference) = target
        && reference.collection != Collection::Idea
    {
        if reference.collection == Collection::Decision {
            return Err(decision_refusal("adopt", &target.to_string()));
        }
        return Err(Error::InvalidInvocation {
            message: format!(
                "`{target}` is a {} reference; {}",
                reference.collection,
                verb_guidance(reference.collection)
            ),
        });
    }
    let root = repo::discover(&cwd()?)?;
    if let ArtifactTarget::Id(id) = target
        && id_names_a_decision(&root, id)
    {
        return Err(decision_refusal("adopt", id));
    }
    let done = transition::transition_with_provenance(
        &root,
        &read::IDEA,
        selector(target),
        &target.to_string(),
        Status::Adopted,
        adopted_by.map(|raw| ("adopted-by", raw)),
    )?;
    render_transition(&done);
    Ok(())
}

/// `scarp close`: dragons and sprints share the verb, so the reference's
/// collection picks the lifecycle; a bare stable id resolves over the
/// union of the closable collections. `--resolved-by` records dragon
/// resolution provenance and belongs to no other collection's vocabulary.
fn close(
    target: &ArtifactTarget,
    resolved_by: Option<&str>,
    body_file: Option<&Path>,
) -> Result<(), Error> {
    let root = repo::discover(&cwd()?)?;
    let narrative = body_file.map(read_body_file).transpose()?;
    let narrative = narrative.as_deref();
    let collection = match target {
        ArtifactTarget::Reference(reference) => reference.collection,
        ArtifactTarget::Id(id) => {
            let union = || -> Result<Vec<read::Artifact>, Error> {
                let mut all = read::scan(&root, &read::DRAGON)?;
                all.extend(read::scan(&root, &read::DECISION)?);
                all.extend(read::scan(&root, &read::MAINTENANCE)?);
                all.extend(read::scan_sprints(&root)?);
                all.extend(read::scan_tasks(&root)?);
                Ok(all)
            };
            let union = union().map_err(|err| err.blocking(id))?;
            let artifact = read::resolve(&union, read::Selector::Id(id), id)?;
            match artifact.summary.kind.as_str() {
                "sprint" => Collection::Sprint,
                "task" => Collection::Task,
                "decision" => Collection::Decision,
                "maintenance" => Collection::Maintenance,
                _ => Collection::Dragon,
            }
        }
    };
    if collection == Collection::Decision {
        return Err(decision_refusal("close", &target.to_string()));
    }
    if resolved_by.is_some() && collection != Collection::Dragon {
        return Err(Error::InvalidInvocation {
            message: format!(
                "`--resolved-by` records dragon resolution provenance; the \
                 decided vocabulary has no such edge for {}s",
                collection.name()
            ),
        });
    }
    let done = match collection {
        Collection::Dragon => transition::transition_closing(
            &root,
            &read::DRAGON,
            selector(target),
            &target.to_string(),
            Status::Closed,
            resolved_by.map(|raw| ("resolved-by", raw)),
            narrative,
        )?,
        Collection::Sprint => {
            transition::close_sprint(&root, selector(target), &target.to_string(), narrative)?
        }
        Collection::Task => transition::transition_closing(
            &root,
            &read::TASK,
            selector(target),
            &target.to_string(),
            Status::Closed,
            None,
            narrative,
        )?,
        Collection::Maintenance => transition::transition_closing(
            &root,
            &read::MAINTENANCE,
            selector(target),
            &target.to_string(),
            Status::Closed,
            None,
            narrative,
        )?,
        Collection::Idea => {
            return Err(Error::InvalidInvocation {
                message: format!(
                    "`{target}` is an idea reference; {}",
                    verb_guidance(Collection::Idea)
                ),
            });
        }
        Collection::Log => {
            return Err(Error::InvalidInvocation {
                message: format!(
                    "`{target}` is a log reference; {}",
                    verb_guidance(Collection::Log)
                ),
            });
        }
        Collection::Principle => {
            return Err(Error::InvalidInvocation {
                message: format!(
                    "`{target}` is a principle reference; {}",
                    verb_guidance(Collection::Principle)
                ),
            });
        }
        Collection::Decision => unreachable!("decision targets are refused before dispatch"),
    };
    render_transition(&done);
    Ok(())
}

/// Render one performed transition.
fn render_transition(done: &transition::Transition) {
    let verb = match done.to {
        Status::Closed => "closed",
        Status::Open => "reopened",
        Status::Adopted => "adopted",
        Status::Rejected => "rejected",
        Status::Parked => "parked",
        Status::Active => "activated",
        Status::Pending => "pended",
        Status::Accepted => "accepted",
    };
    println!(
        "{verb} {} ({} -> {}) at {}",
        done.reference, done.from, done.to, done.path
    );
}

/// Surface one open dragon or parked idea, weighted toward stale artifacts.
fn fortune() -> Result<(), Error> {
    let root = repo::discover(&cwd()?)?;
    let dragons = read::scan(&root, &read::DRAGON)?;
    let ideas = read::scan(&root, &read::IDEA)?;
    // The candidate pool is every artifact still owed attention: open
    // dragons and parked ideas. Terminal states never resurface.
    let pool: Vec<_> = dragons
        .iter()
        .filter(|artifact| artifact.summary.status == Some(Status::Open))
        .chain(
            ideas
                .iter()
                .filter(|artifact| artifact.summary.status == Some(Status::Parked)),
        )
        .collect();
    if pool.is_empty() {
        println!(
            "no open dragons or parked ideas — nothing lurks; record a risk \
             with `scarp new dragon \"<title>\"` or park a proposal with \
             `scarp new idea \"<title>\"`"
        );
        return Ok(());
    }
    let today = jiff::Zoned::now().date();
    let ages: Vec<_> = pool
        .iter()
        .map(|artifact| fortune::age_days(&artifact.summary.created, today))
        .collect();
    let weights: Vec<u64> = ages.iter().map(|age| fortune::weight(*age)).collect();
    // The draw's entropy comes from a fresh ULID's 80-bit random component;
    // selection itself is the pure, tested `pick`.
    let index = fortune::pick(&weights, ulid::Ulid::new().random());
    let chosen = pool[index];
    println!("{}  {}", chosen.summary.reference(), chosen.summary.title);
    println!(
        "{}  {}",
        fortune::age_text(ages[index]),
        chosen.summary.path
    );
    let excerpt = fortune::excerpt(&chosen.content, 3);
    if !excerpt.is_empty() {
        println!();
        for line in &excerpt {
            println!("  {line}");
        }
    }
    Ok(())
}

/// The `resolve --json` projection for one input: the input as given plus
/// the resolved artifact's identity fields. Field names and order are a
/// compatibility surface pinned by tests; `title` is included because the
/// demonstrated consumer assembles `[[id|label]]` markers, where the
/// frozen label comes from the title.
#[derive(serde::Serialize)]
struct ResolveRecord<'a> {
    input: String,
    kind: &'a str,
    id: &'a str,
    sequence: u32,
    reference: String,
    path: &'a str,
    title: &'a str,
}

/// `scarp resolve` (task 38): map references to stable ids, one output
/// line per input in input order. All-or-nothing on stdout — if any input
/// fails, stdout emits nothing, every failure is reported on stderr in
/// input order under the decision 4 contract, and the exit code is the
/// last failure's. No in-band not-found sentinel ever reaches stdout: an
/// empty stdout composes safely with `&&` and `$(...)`.
fn resolve(targets: &[ArtifactTarget], json: bool) -> Result<(), Error> {
    let root = repo::discover(&cwd()?)?;
    // One strict scan of every managed collection serves the whole batch;
    // a blocking sibling names the batch it blocked (decision 13).
    let display_all = targets
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    let collections: [(&str, Vec<read::Artifact>); 5] = (|| -> Result<_, Error> {
        Ok([
            ("dragon", read::scan(&root, &read::DRAGON)?),
            ("idea", read::scan(&root, &read::IDEA)?),
            ("decision", read::scan(&root, &read::DECISION)?),
            ("sprint", read::scan_sprints(&root)?),
            ("task", read::scan_tasks(&root)?),
        ])
    })()
    .map_err(|err| err.blocking(&display_all))?;
    // Bare ids resolve over the union: sequences are collection-scoped,
    // identities are global.
    let union: Vec<read::Artifact> = collections
        .iter()
        .flat_map(|(_, artifacts)| artifacts.iter().cloned())
        .collect();

    let mut resolved = Vec::new();
    let mut failures = Vec::new();
    for target in targets {
        let display = target.to_string();
        let result = match target {
            ArtifactTarget::Reference(reference) => {
                let pool = collections
                    .iter()
                    .find(|(kind, _)| *kind == reference.collection.name())
                    .map(|(_, artifacts)| artifacts.as_slice())
                    .expect("every CLI collection has a scanned pool");
                read::resolve(pool, read::Selector::Sequence(reference.sequence), &display)
            }
            ArtifactTarget::Id(id) => read::resolve(&union, read::Selector::Id(id), &display),
        };
        match result {
            Ok(artifact) => resolved.push((display, artifact)),
            Err(error) => failures.push(error),
        }
    }

    // The returned error is rendered by `main` after the ones printed
    // here, so returning the last failure keeps stderr in input order.
    if let Some(last) = failures.pop() {
        for failure in &failures {
            eprintln!("{}", failure.render());
        }
        return Err(last);
    }

    if json {
        let records: Vec<ResolveRecord<'_>> = resolved
            .iter()
            .map(|(input, artifact)| ResolveRecord {
                input: input.clone(),
                kind: &artifact.summary.kind,
                id: &artifact.summary.id,
                sequence: artifact.summary.sequence,
                reference: artifact.summary.reference(),
                path: &artifact.summary.path,
                title: &artifact.summary.title,
            })
            .collect();
        println!("{}", to_json(&records));
    } else {
        for (_, artifact) in &resolved {
            println!("{}", artifact.summary.id);
        }
    }
    Ok(())
}

/// Parse a `--sprint` value into a sprint target (decision 15): `sprint:N`
/// or an addressable stable id. A sequence reference into any other
/// collection cannot name a sprint and is refused.
fn parse_sprint_selector(raw: &str) -> Result<(ArtifactTarget, String), Error> {
    let target: ArtifactTarget =
        raw.parse()
            .map_err(|reason: String| Error::InvalidInvocation {
                message: format!("invalid `--sprint {raw}`: {reason}"),
            })?;
    if let ArtifactTarget::Reference(reference) = &target
        && reference.collection != Collection::Sprint
    {
        return Err(Error::InvalidInvocation {
            message: format!(
                "`--sprint {raw}` names a {} reference; the owning sprint is \
                 selected as `sprint:N` or by a sprint's stable id",
                reference.collection
            ),
        });
    }
    Ok((target, raw.to_string()))
}

/// Resolve the current working directory.
fn cwd() -> Result<PathBuf, Error> {
    std::env::current_dir().map_err(|source| Error::Filesystem {
        operation: "resolve current directory".into(),
        path: PathBuf::from("."),
        source,
    })
}

/// Create an artifact in the enclosing repository and render the outcome.
///
/// Human and JSON projections consume the same semantic result: the
/// created artifact plus its decision 13 reachability. Flat creation
/// (dragons, ideas) runs the observational post-write probe; sprint and
/// task creation performed their strict containment scans before
/// writing, so a successful write is reachable by construction. A
/// degraded creation stays exit 0 — the write happened — with the stable
/// `warning[degraded-repository]:` line on stderr, leaving stdout (human
/// line or JSON object) unpolluted.
/// `scarp proposal`: list or realize remote proposals.
///
/// Realization creates a file and stops. It never commits and never
/// pushes: the operator reviews the artifact and commits it through the
/// ordinary workflow, which is the whole point of the design.
fn proposal_command(command: &ProposalCommand) -> Result<(), Error> {
    let root = repo::discover(&cwd()?)?;
    match command {
        ProposalCommand::List { json } => {
            let proposals = proposal::list(&root)?;
            if *json {
                println!("{}", to_json(&proposals));
            } else if proposals.is_empty() {
                println!("no open proposals");
            } else {
                for p in &proposals {
                    match &p.realized_as {
                        Some(path) => {
                            println!("#{}  realized  {}  ({path})", p.number, p.title);
                        }
                        None => println!("#{}  open      {}", p.number, p.title),
                    }
                }
            }
            Ok(())
        }
        ProposalCommand::Realize { number, json } => {
            let created = proposal::realize(&root, *number)?;
            if *json {
                println!("{}", to_json(&created.record()));
            } else {
                println!(
                    "created {} {} at {}",
                    created.kind,
                    created.reference(),
                    created.relative_path.display()
                );
                println!("review it, then commit; Scarp does not commit or push");
            }
            Ok(())
        }
        ProposalCommand::Reconcile { number, json } => {
            let result = proposal::reconcile(&root, *number)?;
            if *json {
                println!("{}", to_json(&result));
            } else if result.outcome == "already-reconciled" {
                println!("proposal #{number} is already closed; nothing to do");
            } else {
                let reference = result.reference.as_deref().unwrap_or("the artifact");
                println!("reconciled proposal #{number}: closed, citing {reference}");
            }
            Ok(())
        }
    }
}

/// Read a `--body-file` payload as UTF-8.
///
/// Non-UTF-8 input is a read failure rather than a parse failure: Scarp's
/// narrative payloads are UTF-8 text, and a caller that hands over bytes
/// which are not text has made a filesystem-level mistake. Reported through
/// the same typed filesystem error every other read uses, so automated
/// callers need no special case.
fn read_body_file(path: &Path) -> Result<String, Error> {
    fs::read_to_string(path).map_err(|source| Error::Filesystem {
        operation: "read".to_string(),
        path: path.to_path_buf(),
        source,
    })
}

fn new_artifact(
    collection: Collection,
    title: &str,
    sprint: Option<&str>,
    body_file: Option<&Path>,
    json: bool,
) -> Result<(), Error> {
    // `--sprint` chooses a task's owning sprint (decision 15) and belongs
    // to no other kind's creation vocabulary.
    if sprint.is_some() && collection != Collection::Task {
        return Err(Error::InvalidInvocation {
            message: format!(
                "`--sprint` chooses the owning sprint for `scarp new task`; \
                 it does not apply to `scarp new {}`",
                collection.name()
            ),
        });
    }
    let selection = sprint.map(parse_sprint_selector).transpose()?;
    // Read before the repository is touched: an unreadable or non-UTF-8
    // body must fail with a filesystem error, not a half-created artifact.
    let body = body_file.map(read_body_file).transpose()?;
    let root = repo::discover(&cwd()?)?;
    // Bind the body's legal sugar at the write boundary, exactly as
    // `close --body-file` binds terminal narrative: authored prose that
    // becomes canonical through a Scarp write carries canonical markers,
    // whichever command performed the write. Unresolvable sugar refuses
    // creation here, before a sequence is allocated or a path is touched.
    let body = body
        .map(|raw| scarp::edges::bind_prose(&root, &raw))
        .transpose()?;
    let body = body.as_deref();
    let created = match collection {
        Collection::Dragon => artifact::create_dragon(&root, title, body)?,
        Collection::Idea => artifact::create_idea(&root, title, body)?,
        Collection::Decision => artifact::create_decision(&root, title, body)?,
        Collection::Log => artifact::create_log(&root, title, body)?,
        Collection::Principle => artifact::create_principle(&root, title, body)?,
        Collection::Maintenance => artifact::create_maintenance(&root, title, body)?,
        Collection::Sprint => artifact::create_sprint(&root, title, body)?,
        Collection::Task => artifact::create_task(
            &root,
            title,
            selection
                .as_ref()
                .map(|(target, display)| (selector(target), display.as_str())),
            body,
        )?,
    };
    let reachability = match collection {
        Collection::Dragon => artifact::probe_reachability(&root, &read::DRAGON, &created),
        Collection::Idea => artifact::probe_reachability(&root, &read::IDEA, &created),
        Collection::Decision => artifact::probe_reachability(&root, &read::DECISION, &created),
        Collection::Log => artifact::probe_reachability(&root, &read::LOG, &created),
        Collection::Principle => artifact::probe_reachability(&root, &read::PRINCIPLE, &created),
        Collection::Maintenance => {
            artifact::probe_reachability(&root, &read::MAINTENANCE, &created)
        }
        Collection::Sprint | Collection::Task => artifact::Reachability::Reachable,
    };
    if json {
        println!("{}", to_json(&created.record()));
    } else {
        println!(
            "created {} at {}",
            created.reference(),
            created.relative_path.display()
        );
    }
    if let artifact::Reachability::Degraded { blocker } = &reachability {
        eprintln!(
            "warning[degraded-repository]: created {} at {}, but repository \
             degradation currently blocks normal access — {blocker}; the \
             artifact was created and the exit status remains success; \
             repairing the blocker restores normal access",
            created.reference(),
            created.relative_path.display()
        );
    }
    Ok(())
}

/// List a collection's artifacts and render the requested projection.
///
/// `--active` narrows tasks to the union of every active sprint's tasks
/// (decision 15); it is meaningless for other collections and refused
/// rather than ignored.
fn list(collection: Collection, json: bool, active: bool) -> Result<(), Error> {
    let root = repo::discover(&cwd()?)?;
    let mut artifacts = scan(&root, collection)?;
    if active {
        if collection != Collection::Task {
            return Err(Error::InvalidInvocation {
                message: format!(
                    "`--active` filters tasks by the active sprints; it does \
                     not apply to `scarp list {}s`",
                    collection.name()
                ),
            });
        }
        let sprints = read::scan_sprints(&root)?;
        let active_ids: Vec<&str> = sprints
            .iter()
            .filter(|sprint| sprint.summary.status == Some(Status::Active))
            .map(|sprint| sprint.summary.id.as_str())
            .collect();
        // The union across every active sprint, in the scan's global
        // deterministic task order; closed sprints' tasks drop out.
        artifacts.retain(|task| {
            task.summary
                .sprint
                .as_deref()
                .is_some_and(|owner| active_ids.contains(&owner))
        });
    }
    if json {
        let summaries: Vec<_> = artifacts.iter().map(|a| &a.summary).collect();
        println!("{}", to_json(&summaries));
    } else if artifacts.is_empty() {
        println!(
            "no {} found; create one with `scarp new {} \"<title>\"`",
            collection.plural(),
            collection.name()
        );
    } else {
        for artifact in &artifacts {
            let summary = &artifact.summary;
            // The status column is dropped entirely for a stateless
            // collection rather than filled with a constant: every row
            // would carry the same word, which is noise in the human
            // projection for the same reason the key is omitted from the
            // JSON one.
            match summary.status {
                Some(status) => println!(
                    "{}  {:<8}  {}  ({})",
                    summary.reference(),
                    status,
                    summary.title,
                    summary.path
                ),
                None => println!(
                    "{}  {}  ({})",
                    summary.reference(),
                    summary.title,
                    summary.path
                ),
            }
        }
    }
    Ok(())
}

/// Resolve one artifact reference and render it.
///
/// A `collection:sequence` reference scans exactly that collection; a bare
/// stable id could live in any collection, so every managed collection is
/// scanned and the id resolved over the union.
fn show(target: &ArtifactTarget, json: bool) -> Result<(), Error> {
    let root = repo::discover(&cwd()?)?;
    // A sibling that blocks the strict scan gets the requested target
    // attached (decision 13): the diagnostic names what could not be
    // delivered as well as what blocked it.
    let display = target.to_string();
    let artifacts = match target {
        ArtifactTarget::Reference(reference) => {
            scan(&root, reference.collection).map_err(|err| err.blocking(&display))?
        }
        ArtifactTarget::Id(_) => {
            let union = || -> Result<Vec<read::Artifact>, Error> {
                let mut all = read::scan(&root, &read::DRAGON)?;
                all.extend(read::scan(&root, &read::IDEA)?);
                all.extend(read::scan(&root, &read::DECISION)?);
                all.extend(read::scan(&root, &read::LOG)?);
                all.extend(read::scan(&root, &read::PRINCIPLE)?);
                all.extend(read::scan(&root, &read::MAINTENANCE)?);
                all.extend(read::scan_sprints(&root)?);
                all.extend(read::scan_tasks(&root)?);
                Ok(all)
            };
            union().map_err(|err| err.blocking(&display))?
        }
    };
    let artifact = read::resolve(&artifacts, selector(target), &target.to_string())?;
    if json {
        println!("{}", to_json(&artifact.show_record()));
    } else {
        // The canonical file contents, byte-for-byte: no added newline.
        print!("{}", artifact.content);
    }
    Ok(())
}

/// Validate the enclosing repository and render every finding.
///
/// Findings are the stdout payload — human lines or a deterministic JSON
/// array — so `--json` output stays parseable even when validation fails;
/// an unhealthy repository is then reported through the error contract
/// (`unhealthy-repository`, exit code 9) on stderr.
fn doctor(json: bool) -> Result<(), Error> {
    let root = repo::discover(&cwd()?)?;
    let report = doctor::check(&root)?;
    if json {
        println!("{}", to_json(&report.findings));
    } else {
        for finding in &report.findings {
            let prefix = match finding.severity {
                doctor::Severity::Error => "",
                doctor::Severity::Advice => "advice  ",
            };
            println!(
                "{prefix}{}  {}: {}",
                finding.problem, finding.path, finding.detail
            );
        }
        if report.healthy() {
            let advice = report.findings.len();
            if advice > 0 {
                println!(
                    "doctor: {} artifact(s) checked, no problems found, \
                     {advice} advisory note(s)",
                    report.artifacts_checked
                );
            } else {
                println!(
                    "doctor: {} artifact(s) checked, no problems found",
                    report.artifacts_checked
                );
            }
        }
    }
    if report.healthy() {
        Ok(())
    } else {
        Err(Error::UnhealthyRepository {
            problems: report.problems(),
        })
    }
}

/// Serialize a projection built from plain strings and integers.
fn to_json<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("projections of plain data always serialize")
}

/// Initialize the current working directory and render the outcome.
fn init() -> Result<(), Error> {
    let cwd = cwd()?;
    let report = repo::init(&cwd)?;
    if report.already_initialized() {
        println!(
            "Scarp repository at `{}` is already initialized; nothing to change",
            cwd.display()
        );
    } else {
        println!("initialized Scarp repository at `{}`", cwd.display());
        for path in &report.created {
            println!("  created {}", path.display());
        }
    }
    Ok(())
}
