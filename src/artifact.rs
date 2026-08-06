//! Artifact creation for managed collections.
//!
//! Scarp owns the mechanics callers must not hand-roll: display sequence
//! allocation, deterministic slugging, stable identity assignment, and safe
//! writes. The filesystem stays canonical — a created artifact is an
//! ordinary Markdown file with YAML-style front matter.
//!
//! # Identity
//!
//! The `id` front matter field is an opaque stable string. Artifacts created
//! here use a per-collection prefix (`drg_`, `ide_`) followed by an
//! uppercase ULID. Pre-existing hand-seeded identifiers (for example
//! `drg-bootstrap-branch-collisions` or `idea-strata-fortune`) remain valid:
//! nothing in Scarp may require every `id` to be a ULID, and there is no
//! second identity field.
//!
//! # Concurrency boundary
//!
//! Creation is scan-then-write without locking. Two simultaneous Scarp
//! processes — or two Git branches — can allocate the same next display
//! sequence; bootstrap deliberately does not make allocation linearizable
//! and introduces no lock service. What IS guaranteed:
//!
//! - no existing file is ever overwritten: content is staged in a temporary
//!   file and persisted with an atomic no-clobber rename;
//! - a failed creation leaves no partial destination artifact (an abandoned
//!   temporary is a dot-file, which artifact scans ignore), and a failed
//!   sprint creation removes the containment directories it materialized —
//!   only those, in reverse creation order, never a pre-existing directory
//!   or concurrent content;
//! - duplicate display sequences produced by concurrent allocation remain on
//!   disk as distinct files, detectable later by `scarp doctor`.

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::error::Error;
use crate::read::{
    Collection, DECISION, DRAGON, IDEA, LOG, MAINTENANCE, PRINCIPLE, SPRINT, Status, TASK,
};
use crate::repo::{SPRINT_FILE, SPRINTS_DIR};

/// Prefix for generated dragon identities.
pub const DRAGON_ID_PREFIX: &str = "drg_";

/// Prefix for generated idea identities.
pub const IDEA_ID_PREFIX: &str = "ide_";

/// Prefix for generated decision identities.
pub const DECISION_ID_PREFIX: &str = "dec_";

/// Prefix for generated maintenance identities.
pub const MAINTENANCE_ID_PREFIX: &str = "mnt_";

/// Prefix for generated principle identities.
pub const PRINCIPLE_ID_PREFIX: &str = "prn_";

/// Prefix for generated log identities. The two logs written before the
/// collection was managed carry hand-seeded ids
/// (`log-bootstrap-inception`) and stay exactly as they are.
pub const LOG_ID_PREFIX: &str = "log_";

/// Prefix for generated sprint identities.
pub const SPRINT_ID_PREFIX: &str = "spr_";

/// Prefix for generated task identities.
pub const TASK_ID_PREFIX: &str = "tsk_";

/// Largest display sequence representable by four-digit filename prefixes.
pub const MAX_SEQUENCE: u32 = 9999;

/// A successfully created artifact.
#[derive(Debug)]
pub struct NewArtifact {
    /// Canonical singular kind name, e.g. `dragon`.
    pub kind: &'static str,
    /// Stable opaque identity: collection prefix + uppercase ULID.
    pub id: String,
    /// Collection-scoped display sequence.
    pub sequence: u32,
    /// Destination path relative to the repository root.
    pub relative_path: PathBuf,
}

impl NewArtifact {
    /// Human reference for the created artifact, e.g. `dragon:2`.
    pub fn reference(&self) -> String {
        format!("{}:{}", self.kind, self.sequence)
    }

    /// The `new --json` projection of this creation. Field names and
    /// order are a compatibility surface pinned by tests; healthy and
    /// degraded creation share this schema (decision 13).
    pub fn record(&self) -> NewRecord<'_> {
        NewRecord {
            kind: self.kind,
            id: &self.id,
            sequence: self.sequence,
            reference: self.reference(),
            path: self.relative_path.display().to_string(),
        }
    }
}

/// The `scarp new --json` stdout payload: one deterministic object per
/// creation, identical for healthy and degraded creation (decision 13 —
/// the degraded warning rides stderr, never this object).
#[derive(Debug, Serialize)]
pub struct NewRecord<'a> {
    /// Canonical singular kind name, e.g. `dragon`.
    kind: &'a str,
    /// Stable opaque identity.
    id: &'a str,
    /// Collection-scoped display sequence.
    sequence: u32,
    /// Canonical reference, e.g. `dragon:2`.
    reference: String,
    /// Root-relative path with `/` separators.
    path: String,
}

/// Reachability of a just-created flat artifact through the normal
/// strict read path (decision 13).
#[derive(Debug)]
pub enum Reachability {
    /// The strict scan succeeded and the artifact resolves by sequence
    /// and by stable id: created and normally reachable.
    Reachable,
    /// Created successfully, but repository degradation currently blocks
    /// normal access. Holds the deterministic blocking description — the
    /// blocking sibling path with its reason, or the precise blocking
    /// reason when no single path applies.
    Degraded { blocker: String },
}

/// Observational post-write reachability probe for flat creation
/// (decision 13).
///
/// Runs the corresponding normal strict read path after a successful
/// write and proves the new artifact resolves by both sequence and
/// stable id. Purely observational: a failed probe never rolls back the
/// already-successful creation, never mutates anything, and never turns
/// into a nonzero exit — the caller reports the degraded state instead.
/// The strict scan's first failure is by construction the deterministic
/// blocking artifact.
pub fn probe_reachability(
    root: &Path,
    collection: &Collection,
    created: &NewArtifact,
) -> Reachability {
    let artifacts = match crate::read::scan(root, collection) {
        Ok(artifacts) => artifacts,
        Err(err) => {
            return Reachability::Degraded {
                blocker: err.to_string(),
            };
        }
    };
    let display = created.reference();
    let by_sequence = crate::read::resolve(
        &artifacts,
        crate::read::Selector::Sequence(created.sequence),
        &display,
    );
    if let Err(err) = by_sequence {
        return Reachability::Degraded {
            blocker: err.to_string(),
        };
    }
    if let Err(err) = crate::read::resolve(
        &artifacts,
        crate::read::Selector::Id(&created.id),
        &created.id,
    ) {
        return Reachability::Degraded {
            blocker: err.to_string(),
        };
    }
    Reachability::Reachable
}

/// Create a new open dragon in the repository at `root`.
pub fn create_dragon(root: &Path, title: &str, body: Option<&str>) -> Result<NewArtifact, Error> {
    const SECTIONS: &[&str] = &[
        "Context",
        "Question",
        "Constraints",
        "Candidate direction",
        "Resolution criteria",
    ];
    create(root, &DRAGON, DRAGON_ID_PREFIX, SECTIONS, title, body)
}

/// Create a new parked idea in the repository at `root`.
pub fn create_idea(root: &Path, title: &str, body: Option<&str>) -> Result<NewArtifact, Error> {
    create_idea_from(root, title, body, None)
}

/// Create a new parked idea, optionally stamped with the proposal it was
/// realized from.
///
/// `proposal` is the proposal's canonical URL. It is one-way provenance
/// written once at creation: nothing later updates it, and nothing here
/// checks that the proposal still exists. Refusing a duplicate belongs to
/// the caller before it gets here, and to `doctor` afterwards.
pub fn create_idea_from(
    root: &Path,
    title: &str,
    body: Option<&str>,
    proposal: Option<&str>,
) -> Result<NewArtifact, Error> {
    let sections = crate::read::IDEA_SECTIONS;
    let proposal = validate_proposal(proposal)?;
    let extra: Vec<(&str, &str)> = match proposal.as_deref() {
        Some(url) => vec![("proposal", url)],
        None => Vec::new(),
    };
    create_with(root, &IDEA, IDEA_ID_PREFIX, sections, title, body, &extra)
}

/// Create a new pending maintenance item in the repository at `root`.
///
/// No sprint membership: maintenance is precisely the work no sprint
/// commissioned, so nothing here consults or records one.
pub fn create_maintenance(
    root: &Path,
    title: &str,
    body: Option<&str>,
) -> Result<NewArtifact, Error> {
    create_maintenance_from(root, title, body, None)
}

/// Create a new pending maintenance item, optionally stamped with the
/// proposal it was realized from.
///
/// The `proposal` argument carries exactly the semantics
/// [`create_idea_from`] gives it — one-way provenance, written once,
/// never updated, never checked for reachability. Maintenance is a
/// second collection the stamp can land on, not a second meaning for it.
pub fn create_maintenance_from(
    root: &Path,
    title: &str,
    body: Option<&str>,
    proposal: Option<&str>,
) -> Result<NewArtifact, Error> {
    let proposal = validate_proposal(proposal)?;
    let extra: Vec<(&str, &str)> = match proposal.as_deref() {
        Some(url) => vec![("proposal", url)],
        None => Vec::new(),
    };
    create_with(
        root,
        &MAINTENANCE,
        MAINTENANCE_ID_PREFIX,
        crate::read::MAINTENANCE_SECTIONS,
        title,
        body,
        &extra,
    )
}

/// Check an optional `proposal:` stamp before anything is allocated or
/// written, returning it normalized.
///
/// Shared by every collection that can carry the field, so a value one
/// creation path would refuse cannot slip in through another.
fn validate_proposal(proposal: Option<&str>) -> Result<Option<String>, Error> {
    proposal
        .map(|raw| {
            crate::read::validated_proposal(raw)
                .map_err(|message| Error::InvalidInvocation { message })
        })
        .transpose()
}

/// Create a new active principle in the repository at `root`.
///
/// Principles advise; nothing here or in `doctor` ever judges whether an
/// artifact conforms to one.
pub fn create_principle(
    root: &Path,
    title: &str,
    body: Option<&str>,
) -> Result<NewArtifact, Error> {
    create(
        root,
        &PRINCIPLE,
        PRINCIPLE_ID_PREFIX,
        crate::read::PRINCIPLE_SECTIONS,
        title,
        body,
    )
}

/// Create a new log in the repository at `root`.
///
/// Logs have no template, so a supplied body is written verbatim beneath
/// the title and an omitted one leaves nothing but the title. There is no
/// status line to write and no home state to enter: a log records
/// something that already happened.
pub fn create_log(root: &Path, title: &str, body: Option<&str>) -> Result<NewArtifact, Error> {
    create(root, &LOG, LOG_ID_PREFIX, &[], title, body)
}

/// Create a new accepted decision in the repository at `root`.
///
/// Decisions are created directly in their one admitted state: a decision
/// worth recording is already settled, and drafts are not artifacts.
pub fn create_decision(root: &Path, title: &str, body: Option<&str>) -> Result<NewArtifact, Error> {
    const SECTIONS: &[&str] = &["Context", "Decision", "Consequences"];
    create(root, &DECISION, DECISION_ID_PREFIX, SECTIONS, title, body)
}

/// Create a new active sprint in the repository at `root`.
///
/// A sprint artifact is `sprint.md` inside a fresh containment directory
/// `NNNN-slug/`; the directory name carries the display sequence.
/// Concurrent active sprints are legal (decision 15): creation performs
/// the strict sprint scan it needs for sequence allocation, but another
/// active sprint is never a refusal.
///
/// Deliberate duplication of [`create`] (idea 10 discipline): the
/// containment-directory layout diverges from flat files at almost every
/// step — sequence source, destination materialization, template shape —
/// so the shared machinery reduces to slugging, identity, and the safe
/// write.
pub fn create_sprint(root: &Path, title: &str, body: Option<&str>) -> Result<NewArtifact, Error> {
    create_sprint_with(root, title, body, write_new)
}

/// Implementation seam for [`create_sprint`]: `write` performs the final
/// `sprint.md` write, so fault-injection tests can fail it after the
/// containment directory has been materialized. Production callers always
/// pass [`write_new`].
fn create_sprint_with(
    root: &Path,
    title: &str,
    body: Option<&str>,
    write: impl FnOnce(&Path, &str, &str) -> Result<(), Error>,
) -> Result<NewArtifact, Error> {
    const SECTIONS: &[&str] = &["Goal", "Rationale", "Success criteria", "Non-goals"];
    validate_title(SPRINT.kind, title)?;
    let body = body
        .map(|raw| Body::parse(SPRINT.kind, SECTIONS, raw))
        .transpose()?;
    let title = title.trim();
    let slug = slugify(title).ok_or_else(|| Error::InvalidInvocation {
        message: format!(
            "cannot create a sprint titled `{title}`: the title must contain \
             at least one ASCII letter or digit to derive a directory slug"
        ),
    })?;

    let sprints = crate::read::scan_sprints(root)?;
    let max = sprints
        .iter()
        .map(|sprint| sprint.summary.sequence)
        .max()
        .unwrap_or(0);
    if max >= MAX_SEQUENCE {
        return Err(Error::ArtifactConflict {
            path: root.join(SPRINTS_DIR),
            reason: format!(
                "the sprint collection has exhausted the four-digit display \
                 sequence space (last sequence is {MAX_SEQUENCE})"
            ),
        });
    }
    let sequence = max + 1;
    let id = format!("{SPRINT_ID_PREFIX}{}", ulid::Ulid::new());
    let created = jiff::Zoned::now().strftime("%Y-%m-%d").to_string();
    let content = render_artifact(&Template {
        id: &id,
        sequence,
        kind: SPRINT.kind,
        status: Some(Status::Active.name()),
        extra_fields: &[],
        created: &created,
        title,
        sections: SECTIONS,
        body: body.as_ref(),
    });

    let dir_rel = format!("{SPRINTS_DIR}/{sequence:04}-{slug}");
    let mut created_dirs = Vec::new();
    let result = crate::repo::ensure_dir(root, &dir_rel, &mut created_dirs)
        .and_then(|()| write(&root.join(&dir_rel), SPRINT_FILE, &content));
    if let Err(original) = result {
        return Err(rollback_sprint_dirs(root, &created_dirs, original));
    }

    Ok(NewArtifact {
        kind: SPRINT.kind,
        id,
        sequence,
        relative_path: Path::new(&dir_rel).join(SPRINT_FILE),
    })
}

/// Compensate a failed sprint creation (decision 8, returned-error class):
/// remove exactly the directories this invocation created, in reverse
/// creation order, using empty-directory removal only. A pre-existing
/// directory is never in `created`, and `fs::remove_dir` refuses a
/// non-empty directory, so concurrent content is never deleted to make
/// rollback pass. When cleanup succeeds the original error is returned
/// unchanged; when cleanup itself fails — decision 8's doubly degraded
/// case — the error names the original creation failure, the exact path
/// whose cleanup failed, and the debris left for inspection.
fn rollback_sprint_dirs(root: &Path, created: &[PathBuf], original: Error) -> Error {
    for rel in created.iter().rev() {
        let path = root.join(rel);
        if let Err(source) = fs::remove_dir(&path) {
            return Error::Filesystem {
                operation: "rollback of failed sprint creation".into(),
                path: path.clone(),
                source: io::Error::new(
                    source.kind(),
                    format!(
                        "{source}; the original creation failure was: {original}; \
                         this directory, created by the failed invocation, could \
                         not be removed — structural debris may remain and \
                         requires inspection"
                    ),
                ),
            };
        }
    }
    original
}

/// Refuse a title that cannot render into a valid single-heading artifact.
///
/// Checked on the raw supplied title, before trimming, slugging, or any
/// other creation work: every character [`char::is_control`] admits — LF,
/// CR, tab, NUL, DEL, and the remaining Unicode control characters — would
/// split or pollute the rendered `# <title>` heading, which the shared
/// reader then rejects. Nothing is sanitized or discarded; the invocation
/// is refused, and the offending character is reported by escaped spelling
/// and code point, never interpolated raw.
/// Indefinite article for a collection kind.
///
/// `idea` is currently the only managed kind that takes `an`, but deriving
/// it beats a special case that a future vowel-initial collection would
/// silently get wrong.
fn article(kind: &str) -> &'static str {
    match kind.chars().next() {
        Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
        _ => "a",
    }
}

fn validate_title(kind: &str, title: &str) -> Result<(), Error> {
    let article = article(kind);
    if let Some(c) = title.chars().find(|c| c.is_control()) {
        return Err(Error::InvalidInvocation {
            message: format!(
                "cannot create {article} {kind}: the title must be a single line \
                 without control characters, but it contains `{}` (U+{:04X}) — \
                 retry with a plain single-line title",
                c.escape_debug(),
                c as u32
            ),
        });
    }
    Ok(())
}

/// Create a new artifact in its collection's home lifecycle state.
///
/// Allocates `max(existing sequence) + 1` across the collection's
/// directory, derives a deterministic kebab-case slug from `title`,
/// assigns a fresh prefixed ULID identity, and writes the Markdown template
/// through a temporary file with an atomic no-clobber persist. Neither
/// `.scarp.toml` nor any existing artifact is modified.
fn create(
    root: &Path,
    collection: &Collection,
    id_prefix: &str,
    sections: &[&str],
    title: &str,
    body: Option<&str>,
) -> Result<NewArtifact, Error> {
    create_with(root, collection, id_prefix, sections, title, body, &[])
}

/// [`create`] with extra front-matter fields, which land after `status`
/// and before `created`.
#[allow(clippy::too_many_arguments)]
fn create_with(
    root: &Path,
    collection: &Collection,
    id_prefix: &str,
    sections: &[&str],
    title: &str,
    body: Option<&str>,
    extra_fields: &[(&str, &str)],
) -> Result<NewArtifact, Error> {
    let kind = collection.kind;
    validate_title(kind, title)?;
    // Parsed before any sequence is allocated or any path is touched: a
    // rejected body must leave the repository exactly as it was.
    let body = body
        .map(|raw| Body::parse(kind, sections, raw))
        .transpose()?;
    let title = title.trim();
    let slug = slugify(title).ok_or_else(|| Error::InvalidInvocation {
        message: format!(
            "cannot create {} {kind} titled `{title}`: the title must contain \
             at least one ASCII letter or digit to derive a filename slug",
            article(kind)
        ),
    })?;

    let sequence = next_sequence(root, collection)?;
    let id = format!("{id_prefix}{}", ulid::Ulid::new());
    let created = jiff::Zoned::now().strftime("%Y-%m-%d").to_string();
    let content = render_artifact(&Template {
        id: &id,
        sequence,
        kind,
        status: collection.states.first().map(|home| home.name()),
        extra_fields,
        created: &created,
        title,
        sections,
        body: body.as_ref(),
    });

    // Git does not round-trip empty directories, so a cloned repository may
    // lack the destination; materialize it with the same conflict checks
    // `init` uses.
    crate::repo::ensure_dir(root, collection.dir, &mut Vec::new())?;
    let filename = format!("{sequence:04}-{slug}.md");
    write_new(&root.join(collection.dir), &filename, &content)?;

    Ok(NewArtifact {
        kind,
        id,
        sequence,
        relative_path: Path::new(collection.dir).join(filename),
    })
}

/// Create a new pending task at `root`, owned by one active sprint.
///
/// `sprint` is the explicit `--sprint` selection as `(selector, display)`;
/// without it, the owning sprint is inferred only when exactly one sprint
/// is active (decision 15). The new file lands in the chosen sprint's
/// containment directory, stamps that sprint's stable id into the
/// `sprint:` field, and takes the next display sequence globally across
/// every sprint.
pub fn create_task(
    root: &Path,
    title: &str,
    sprint: Option<(crate::read::Selector<'_>, &str)>,
    body: Option<&str>,
) -> Result<NewArtifact, Error> {
    create_task_from(root, title, sprint, body, None)
}

/// Create a new pending task, optionally stamped with the proposal it was
/// realized from.
///
/// The stamp is [`create_idea_from`]'s, unchanged: one-way provenance
/// written once at creation. A task carries it beside — never instead of
/// — the `sprint:` field naming its owner, so a promoted report is an
/// ordinary member of its sprint that happens to record where it came
/// from.
pub fn create_task_from(
    root: &Path,
    title: &str,
    sprint: Option<(crate::read::Selector<'_>, &str)>,
    body: Option<&str>,
    proposal: Option<&str>,
) -> Result<NewArtifact, Error> {
    const SECTIONS: &[&str] = &["Objective", "Acceptance criteria"];
    validate_title(TASK.kind, title)?;
    let proposal = validate_proposal(proposal)?;
    let body = body
        .map(|raw| Body::parse(TASK.kind, SECTIONS, raw))
        .transpose()?;
    let title = title.trim();
    let slug = slugify(title).ok_or_else(|| Error::InvalidInvocation {
        message: format!(
            "cannot create a task titled `{title}`: the title must contain \
             at least one ASCII letter or digit to derive a filename slug"
        ),
    })?;

    let sprints = crate::read::scan_sprints(root)?;
    let chosen = match sprint {
        Some((selector, display)) => {
            // Explicit selection resolves through the established typed
            // contracts (not-found, ambiguity), then requires the sprint
            // to be active — a closed sprint is refused before writing.
            let sprint = crate::read::resolve(&sprints, selector, display)?;
            if sprint.summary.status != Some(Status::Active) {
                return Err(Error::InvalidInvocation {
                    message: format!(
                        "sprint `{}` ({}) is {}; tasks are created only in \
                         an active sprint",
                        sprint.summary.reference(),
                        sprint.summary.title,
                        sprint.summary.state()
                    ),
                });
            }
            sprint
        }
        None => {
            let active: Vec<&crate::read::Artifact> = sprints
                .iter()
                .filter(|sprint| sprint.summary.status == Some(Status::Active))
                .collect();
            match active.as_slice() {
                [] => {
                    return Err(Error::InvalidInvocation {
                        message: "tasks belong to a sprint, and no sprint is \
                                  active; open one with `scarp new sprint \
                                  \"<goal>\"` first"
                            .into(),
                    });
                }
                [only] => *only,
                // Never resolved by scan order (decision 15): the refusal
                // names every active sprint, deterministically ordered by
                // the scan's sequence-then-path sort.
                several => {
                    let candidates: Vec<String> = several
                        .iter()
                        .map(|sprint| {
                            format!("{} ({})", sprint.summary.reference(), sprint.summary.title)
                        })
                        .collect();
                    return Err(Error::InvalidInvocation {
                        message: format!(
                            "{} sprints are active: {}; pass `--sprint \
                             <sprint-ref>` to choose the owning sprint",
                            several.len(),
                            candidates.join(", ")
                        ),
                    });
                }
            }
        }
    };
    let active = chosen;

    let tasks = crate::read::scan_tasks(root)?;
    let max = tasks
        .iter()
        .map(|task| task.summary.sequence)
        .max()
        .unwrap_or(0);
    if max >= MAX_SEQUENCE {
        return Err(Error::ArtifactConflict {
            path: root.join(SPRINTS_DIR),
            reason: format!(
                "the task collection has exhausted the four-digit display \
                 sequence space (last sequence is {MAX_SEQUENCE})"
            ),
        });
    }
    let sequence = max + 1;
    let id = format!("{TASK_ID_PREFIX}{}", ulid::Ulid::new());
    let created = jiff::Zoned::now().strftime("%Y-%m-%d").to_string();
    // `sprint` first, then the optional stamp: extra fields render in the
    // order given, and the owning sprint is the more load-bearing of the
    // two.
    let mut extra_fields: Vec<(&str, &str)> = vec![("sprint", &active.summary.id)];
    if let Some(url) = proposal.as_deref() {
        extra_fields.push(("proposal", url));
    }
    let content = render_artifact(&Template {
        id: &id,
        sequence,
        kind: TASK.kind,
        status: Some(Status::Pending.name()),
        extra_fields: &extra_fields,
        created: &created,
        title,
        sections: SECTIONS,
        body: body.as_ref(),
    });

    let sprint_dir = active
        .summary
        .path
        .rsplit_once('/')
        .expect("sprint paths always contain a directory")
        .0
        .to_string();
    let filename = format!("{sequence:04}-{slug}.md");
    write_new(&root.join(&sprint_dir), &filename, &content)?;

    Ok(NewArtifact {
        kind: TASK.kind,
        id,
        sequence,
        relative_path: Path::new(&sprint_dir).join(filename),
    })
}

/// Derive a deterministic slug from a title.
///
/// Lowercase ASCII alphanumerics, words separated by single hyphens; runs of
/// any other character — including all non-ASCII characters, which are not
/// transliterated — collapse into one separator, and leading/trailing
/// separators are stripped. Returns `None` when nothing sluggable remains.
pub fn slugify(title: &str) -> Option<String> {
    let mut slug = String::new();
    let mut pending_separator = false;
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            if pending_separator {
                slug.push('-');
            }
            slug.push(c.to_ascii_lowercase());
            pending_separator = false;
        } else if !slug.is_empty() {
            pending_separator = true;
        }
    }
    if slug.is_empty() { None } else { Some(slug) }
}

/// Allocate the next display sequence by scanning the collection's
/// directory, refusing to exceed the four-digit space.
fn next_sequence(root: &Path, collection: &Collection) -> Result<u32, Error> {
    let max = max_sequence_in(&root.join(collection.dir), collection.kind)?;
    if max >= MAX_SEQUENCE {
        return Err(Error::ArtifactConflict {
            path: root.join(collection.dir),
            reason: format!(
                "the {} collection has exhausted the four-digit display \
                 sequence space (last sequence is {MAX_SEQUENCE})",
                collection.kind
            ),
        });
    }
    Ok(max + 1)
}

/// Largest display sequence among the artifacts in one managed directory.
///
/// A missing managed directory is an empty collection (Git does not
/// round-trip empty directories); a non-directory object occupying the
/// managed path is a conflict. Every non-hidden entry must be a valid
/// dragon filename; malformed names are a typed error naming the path,
/// never silently skipped. Entries starting with `.` (editor and VCS
/// metadata, abandoned temporaries) are not artifacts and are ignored.
fn max_sequence_in(dir: &Path, kind: &str) -> Result<u32, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(source) if source.kind() == io::ErrorKind::NotFound => return Ok(0),
        Err(source) if source.kind() == io::ErrorKind::NotADirectory => {
            return Err(Error::ArtifactConflict {
                path: dir.to_path_buf(),
                reason: "a non-directory object occupies a managed directory \
                         path; move it aside"
                    .into(),
            });
        }
        Err(source) => {
            return Err(Error::Filesystem {
                operation: "read directory".into(),
                path: dir.to_path_buf(),
                source,
            });
        }
    };

    let mut max = 0u32;
    for entry in entries {
        let entry = entry.map_err(|source| Error::Filesystem {
            operation: "read directory entry".into(),
            path: dir.to_path_buf(),
            source,
        })?;
        let name = entry.file_name();
        let malformed = |reason: String| Error::MalformedArtifact {
            path: dir.join(&name),
            reason,
        };
        let Some(name_str) = name.to_str() else {
            return Err(malformed("filename is not valid UTF-8".into()));
        };
        if name_str.starts_with('.') {
            continue;
        }
        let sequence = parse_sequence(name_str).ok_or_else(|| {
            malformed(format!(
                "{kind} filenames must be `NNNN-slug.md` with a four-digit \
                 display sequence"
            ))
        })?;
        max = max.max(sequence);
    }
    Ok(max)
}

/// Extract the display sequence from a valid artifact filename
/// (`NNNN-slug.md`), or `None` when the name does not conform.
pub(crate) fn parse_sequence(name: &str) -> Option<u32> {
    let digits = name.get(..4)?;
    if !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let slug = name[4..].strip_prefix('-')?.strip_suffix(".md")?;
    if slug.is_empty() {
        return None;
    }
    digits.parse().ok()
}

/// Extract the display sequence from a valid sprint containment directory
/// name (`NNNN-slug`), or `None` when the name does not conform.
pub(crate) fn parse_dir_sequence(name: &str) -> Option<u32> {
    let digits = name.get(..4)?;
    if !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let slug = name[4..].strip_prefix('-')?;
    if slug.is_empty() || slug.ends_with(".md") {
        return None;
    }
    digits.parse().ok()
}

/// One artifact payload to render: front matter plus template sections.
/// `extra_fields` land in the front matter after `status`, before
/// `created`.
struct Template<'a> {
    id: &'a str,
    sequence: u32,
    kind: &'a str,
    /// The home lifecycle state, or `None` for a stateless collection,
    /// whose rendered front matter carries no `status:` line at all.
    status: Option<&'a str>,
    extra_fields: &'a [(&'a str, &'a str)],
    created: &'a str,
    title: &'a str,
    sections: &'a [&'a str],
    /// Caller-supplied section content, or `None` for the empty template.
    body: Option<&'a Body>,
}

/// The largest accepted `--body-file` payload. Narrative artifacts are
/// prose; anything past this is a sign the caller means to write a file
/// rather than propose one, and an unbounded body is an unbounded write
/// driven by whoever authored the input.
const MAX_BODY_BYTES: usize = 64 * 1024;

/// A Markdown fenced-code-block delimiter: three or more backticks or
/// tildes, optionally indented. Returns the marker character and run
/// length so a closing fence can be matched against its opener.
pub(crate) fn fence_marker(line: &str) -> Option<(char, usize)> {
    let trimmed = line.trim_start();
    let marker = trimmed.chars().next().filter(|c| *c == '`' || *c == '~')?;
    let run = trimmed.chars().take_while(|c| *c == marker).count();
    (run >= 3).then_some((marker, run))
}

/// Append one non-heading line to the section being accumulated.
///
/// Content outside any section has nowhere canonical to go, so it is a
/// typed refusal rather than a silently dropped line — which also refuses
/// a body that opens with its own front matter.
/// Advance fenced-code-block state across a line already known to be a
/// fence marker. A closing fence matches the opener's character, is at
/// least as long, and carries no info string.
pub(crate) fn advance_fence(
    fence: &mut Option<(char, usize)>,
    line: &str,
    marker: char,
    run: usize,
) {
    match *fence {
        None => *fence = Some((marker, run)),
        Some((open, len))
            if marker == open && run >= len && line.trim_start()[run..].trim().is_empty() =>
        {
            *fence = None;
        }
        Some(_) => {}
    }
}

fn push_content<'a>(
    current: &mut Option<(String, Vec<&'a str>)>,
    line: &'a str,
    number: usize,
    kind: &str,
    article: &str,
    known: &impl Fn() -> String,
) -> Result<(), Error> {
    match current.as_mut() {
        Some((_, lines)) => lines.push(line),
        None if line.trim().is_empty() => {}
        None => {
            return Err(Error::InvalidInvocation {
                message: format!(
                    "cannot create {article} {kind}: line {number} of the body \
                     has content before any `## ` section heading; every line \
                     must belong to one of {} — this also refuses a body that \
                     opens with its own `---` front matter",
                    known()
                ),
            });
        }
    }
    Ok(())
}

/// Section content supplied at creation time through `--body-file`.
///
/// Scarp owns canonical form: the caller names sections and supplies their
/// prose, and Scarp decides which sections exist, in what order, and how
/// the file is laid out. A body therefore never introduces a section, never
/// reorders one, and never reaches the front matter. Parsing happens before
/// any sequence is allocated or any file is opened, so a rejected body
/// leaves the repository untouched.
enum Body {
    /// `(section name, content)` in input order, each name already checked
    /// against the collection's template.
    Sections(Vec<(String, String)>),
    /// The whole body, verbatim, for a collection with no managed
    /// template. Scarp owns no headings in such an artifact, so it
    /// polices none: there is no section vocabulary to violate and no
    /// template order to disturb.
    Verbatim(String),
}

impl Body {
    /// Parse `raw` as section content for a `kind` artifact whose template
    /// has `sections`.
    ///
    /// The accepted grammar is deliberately the one a human already writes:
    /// `## Section` headings from the collection's own template, with prose
    /// beneath each. Deeper headings are ordinary content.
    ///
    /// An empty `sections` means the collection has no template, and the
    /// body is taken verbatim. The guards every body passes — the size
    /// limit, the control-character refusal, CRLF normalization, and the
    /// refusal of a level-1 heading that would compete with the title —
    /// still apply, because none of them is about sections.
    fn parse(kind: &str, sections: &[&str], raw: &str) -> Result<Self, Error> {
        let invalid = |message: String| Error::InvalidInvocation { message };
        let article = article(kind);
        let known = || {
            sections
                .iter()
                .map(|s| format!("`{s}`"))
                .collect::<Vec<_>>()
                .join(", ")
        };

        if raw.len() > MAX_BODY_BYTES {
            return Err(invalid(format!(
                "cannot create {article} {kind}: the body is {} bytes, over the \
                 {MAX_BODY_BYTES}-byte limit — shorten it, or create the \
                 artifact without a body and edit the file directly",
                raw.len()
            )));
        }

        // Decision 14: the corpus is LF. CRLF input is accepted and
        // normalized here so the written artifact never carries the
        // caller's line endings.
        let raw = raw.replace("\r\n", "\n");

        if let Some(c) = raw
            .chars()
            .find(|&c| c.is_control() && c != '\n' && c != '\t')
        {
            return Err(invalid(format!(
                "cannot create {article} {kind}: the body contains the control \
                 character `{}` (U+{:04X}); bodies are plain UTF-8 text \
                 whose only control characters are tab and newline — strip \
                 it and retry",
                c.escape_debug(),
                c as u32
            )));
        }

        if sections.is_empty() {
            // A level-1 heading is still refused: the title is the
            // command's argument, and a second `# ` would make the
            // written artifact fail title extraction on read-back. Fences
            // are tracked for the same reason they are below — a shell
            // snippet's `# comment` is a comment, and title extraction
            // ignores fenced text too, so refusing it here would reject a
            // body that reads back perfectly.
            let mut fence: Option<(char, usize)> = None;
            for (index, line) in raw.lines().enumerate() {
                if let Some((marker, run)) = fence_marker(line) {
                    advance_fence(&mut fence, line, marker, run);
                    continue;
                }
                if fence.is_some() {
                    continue;
                }
                if let Some(rest) = line.strip_prefix("# ") {
                    return Err(invalid(format!(
                        "cannot create {article} {kind}: line {} of the body is the \
                         level-1 heading `# {}`; the title is the command's \
                         argument and the body carries the prose beneath it",
                        index + 1,
                        rest.trim()
                    )));
                }
            }
            return Ok(Self::Verbatim(raw.trim().to_string()));
        }

        let mut parsed: Vec<(String, String)> = Vec::new();
        let mut current: Option<(String, Vec<&str>)> = None;
        // Open fenced code block, as (marker character, run length). Inside
        // one, nothing is a heading: a shell snippet's `# comment` is a
        // comment, and a sample of Markdown output is a sample.
        let mut fence: Option<(char, usize)> = None;

        for (index, line) in raw.lines().enumerate() {
            let number = index + 1;

            if let Some((marker, run)) = fence_marker(line) {
                advance_fence(&mut fence, line, marker, run);
                push_content(&mut current, line, number, kind, article, &known)?;
                continue;
            }

            if fence.is_some() {
                push_content(&mut current, line, number, kind, article, &known)?;
                continue;
            }

            if let Some(name) = line.strip_prefix("## ") {
                let name = name.trim();
                if !sections.contains(&name) {
                    return Err(invalid(format!(
                        "cannot create {article} {kind}: line {number} of the body \
                         names the section `{name}`, which {article} {kind} \
                         does not have; the sections are {} — Scarp owns the \
                         template, \
                         so a body fills sections but never adds one",
                        known()
                    )));
                }
                if parsed.iter().any(|(seen, _)| seen == name)
                    || current.as_ref().is_some_and(|(seen, _)| seen == name)
                {
                    return Err(invalid(format!(
                        "cannot create {article} {kind}: the body gives the section \
                         `{name}` twice, at line {number} and earlier; \
                         supply each section at most once"
                    )));
                }
                if let Some((seen, lines)) = current.take() {
                    parsed.push((seen, lines.join("\n").trim().to_string()));
                }
                current = Some((name.to_string(), Vec::new()));
                continue;
            }

            if let Some(rest) = line.strip_prefix("# ") {
                return Err(invalid(format!(
                    "cannot create {article} {kind}: line {number} of the body is the \
                     level-1 heading `# {}`; the title is the command's \
                     argument and the body carries only `## ` sections",
                    rest.trim()
                )));
            }

            push_content(&mut current, line, number, kind, article, &known)?;
        }

        if let Some((seen, lines)) = current {
            parsed.push((seen, lines.join("\n").trim().to_string()));
        }

        Ok(Self::Sections(parsed))
    }

    /// Content for `section`, or `None` when the caller did not supply it.
    /// An unsupplied section renders exactly as the empty template does.
    fn content_for(&self, section: &str) -> Option<&str> {
        match self {
            Self::Sections(sections) => sections
                .iter()
                .find(|(name, _)| name == section)
                .map(|(_, content)| content.as_str())
                .filter(|content| !content.is_empty()),
            Self::Verbatim(_) => None,
        }
    }

    /// The whole body for a template-free collection, or `None` when this
    /// body was mapped onto sections.
    fn verbatim(&self) -> Option<&str> {
        match self {
            Self::Verbatim(text) if !text.is_empty() => Some(text),
            _ => None,
        }
    }
}

/// Render an artifact Markdown payload.
fn render_artifact(template: &Template<'_>) -> String {
    let Template {
        id,
        sequence,
        kind,
        status,
        extra_fields,
        created,
        title,
        sections,
        body,
    } = template;
    let mut content = format!(
        "---\n\
         id: {id}\n\
         sequence: {sequence}\n\
         kind: {kind}\n"
    );
    // Omitted entirely for a stateless collection, whose artifacts carry
    // no lifecycle to record; the reader refuses the line rather than
    // ignoring it, so writing an empty one would be corruption.
    if let Some(status) = status {
        content.push_str("status: ");
        content.push_str(status);
        content.push('\n');
    }
    for (key, value) in *extra_fields {
        content.push_str(key);
        content.push_str(": ");
        content.push_str(value);
        content.push('\n');
    }
    content.push_str(&format!(
        "created: {created}\n\
         ---\n\
         \n\
         # {title}\n"
    ));
    // A template-free collection renders its body straight beneath the
    // title, byte-identical to what the author supplied: no headings are
    // introduced, because none are owned.
    if let Some(text) = body.and_then(Body::verbatim) {
        content.push('\n');
        content.push_str(text);
        content.push('\n');
    }
    for section in *sections {
        content.push_str("\n## ");
        content.push_str(section);
        content.push('\n');
        // A filled section is byte-identical to the same section filled by
        // hand: blank line, prose, newline. An unsupplied one is untouched.
        if let Some(text) = body.and_then(|body| body.content_for(section)) {
            content.push('\n');
            content.push_str(text);
            content.push('\n');
        }
    }
    content
}

/// Write `content` to `dir/filename` through an exclusive temporary file in
/// `dir` and an atomic no-clobber persist. A failure at any point leaves no
/// destination file; an existing destination is never replaced.
fn write_new(dir: &Path, filename: &str, content: &str) -> Result<(), Error> {
    let destination = dir.join(filename);
    let mut tmp = tempfile::Builder::new()
        .prefix(".scarp.artifact.tmp")
        .tempfile_in(dir)
        .map_err(|source| Error::Filesystem {
            operation: "create temporary artifact".into(),
            path: dir.to_path_buf(),
            source,
        })?;
    tmp.write_all(content.as_bytes())
        .map_err(|source| Error::Filesystem {
            operation: "write temporary artifact".into(),
            path: tmp.path().to_path_buf(),
            source,
        })?;
    tmp.persist_noclobber(&destination).map_err(|err| {
        if err.error.kind() == io::ErrorKind::AlreadyExists {
            Error::ArtifactConflict {
                path: destination.clone(),
                reason: "an artifact already occupies the destination path".into(),
            }
        } else {
            Error::Filesystem {
                operation: "persist artifact".into(),
                path: destination.clone(),
                source: err.error,
            }
        }
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo;
    use crate::repo::{DECISIONS_DIR, DRAGONS_DIR, IDEAS_DIR};

    fn temp_repo() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().expect("create temporary directory");
        repo::init(tmp.path()).expect("initialize repository");
        tmp
    }

    fn dragons_dir_entries(root: &Path) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(root.join(DRAGONS_DIR))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn slugify_is_deterministic_kebab_case() {
        let cases = [
            ("Branch sequence collisions", "branch-sequence-collisions"),
            ("Hello,   World!!", "hello-world"),
            ("--Already--Separated--", "already-separated"),
            ("MiXeD CaSe 123", "mixed-case-123"),
            ("v2.0 rollout", "v2-0-rollout"),
        ];
        for (title, expected) in cases {
            assert_eq!(slugify(title).as_deref(), Some(expected), "for {title:?}");
        }
    }

    #[test]
    fn slugify_treats_non_ascii_as_separators_without_transliterating() {
        assert_eq!(slugify("Café risk").as_deref(), Some("caf-risk"));
        assert_eq!(slugify("naïve merge").as_deref(), Some("na-ve-merge"));
        assert_eq!(slugify("日本語"), None);
    }

    #[test]
    fn slugify_rejects_titles_with_nothing_sluggable() {
        for title in ["", "   ", "!!!", "---", "…"] {
            assert_eq!(slugify(title), None, "for {title:?}");
        }
    }

    #[test]
    fn create_writes_zero_padded_filename_with_front_matter_and_headings() {
        let tmp = temp_repo();

        let dragon = create_dragon(tmp.path(), "Branch sequence collisions", None).unwrap();

        assert_eq!(dragon.sequence, 1);
        assert_eq!(dragon.reference(), "dragon:1");
        assert_eq!(
            dragon.relative_path,
            Path::new(DRAGONS_DIR).join("0001-branch-sequence-collisions.md")
        );
        let content = fs::read_to_string(tmp.path().join(&dragon.relative_path)).unwrap();
        assert!(content.starts_with("---\n"), "{content}");
        for line in [
            format!("id: {}", dragon.id),
            "sequence: 1".into(),
            "kind: dragon".into(),
            "status: open".into(),
        ] {
            assert!(
                content.contains(&format!("\n{line}\n")),
                "missing `{line}` in:\n{content}"
            );
        }
        let mut cursor = 0;
        for heading in [
            "# Branch sequence collisions",
            "## Context",
            "## Question",
            "## Constraints",
            "## Candidate direction",
            "## Resolution criteria",
        ] {
            let at = content[cursor..]
                .find(heading)
                .unwrap_or_else(|| panic!("missing or out-of-order `{heading}` in:\n{content}"));
            cursor += at + heading.len();
        }
    }

    /// Real proposals contain code. A shell snippet's `# comment` is a
    /// comment and a sample of Markdown output is a sample — neither is a
    /// heading. Found against a real GitHub proposal body during task 54.
    #[test]
    fn headings_inside_fenced_code_blocks_are_content() {
        let tmp = temp_repo();
        let body = "## Problem\n\n```sh\n# install it\nscarp init\n```\n\n\
                    ## Sketch\n\n```\n## Consequences\n# Title\n```\n";

        let idea = create_idea(tmp.path(), "Fenced", Some(body)).unwrap();

        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        assert!(content.contains("# install it"), "{content}");
        assert!(content.contains("\n## Consequences\n"), "{content}");
        // The forged heading stayed inside Sketch rather than becoming a
        // section: Boundaries and Evidence are still empty and in order.
        assert!(
            content.ends_with("## Boundaries\n\n## Evidence\n"),
            "{content}"
        );
    }

    /// A fence only closes on its own marker, at least as long, with no
    /// info string — so a nested-looking inner fence does not end the block.
    #[test]
    fn fence_closes_only_on_a_matching_marker() {
        let tmp = temp_repo();
        let body = "## Problem\n\n~~~~\n```\n## Not a section\n```\n~~~~\n";

        let idea = create_idea(tmp.path(), "Nested fences", Some(body)).unwrap();

        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        assert!(content.contains("## Not a section"), "{content}");
        assert!(
            content.ends_with("## Sketch\n\n## Boundaries\n\n## Evidence\n"),
            "{content}"
        );
    }

    /// An unterminated fence swallows the rest of the body rather than
    /// resuming heading detection halfway through a code block.
    #[test]
    fn an_unclosed_fence_keeps_the_remainder_as_content() {
        let tmp = temp_repo();
        let body = "## Problem\n\n```\n## Sketch\nstill code\n";

        let idea = create_idea(tmp.path(), "Unclosed", Some(body)).unwrap();

        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        assert!(content.contains("still code"), "{content}");
        assert!(
            content.ends_with("## Sketch\n\n## Boundaries\n\n## Evidence\n"),
            "{content}"
        );
    }

    /// The proposal stamp is a managed front-matter field: written by
    /// Scarp, parsed back by the read model, and checked by doctor. That
    /// is what distinguishes it from a link in prose, which nothing
    /// validates.
    #[test]
    fn a_realized_idea_carries_its_proposal_url_in_front_matter() {
        let tmp = temp_repo();
        let url = "https://github.com/henry-filgueiras/scarp/issues/2";

        let idea = create_idea_from(tmp.path(), "Realized", None, Some(url)).unwrap();

        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        assert!(
            content.contains(&format!("\nproposal: {url}\n")),
            "{content}"
        );
        // Ordered after status and before created, like every extra field.
        let status = content.find("status: parked").unwrap();
        let proposal = content.find("proposal:").unwrap();
        let created = content.find("created:").unwrap();
        assert!(status < proposal && proposal < created, "{content}");

        let parsed = crate::read::scan(tmp.path(), &crate::read::IDEA).unwrap();
        assert_eq!(parsed[0].summary.proposal.as_deref(), Some(url));
    }

    /// An idea authored locally carries no stamp at all, rather than an
    /// empty one: absence is the normal case and must stay invisible.
    #[test]
    fn an_unstamped_idea_has_no_proposal_field() {
        let tmp = temp_repo();

        let idea = create_idea(tmp.path(), "Local", None).unwrap();

        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        assert!(!content.contains("proposal:"), "{content}");
        let parsed = crate::read::scan(tmp.path(), &crate::read::IDEA).unwrap();
        assert!(parsed[0].summary.proposal.is_none());
    }

    /// A value nobody can follow is worse than no provenance, because it
    /// looks like provenance. Refused before any write.
    #[test]
    fn an_unusable_proposal_value_is_refused_before_writing() {
        let tmp = temp_repo();

        for bad in [
            "",
            "   ",
            "henry-filgueiras/scarp#2",
            "https://example.com/a b",
        ] {
            let err = create_idea_from(tmp.path(), "Bad stamp", None, Some(bad)).unwrap_err();
            assert!(
                matches!(err, Error::InvalidInvocation { .. }),
                "expected a typed refusal for {bad:?}, got {err:?}"
            );
        }

        let ok = create_idea(tmp.path(), "First real idea", None).unwrap();
        assert_eq!(
            ok.sequence, 1,
            "a refused stamp consumed a display sequence"
        );
    }

    /// The whole point of `--body-file`: a body-filled artifact must be
    /// indistinguishable from the same artifact filled in by hand, or the
    /// channel introduces a second author of canonical form.
    #[test]
    fn body_filled_artifact_is_byte_identical_to_the_hand_filled_template() {
        let tmp = temp_repo();
        let body = "## Problem\n\nProse decays on a phone.\n\n## Evidence\n\nSprint 10.\n";

        let idea = create_idea(tmp.path(), "Remote proposals", Some(body)).unwrap();

        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        let expected = format!(
            "---\nid: {}\nsequence: 1\nkind: idea\nstatus: parked\ncreated: {}\n---\n\n\
             # Remote proposals\n\n## Problem\n\nProse decays on a phone.\n\n\
             ## Sketch\n\n## Boundaries\n\n## Evidence\n\nSprint 10.\n",
            idea.id,
            jiff::Zoned::now().strftime("%Y-%m-%d"),
        );
        assert_eq!(content, expected);
    }

    /// Section order is the template's, never the caller's.
    #[test]
    fn body_sections_render_in_template_order_not_input_order() {
        let tmp = temp_repo();
        let body = "## Evidence\n\nLast in input.\n\n## Problem\n\nFirst in template.\n";

        let idea = create_idea(tmp.path(), "Ordering", Some(body)).unwrap();

        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        let problem = content.find("## Problem").unwrap();
        let evidence = content.find("## Evidence").unwrap();
        assert!(
            problem < evidence,
            "template order not preserved:\n{content}"
        );
    }

    /// A body may fill some sections; the rest stay exactly as the empty
    /// template renders them.
    #[test]
    fn unsupplied_and_empty_sections_render_as_the_bare_template() {
        let tmp = temp_repo();
        let body = "## Problem\n\nOnly this one.\n\n## Sketch\n";

        let idea = create_idea(tmp.path(), "Partial", Some(body)).unwrap();

        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        assert!(
            content.ends_with("## Sketch\n\n## Boundaries\n\n## Evidence\n"),
            "{content}"
        );
    }

    /// Decision 14: the corpus is LF whatever the caller sent.
    #[test]
    fn crlf_body_is_normalized_to_lf() {
        let tmp = temp_repo();
        let body = "## Problem\r\n\r\nCarriage returns in.\r\n";

        let idea = create_idea(tmp.path(), "Line endings", Some(body)).unwrap();

        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        assert!(
            !content.contains('\r'),
            "CR survived into the artifact:\n{content}"
        );
        assert!(
            content.contains("\n## Problem\n\nCarriage returns in.\n"),
            "{content}"
        );
    }

    /// Every rejection must happen before anything is written. The strongest
    /// evidence is that the collection is still empty and the next creation
    /// still gets sequence 1 — a burned sequence would be a silent leak.
    #[test]
    fn a_rejected_body_writes_nothing_and_burns_no_sequence() {
        let tmp = temp_repo();
        let rejected = [
            "## Nonexistent\n\nUnknown section.\n",
            "Content before any heading.\n",
            "# Forged title\n",
            "## Problem\n\nOne.\n\n## Problem\n\nTwice.\n",
            "## Problem\n\nNul\u{0}byte.\n",
            "---\nid: forged\n---\n\n## Problem\n\nFront matter.\n",
        ];

        for body in rejected {
            let err = create_idea(tmp.path(), "Refused", Some(body)).unwrap_err();
            assert!(
                matches!(err, Error::InvalidInvocation { .. }),
                "expected a typed invalid invocation for {body:?}, got {err:?}"
            );
        }

        assert!(
            fs::read_dir(tmp.path().join(IDEAS_DIR))
                .map(|mut entries| entries.next().is_none())
                .unwrap_or(true),
            "a refused body left a file behind"
        );
        let ok = create_idea(tmp.path(), "First real idea", None).unwrap();
        assert_eq!(ok.sequence, 1, "a refused body consumed a display sequence");
    }

    /// The unknown-section refusal has to name the sections that exist, or
    /// the caller cannot act on it (decision 4).
    #[test]
    fn a_template_free_body_is_written_verbatim_beneath_the_title() {
        // Logs own no headings, so none are policed: the author's `## `
        // and `### ` headings survive exactly as written, in order.
        let tmp = temp_repo();
        let body = "Opening prose.\n\n## An author heading\n\nMore.\n\n### Deeper\n";
        let created = create_log(tmp.path(), "A log", Some(body)).unwrap();
        let written = fs::read_to_string(tmp.path().join(&created.relative_path)).unwrap();

        assert!(
            written.ends_with(
                "# A log\n\nOpening prose.\n\n## An author heading\n\nMore.\n\n### Deeper\n"
            ),
            "verbatim body must follow the title untouched:\n{written}"
        );
        assert!(
            !written.contains("status:"),
            "a stateless artifact must carry no status line:\n{written}"
        );
        assert!(written.contains("id: log_"), "{written}");
    }

    #[test]
    fn a_template_free_artifact_without_a_body_is_just_its_title() {
        let tmp = temp_repo();
        let created = create_log(tmp.path(), "A log", None).unwrap();
        let written = fs::read_to_string(tmp.path().join(&created.relative_path)).unwrap();

        assert!(written.ends_with("---\n\n# A log\n"), "{written}");
    }

    #[test]
    fn a_verbatim_body_refuses_a_competing_level_one_heading() {
        let tmp = temp_repo();
        let err = create_log(tmp.path(), "A log", Some("ok\n\n# Another title\n")).unwrap_err();
        let message = err.to_string();

        assert!(
            message.contains("level-1 heading") && message.contains("Another title"),
            "{message}"
        );
        // Refused before any write: a rejected body leaves no artifact.
        assert!(!tmp.path().join(crate::repo::LOGS_DIR).exists());
    }

    #[test]
    fn a_fenced_hash_comment_is_not_a_competing_title() {
        // The refusal above must not fire on a shell snippet, because
        // title extraction ignores fenced text too — refusing here would
        // reject a body that reads back perfectly.
        let tmp = temp_repo();
        let body = "Setup:\n\n```sh\n# install it\ncargo install scarp\n```\n";
        let created = create_log(tmp.path(), "A log", Some(body)).unwrap();
        let written = fs::read_to_string(tmp.path().join(&created.relative_path)).unwrap();

        assert!(written.contains("# install it"), "{written}");
        // The written artifact must survive the reader that will parse it.
        let logs = crate::read::scan(tmp.path(), &LOG).unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].summary.title, "A log");
    }

    #[test]
    fn unknown_section_refusal_names_the_offender_and_the_template() {
        let tmp = temp_repo();

        let err = create_idea(tmp.path(), "Bad", Some("## Consequences\n\nx\n")).unwrap_err();

        let message = err.to_string();
        for expected in [
            "Consequences",
            "`Problem`",
            "`Sketch`",
            "`Boundaries`",
            "`Evidence`",
        ] {
            assert!(
                message.contains(expected),
                "missing {expected} in: {message}"
            );
        }
    }

    /// An oversized body is refused rather than written: the input is
    /// authored by whoever filed the proposal.
    #[test]
    fn oversized_body_is_refused() {
        let tmp = temp_repo();
        let body = format!("## Problem\n\n{}\n", "x".repeat(MAX_BODY_BYTES));

        let err = create_idea(tmp.path(), "Too big", Some(&body)).unwrap_err();

        assert!(err.to_string().contains("over the"), "{err}");
    }

    /// Deeper headings are ordinary prose — the corpus uses `###` inside
    /// sections and a body must be able to as well.
    #[test]
    fn deeper_headings_are_content_not_sections() {
        let tmp = temp_repo();
        let body = "## Problem\n\n### A subsection\n\nStill inside Problem.\n";

        let idea = create_idea(tmp.path(), "Subsections", Some(body)).unwrap();

        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        assert!(content.contains("### A subsection"), "{content}");
    }

    /// The body surface is not idea-specific; it is a property of `new`.
    #[test]
    fn every_narrative_collection_accepts_a_body() {
        let tmp = temp_repo();

        let dragon = create_dragon(tmp.path(), "Risk", Some("## Context\n\nD.\n")).unwrap();
        let decision = create_decision(tmp.path(), "Choice", Some("## Context\n\nX.\n")).unwrap();
        let sprint = create_sprint(tmp.path(), "Cycle", Some("## Goal\n\nS.\n")).unwrap();
        let task = create_task(tmp.path(), "Item", None, Some("## Objective\n\nT.\n")).unwrap();

        for (created, needle) in [
            (&dragon, "\n## Context\n\nD.\n"),
            (&decision, "\n## Context\n\nX.\n"),
            (&sprint, "\n## Goal\n\nS.\n"),
            (&task, "\n## Objective\n\nT.\n"),
        ] {
            let content = fs::read_to_string(tmp.path().join(&created.relative_path)).unwrap();
            assert!(
                content.contains(needle),
                "missing {needle:?} in:\n{content}"
            );
        }
    }

    #[test]
    fn create_idea_writes_a_parked_artifact_with_idea_template() {
        let tmp = temp_repo();

        let idea = create_idea(tmp.path(), "Declarative specs", None).unwrap();

        assert_eq!(idea.sequence, 1);
        assert_eq!(idea.reference(), "idea:1");
        assert_eq!(
            idea.relative_path,
            Path::new(IDEAS_DIR).join("0001-declarative-specs.md")
        );
        assert!(idea.id.starts_with(IDEA_ID_PREFIX), "{}", idea.id);
        let content = fs::read_to_string(tmp.path().join(&idea.relative_path)).unwrap();
        for line in ["kind: idea", "status: parked", "sequence: 1"] {
            assert!(
                content.contains(&format!("\n{line}\n")),
                "missing `{line}` in:\n{content}"
            );
        }
        let mut cursor = 0;
        for heading in [
            "# Declarative specs",
            "## Problem",
            "## Sketch",
            "## Boundaries",
            "## Evidence",
        ] {
            let at = content[cursor..]
                .find(heading)
                .unwrap_or_else(|| panic!("missing or out-of-order `{heading}` in:\n{content}"));
            cursor += at + heading.len();
        }
    }

    #[test]
    fn create_decision_writes_an_accepted_artifact_with_decision_template() {
        let tmp = temp_repo();

        let decision = create_decision(tmp.path(), "Adopt the spec engine", None).unwrap();

        assert_eq!(decision.sequence, 1);
        assert_eq!(decision.reference(), "decision:1");
        assert!(
            decision.id.starts_with(DECISION_ID_PREFIX),
            "{}",
            decision.id
        );
        assert_eq!(
            decision.relative_path,
            Path::new(DECISIONS_DIR).join("0001-adopt-the-spec-engine.md")
        );
        let content = fs::read_to_string(tmp.path().join(&decision.relative_path)).unwrap();
        for line in ["kind: decision", "status: accepted", "sequence: 1"] {
            assert!(
                content.contains(&format!("\n{line}\n")),
                "missing `{line}` in:\n{content}"
            );
        }
        let mut cursor = 0;
        for heading in [
            "# Adopt the spec engine",
            "## Context",
            "## Decision",
            "## Consequences",
        ] {
            let at = content[cursor..]
                .find(heading)
                .unwrap_or_else(|| panic!("missing or out-of-order `{heading}` in:\n{content}"));
            cursor += at + heading.len();
        }
    }

    #[test]
    fn decision_sequences_continue_after_the_existing_corpus() {
        let tmp = temp_repo();
        fs::create_dir_all(tmp.path().join(DECISIONS_DIR)).unwrap();
        fs::write(
            tmp.path().join(DECISIONS_DIR).join("0015-last.md"),
            "seeded",
        )
        .unwrap();

        let decision = create_decision(tmp.path(), "Sixteenth", None).unwrap();

        assert_eq!(decision.sequence, 16, "must continue after the maximum");
    }

    #[test]
    fn idea_sequences_are_collection_scoped_and_ignore_dragons() {
        let tmp = temp_repo();
        // A dragon with a high sequence must not influence idea allocation.
        fs::write(
            tmp.path().join(DRAGONS_DIR).join("0009-dragon.md"),
            "seeded",
        )
        .unwrap();
        fs::create_dir_all(tmp.path().join(IDEAS_DIR)).unwrap();
        fs::write(
            tmp.path().join(IDEAS_DIR).join("0004-rejected.md"),
            "seeded",
        )
        .unwrap();

        let idea = create_idea(tmp.path(), "Next idea", None).unwrap();

        assert_eq!(idea.sequence, 5, "must continue after the terminal maximum");
    }

    #[test]
    fn create_idea_materializes_the_ideas_directory_on_first_use() {
        let tmp = temp_repo();
        assert!(
            !tmp.path().join(IDEAS_DIR).exists(),
            "init must not pre-create the ideas directory"
        );

        let idea = create_idea(tmp.path(), "First idea", None).unwrap();

        assert!(tmp.path().join(&idea.relative_path).is_file());
    }

    #[test]
    fn create_stamps_an_iso_date() {
        let tmp = temp_repo();

        let dragon = create_dragon(tmp.path(), "Dated risk", None).unwrap();

        let content = fs::read_to_string(tmp.path().join(&dragon.relative_path)).unwrap();
        let created = content
            .lines()
            .find_map(|l| l.strip_prefix("created: "))
            .expect("front matter must include `created`");
        let bytes = created.as_bytes();
        assert_eq!(bytes.len(), 10, "created must be YYYY-MM-DD: {created}");
        assert!(
            bytes.iter().enumerate().all(|(i, b)| match i {
                4 | 7 => *b == b'-',
                _ => b.is_ascii_digit(),
            }),
            "created must be YYYY-MM-DD: {created}"
        );
    }

    #[test]
    fn generated_ids_are_prefixed_uppercase_ulids_and_unique() {
        let tmp = temp_repo();

        let first = create_dragon(tmp.path(), "First risk", None).unwrap();
        let second = create_dragon(tmp.path(), "Second risk", None).unwrap();

        assert_ne!(first.id, second.id);
        for id in [&first.id, &second.id] {
            let ulid = id
                .strip_prefix(DRAGON_ID_PREFIX)
                .unwrap_or_else(|| panic!("id must start with `{DRAGON_ID_PREFIX}`: {id}"));
            assert_eq!(ulid.len(), 26, "{id}");
            assert!(
                ulid.bytes().all(
                    |b| b.is_ascii_digit() || (b.is_ascii_uppercase() && !b"ILOU".contains(&b))
                ),
                "id must be an uppercase Crockford base32 ULID: {id}"
            );
        }
    }

    #[test]
    fn sequence_allocation_spans_every_lifecycle_state() {
        let tmp = temp_repo();
        fs::write(tmp.path().join(DRAGONS_DIR).join("0001-old.md"), "seeded").unwrap();
        fs::write(
            tmp.path().join(DRAGONS_DIR).join("0005-resolved.md"),
            "seeded",
        )
        .unwrap();

        let dragon = create_dragon(tmp.path(), "Next risk", None).unwrap();

        assert_eq!(dragon.sequence, 6, "must continue after the maximum");
        assert!(
            tmp.path()
                .join(DRAGONS_DIR)
                .join("0006-next-risk.md")
                .is_file()
        );
    }

    #[test]
    fn seeded_non_ulid_ids_are_left_untouched() {
        let tmp = temp_repo();
        let seeded_path = tmp.path().join(DRAGONS_DIR).join("0001-seeded.md");
        let seeded = "---\nid: drg-bootstrap-branch-collisions\nsequence: 1\n---\n";
        fs::write(&seeded_path, seeded).unwrap();

        let dragon = create_dragon(tmp.path(), "Fresh risk", None).unwrap();

        assert_eq!(dragon.sequence, 2);
        assert_eq!(
            fs::read_to_string(&seeded_path).unwrap(),
            seeded,
            "existing artifacts must remain byte-identical"
        );
    }

    #[test]
    fn malformed_filenames_are_reported_not_skipped() {
        let tmp = temp_repo();
        for bad in [
            "notes.txt",
            "12-short.md",
            "0002.md",
            "0002-.md",
            "abcd-x.md",
        ] {
            let path = tmp.path().join(DRAGONS_DIR).join(bad);
            fs::write(&path, "junk").unwrap();

            let err = create_dragon(tmp.path(), "Any title", None).unwrap_err();

            match err {
                Error::MalformedArtifact { path: reported, .. } => {
                    assert_eq!(reported, path, "error must name the offending file")
                }
                other => panic!("expected malformed artifact for {bad:?}, got {other:?}"),
            }
            fs::remove_file(&path).unwrap();
        }
    }

    #[test]
    fn dot_entries_are_not_artifacts_and_are_ignored() {
        let tmp = temp_repo();
        fs::write(tmp.path().join(DRAGONS_DIR).join(".gitkeep"), "").unwrap();

        let dragon = create_dragon(tmp.path(), "Ignores dotfiles", None).unwrap();

        assert_eq!(dragon.sequence, 1);
    }

    #[test]
    fn create_materializes_missing_managed_directories() {
        // Simulate `git clone` of a freshly initialized repository: Git
        // preserves the marker but drops every empty directory.
        let tmp = temp_repo();
        fs::remove_dir_all(tmp.path().join("archaeology")).unwrap();

        let dragon = create_dragon(tmp.path(), "Post-clone risk", None).unwrap();

        assert_eq!(dragon.sequence, 1);
        assert!(tmp.path().join(&dragon.relative_path).is_file());
    }

    #[test]
    fn non_directory_at_managed_path_is_a_conflict() {
        let tmp = temp_repo();
        fs::remove_dir(tmp.path().join(DRAGONS_DIR)).unwrap();
        fs::write(tmp.path().join(DRAGONS_DIR), "not a directory").unwrap();

        let err = create_dragon(tmp.path(), "Any title", None).unwrap_err();

        assert!(matches!(err, Error::ArtifactConflict { .. }), "{err:?}");
    }

    #[test]
    fn sequence_exhaustion_is_a_typed_error_not_a_five_digit_filename() {
        let tmp = temp_repo();
        fs::write(tmp.path().join(DRAGONS_DIR).join("9999-last.md"), "seeded").unwrap();

        let err = create_dragon(tmp.path(), "One too many", None).unwrap_err();

        assert!(matches!(err, Error::ArtifactConflict { .. }), "{err:?}");
        assert_eq!(
            dragons_dir_entries(tmp.path()),
            vec!["9999-last.md".to_string()],
            "no artifact may be created on exhaustion"
        );
    }

    #[test]
    fn destination_conflict_refuses_overwrite_and_leaves_no_temporary() {
        let tmp = temp_repo();
        let dir = tmp.path().join(DRAGONS_DIR);
        fs::write(dir.join("0001-taken.md"), "original").unwrap();

        // Drive the write layer directly: it is the guard that holds even
        // when a file appears between the sequence scan and the persist.
        let err = write_new(&dir, "0001-taken.md", "new content").unwrap_err();

        assert!(matches!(err, Error::ArtifactConflict { .. }), "{err:?}");
        assert_eq!(
            fs::read_to_string(dir.join("0001-taken.md")).unwrap(),
            "original",
            "the existing artifact must survive byte-identical"
        );
        assert_eq!(
            dragons_dir_entries(tmp.path()),
            vec!["0001-taken.md".to_string()],
            "a failed persist must not litter temporaries"
        );
    }

    #[cfg(unix)]
    #[test]
    fn induced_write_failure_leaves_no_partial_artifact() {
        use std::os::unix::fs::PermissionsExt;

        let tmp = temp_repo();
        let dir = tmp.path().join(DRAGONS_DIR);
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o555)).unwrap();

        let result = create_dragon(tmp.path(), "Cannot be written", None);

        fs::set_permissions(&dir, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(
            matches!(result, Err(Error::Filesystem { .. })),
            "{result:?}"
        );
        assert_eq!(
            dragons_dir_entries(tmp.path()),
            Vec::<String>::new(),
            "a failed creation must leave nothing behind"
        );
    }

    #[test]
    fn control_character_titles_are_refused_before_trim_without_writing() {
        let tmp = temp_repo();
        for (title, code) in [
            ("Evil title\n# Second heading", "U+000A"),
            ("carriage\rreturn", "U+000D"),
            ("tab\there", "U+0009"),
            ("nul\0byte", "U+0000"),
            ("del\u{7f}char", "U+007F"),
            // Leading and trailing controls must be refused, not hidden by
            // the later trim.
            ("\tleading control", "U+0009"),
            ("trailing control\n", "U+000A"),
        ] {
            let err = create_dragon(tmp.path(), title, None).unwrap_err();
            match &err {
                Error::InvalidInvocation { message } => {
                    assert!(message.contains(code), "must name {code}: {message}");
                    assert!(
                        message.contains("single line") && message.contains("control characters"),
                        "must name the constraint: {message}"
                    );
                    assert!(
                        !message.chars().any(char::is_control),
                        "must not embed a raw control character: {message:?}"
                    );
                }
                other => panic!("expected invalid invocation for {title:?}, got {other:?}"),
            }
            assert_eq!(
                dragons_dir_entries(tmp.path()),
                Vec::<String>::new(),
                "a refused title must write nothing, for {title:?}"
            );
        }
    }

    #[test]
    fn every_creator_validates_the_title_before_other_work() {
        let tmp = temp_repo();
        let bad = "bad\ntitle";

        for err in [
            create_dragon(tmp.path(), bad, None).unwrap_err(),
            create_idea(tmp.path(), bad, None).unwrap_err(),
            // No sprint is active, yet the refusal is about the title:
            // validation precedes the active-sprint scan.
            create_task(tmp.path(), bad, None, None).unwrap_err(),
        ] {
            assert!(matches!(err, Error::InvalidInvocation { .. }), "{err:?}");
            assert!(err.to_string().contains("U+000A"), "{err}");
        }

        // A sprint is active, yet the refusal is about the title: sprint
        // title validation precedes every scan.
        create_sprint(tmp.path(), "Occupied", None).unwrap();
        let err = create_sprint(tmp.path(), bad, None).unwrap_err();
        assert!(matches!(err, Error::InvalidInvocation { .. }), "{err:?}");
        assert!(
            err.to_string().contains("U+000A") && !err.to_string().contains("active"),
            "{err}"
        );
    }

    #[test]
    fn marker_significant_punctuation_remains_legal_title_content() {
        let tmp = temp_repo();

        let dragon = create_dragon(tmp.path(), "Handle ]] and # and | and ] safely", None).unwrap();

        let content = fs::read_to_string(tmp.path().join(&dragon.relative_path)).unwrap();
        assert!(
            content.contains("# Handle ]] and # and | and ] safely"),
            "{content}"
        );
    }

    #[test]
    fn failed_sprint_write_rolls_back_created_directories_and_returns_the_original_error() {
        let tmp = temp_repo();

        let err = create_sprint_with(tmp.path(), "Doomed", None, |_, _, _| {
            Err(Error::Filesystem {
                operation: "write temporary artifact".into(),
                path: PathBuf::from("injected"),
                source: io::Error::other("injected fault"),
            })
        })
        .unwrap_err();

        assert!(matches!(err, Error::Filesystem { .. }), "{err:?}");
        let message = err.to_string();
        assert!(
            message.contains("injected fault") && !message.contains("rollback"),
            "the original error must come back unchanged: {message}"
        );
        assert!(
            !tmp.path().join(SPRINTS_DIR).exists(),
            "every directory the invocation created must be removed"
        );
        assert!(
            tmp.path().join("archaeology").is_dir(),
            "pre-existing ancestors must survive"
        );
    }

    #[test]
    fn failed_sprint_write_rolls_back_every_ancestor_it_created() {
        // Simulate `git clone` of an empty repository: the whole layout is
        // materialized by this one invocation, so all of it rolls back.
        let tmp = temp_repo();
        fs::remove_dir_all(tmp.path().join("archaeology")).unwrap();

        let err = create_sprint_with(tmp.path(), "Doomed", None, |_, _, _| {
            Err(Error::Filesystem {
                operation: "write temporary artifact".into(),
                path: PathBuf::from("injected"),
                source: io::Error::other("injected fault"),
            })
        })
        .unwrap_err();

        assert!(matches!(err, Error::Filesystem { .. }), "{err:?}");
        assert!(
            !tmp.path().join("archaeology").exists(),
            "the invocation created the whole chain, so the whole chain rolls back"
        );
    }

    #[test]
    fn rollback_preserves_preexisting_directories_and_the_retry_reuses_the_sequence() {
        let tmp = temp_repo();
        let history = tmp.path().join(SPRINTS_DIR).join("0004-history");
        fs::create_dir_all(&history).unwrap();
        fs::write(
            history.join(SPRINT_FILE),
            "---\nid: spr-history\nsequence: 4\nkind: sprint\nstatus: closed\ncreated: 2026-07-20\n---\n\n# History\n",
        )
        .unwrap();

        let err = create_sprint_with(tmp.path(), "Doomed", None, |_, _, _| {
            Err(Error::Filesystem {
                operation: "write temporary artifact".into(),
                path: PathBuf::from("injected"),
                source: io::Error::other("injected fault"),
            })
        })
        .unwrap_err();

        assert!(matches!(err, Error::Filesystem { .. }), "{err:?}");
        assert!(
            !tmp.path().join(SPRINTS_DIR).join("0005-doomed").exists(),
            "the fresh containment directory must be removed"
        );
        assert!(
            tmp.path().join(SPRINTS_DIR).is_dir() && history.is_dir(),
            "pre-existing directories are never removed"
        );

        let sprint = create_sprint(tmp.path(), "Recovered", None).unwrap();
        assert_eq!(sprint.sequence, 5, "the sequence stays available for retry");
        assert!(
            tmp.path()
                .join(SPRINTS_DIR)
                .join("0005-recovered")
                .join(SPRINT_FILE)
                .is_file()
        );
    }

    #[test]
    fn obstructed_rollback_is_a_filesystem_failure_naming_original_and_leftover() {
        let tmp = temp_repo();

        let err = create_sprint_with(tmp.path(), "Doomed", None, |dir, _, _| {
            // Concurrent content appears in the fresh directory before the
            // write fails; rollback must not delete it.
            fs::write(dir.join("concurrent.md"), "someone else's work").unwrap();
            Err(Error::Filesystem {
                operation: "write temporary artifact".into(),
                path: dir.to_path_buf(),
                source: io::Error::other("injected fault"),
            })
        })
        .unwrap_err();

        let leftover = tmp.path().join(SPRINTS_DIR).join("0001-doomed");
        match &err {
            Error::Filesystem { path, .. } => {
                assert_eq!(path, &leftover, "must name the path whose cleanup failed")
            }
            other => panic!("expected filesystem failure, got {other:?}"),
        }
        let message = err.to_string();
        assert!(
            message.contains("injected fault"),
            "must name the original creation failure: {message}"
        );
        assert!(
            message.contains("debris") && message.contains("inspection"),
            "must warn that debris remains: {message}"
        );
        assert_eq!(
            fs::read_to_string(leftover.join("concurrent.md")).unwrap(),
            "someone else's work",
            "concurrent content is never deleted to make rollback pass"
        );
    }

    #[test]
    fn invalid_title_creates_nothing() {
        let tmp = temp_repo();

        let err = create_dragon(tmp.path(), "!!!", None).unwrap_err();

        assert!(matches!(err, Error::InvalidInvocation { .. }), "{err:?}");
        assert_eq!(dragons_dir_entries(tmp.path()), Vec::<String>::new());
    }

    #[test]
    fn create_sprint_writes_template_in_a_fresh_containment_directory() {
        let tmp = temp_repo();

        let sprint = create_sprint(tmp.path(), "Placement and sprints", None).unwrap();

        assert_eq!(sprint.sequence, 1);
        assert_eq!(sprint.reference(), "sprint:1");
        assert!(sprint.id.starts_with(SPRINT_ID_PREFIX), "{}", sprint.id);
        assert_eq!(
            sprint.relative_path,
            Path::new(SPRINTS_DIR).join("0001-placement-and-sprints/sprint.md")
        );
        let content = fs::read_to_string(tmp.path().join(&sprint.relative_path)).unwrap();
        for needle in [
            "kind: sprint",
            "status: active",
            "# Placement and sprints",
            "## Goal",
            "## Rationale",
            "## Success criteria",
            "## Non-goals",
        ] {
            assert!(content.contains(needle), "missing `{needle}`:\n{content}");
        }
    }

    #[test]
    fn concurrent_active_sprints_are_created_normally() {
        // Decision 15 supersedes the former
        // `a_second_active_sprint_is_refused_naming_the_first` regression:
        // another active sprint is never a refusal.
        let tmp = temp_repo();
        create_sprint(tmp.path(), "First", None).unwrap();

        let second = create_sprint(tmp.path(), "Second", None).unwrap();

        assert_eq!(second.sequence, 2);
        assert!(
            tmp.path()
                .join(SPRINTS_DIR)
                .join("0002-second")
                .join(SPRINT_FILE)
                .is_file()
        );
    }

    #[test]
    fn sprint_sequences_continue_after_closed_sprints() {
        let tmp = temp_repo();
        let dir = tmp.path().join(SPRINTS_DIR).join("0004-history");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join(SPRINT_FILE),
            "---\nid: spr-history\nsequence: 4\nkind: sprint\nstatus: closed\ncreated: 2026-07-20\n---\n\n# History\n",
        )
        .unwrap();

        let sprint = create_sprint(tmp.path(), "Next", None).unwrap();

        assert_eq!(sprint.sequence, 5);
    }

    #[test]
    fn parse_dir_sequence_accepts_only_containment_directory_names() {
        assert_eq!(parse_dir_sequence("0001-bootstrap"), Some(1));
        assert_eq!(parse_dir_sequence("9999-x"), Some(9999));
        for bad in ["0001-x.md", "001-x", "0001-", "abcd-x", "0001"] {
            assert_eq!(parse_dir_sequence(bad), None, "for {bad:?}");
        }
    }

    #[test]
    fn parse_sequence_accepts_only_valid_dragon_filenames() {
        assert_eq!(parse_sequence("0001-x.md"), Some(1));
        assert_eq!(parse_sequence("9999-some-long-slug.md"), Some(9999));
        for bad in [
            "001-x.md",
            "00001-x.md",
            "0001x.md",
            "0001-.md",
            "0001-x.txt",
            "0001-x.md.bak",
        ] {
            assert_eq!(parse_sequence(bad), None, "for {bad:?}");
        }
    }
}
