

use super::error::GitError;
use gix::bstr::{BStr, BString, ByteSlice};
use gix::diff::blob::pipeline::Mode;
use gix::diff::blob::{DiffLineStats, Pipeline};
use gix::diff::Rewrites;
use gix::open::permissions::Config;
use gix::worktree::Stack;
use std::io::Write;
use colored::*;
use std::fmt;
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct CommitDiff {
    pub old_commit: String,
    pub new_commit: String,
    pub changes: Vec<FileChange>,
}

#[derive(Debug)]
pub struct FileChange {
    pub path: String,
    pub change_type: ChangeType,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub source_path: Option<String>,
    pub diff_lines: Vec<DiffLine>,
}

#[derive(Debug)]
pub enum ChangeType {
    Added,
    Deleted,
    Modified,
    Renamed,
    Copied,
}

#[derive(Debug)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub content: String,
    pub syntax_highlighted: Option<String>,
}

#[derive(Debug)]
pub enum DiffLineType {
    Added,
    Removed,
    Context,
}

pub fn get_commit_diff(
    repo: &gix::Repository,
    old_commit: &str,
    new_commit: &str,
) -> Result<CommitDiff, GitError> {
    let old_tree_id = repo
        .rev_parse_single(old_commit.as_bytes())?
        .object()?
        .peel_to_tree()?;
    let new_tree_id = repo
        .rev_parse_single(new_commit.as_bytes())?
        .object()?
        .peel_to_tree()?;

    let mut changes = Vec::new();
    let mut diff_cache = repo.diff_resource_cache_for_tree_diff()?;
  

    old_tree_id
        .changes()?
        .options(|opts| {
            opts.track_path().track_rewrites(Some(Rewrites::default()));
        })
        .for_each_to_obtain_tree(&new_tree_id, |change| {
            match change {
                gix::object::tree::diff::Change::Addition { entry_mode, location, id, .. } => {
                    if entry_mode.is_blob_or_symlink() {
                        if let Ok(obj) = id.object() {
                            let content = String::from_utf8_lossy(&obj.data).to_string();
                            let diff_lines = content
                                .lines()
                                .map(|line| DiffLine {
                                    line_type: DiffLineType::Added,
                                    content: line.to_string(),
                                    syntax_highlighted: None,
                                })
                                .collect();
                            changes.push(FileChange {
                                path: location.to_string(),
                                change_type: ChangeType::Added,
                                lines_added: count_lines(&obj.data),
                                lines_removed: 0,
                                source_path: None,
                                diff_lines,
                            });
                        }
                    }
                }
                gix::object::tree::diff::Change::Deletion { entry_mode, location, id, .. } => {
                    if entry_mode.is_blob_or_symlink() {
                        if let Ok(obj) = id.object() {
                            let content = String::from_utf8_lossy(&obj.data).to_string();
                            let diff_lines = content
                                .lines()
                                .map(|line| DiffLine {
                                    line_type: DiffLineType::Removed,
                                    content: line.to_string(),
                                    syntax_highlighted: None,
                                })
                                .collect();
                            changes.push(FileChange {
                                path: location.to_string(),
                                change_type: ChangeType::Deleted,
                                lines_added: 0,
                                lines_removed: count_lines(&obj.data),
                                source_path: None,
                                diff_lines,
                            });
                        }
                    }
                }
                gix::object::tree::diff::Change::Modification { entry_mode, location, id, previous_id, .. } => {
                    if entry_mode.is_blob() {
                        if let Ok(cache) = change.diff(&mut diff_cache).map(|p| p.resource_cache) {
                            if let Ok(prep) = cache.prepare_diff() {
                                let counts = gix::diff::blob::diff(
                                    gix::diff::blob::Algorithm::Myers,
                                    &prep.interned_input(),
                                    gix::diff::blob::sink::Counter::default(),
                                );

                                // Get the raw content for both versions
                                if let (Ok(old_obj), Ok(new_obj)) = (previous_id.object(), id.object()) {
                                    let old_content = String::from_utf8_lossy(&old_obj.data);
                                    let new_content = String::from_utf8_lossy(&new_obj.data);
                                    
                                    let mut diff_lines: Vec<DiffLine> = Vec::new();
                                    
                                    // Add removed lines
                                    for line in old_content.lines() {
                                        let syntax_highlighted = match syntax_highlight_content(line, &location) {
                                            Ok(highlighted) => Some(highlighted),
                                            Err(_) => None,
                                        };
                                        diff_lines.push(DiffLine {
                                            line_type: DiffLineType::Removed,
                                            content: line.to_string(),
                                            syntax_highlighted,
                                        });
                                    }
                                    
                                    // Add added lines with syntax highlighting
                                    for line in new_content.lines() {
                                        let syntax_highlighted = match syntax_highlight_content(line, &location) {
                                            Ok(highlighted) => Some(highlighted),
                                            Err(_) => None,
                                        };
                                        diff_lines.push(DiffLine {
                                            line_type: DiffLineType::Added,
                                            content: line.to_string(),
                                            syntax_highlighted,
                                        });
                                    }

                                    changes.push(FileChange {
                                        path: location.to_string(),
                                        change_type: ChangeType::Modified,
                                        lines_added: counts.insertions as usize,
                                        lines_removed: counts.removals as usize,
                                        source_path: None,
                                        diff_lines,
                                    });
                                }
                            }
                        }
                    }
                }
                gix::object::tree::diff::Change::Rewrite { location, source_location, copy, diff, .. } => {
                    let diff_lines = Vec::new(); // TODO: Implement proper diff lines for rewrites
                    changes.push(FileChange {
                        path: location.to_string(),
                        change_type: if copy { ChangeType::Copied } else { ChangeType::Renamed },
                        lines_added: diff.map(|d| d.insertions as usize).unwrap_or(0),
                        lines_removed: diff.map(|d| d.removals as usize).unwrap_or(0),
                        source_path: Some(source_location.to_string()),
                        diff_lines,
                    });
                }
            }
            Ok::<_, std::convert::Infallible>(Default::default())
        })?;

    Ok(CommitDiff {
        old_commit: old_commit.to_string(),
        new_commit: new_commit.to_string(),
        changes,
    })
}

fn count_lines(data: &[u8]) -> usize {
    data.iter().filter(|&&b| b == b'\n').count()
}





use gix::{Repository, Reference};
use std::collections::{HashSet, VecDeque};
use gix::prelude::ObjectIdExt;
use gix::hash::ObjectId;


pub trait Walker {
    /// Visit each commit in the walk
    fn visit<F>(&mut self, f: F) -> Result<(), GitError>
    where
        F: FnMut(&ObjectId) -> Result<(), GitError>;

    /// Get pairs of adjacent commits in the walk
    fn adjacent_pairs<F>(&mut self, mut f: F) -> Result<(), GitError>
    where
        F: FnMut(&ObjectId, &ObjectId) -> Result<(), GitError>
    {
        let mut previous: Option<ObjectId> = None;
        self.visit(|current| {
            if let Some(prev) = previous.as_ref() {
                f(prev, current)?;
            }
            previous = Some(current.clone());
            Ok(())
        })
    }
}



pub struct CommitTreeIterator<'a> {
    repo: &'a Repository,
    queue: VecDeque<ObjectId>,
    visited: HashSet<ObjectId>,
}

impl<'a> CommitTreeIterator<'a> {
    pub fn new(repo: &'a Repository) -> Result<Self, GitError> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        
        // Get all branches
        let refs = repo.references()?;
        
        let ref_iter = refs.all()?;

        for reference in ref_iter.into_iter() {
            let reference = reference?;
                if let Ok(commit_id) = reference.id().detach().try_into() {
                    queue.push_back(commit_id);
                }
            
        }

      

        Ok(CommitTreeIterator {
            repo,
            queue,
            visited,
        })
    }
}

impl<'a> Iterator for CommitTreeIterator<'a> {
    type Item = Result<ObjectId, GitError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(commit_id) = self.queue.pop_front() {
            // Skip if already visited
            if !self.visited.insert(commit_id.clone()) {
                continue;
            }

            // Get commit's parents and add them to queue
            match self.repo.find_commit(commit_id.clone()) {
                Ok(commit) => {
                    // Add parents to queue
                    for parent_id in commit.parent_ids() {
                        match parent_id.object() {
                            Ok(obj) => match obj.try_into_commit() {
                                Ok(parent) => {
                                    self.queue.push_back(parent.id().detach());
                                }
                                Err(e) => return Some(Err(e.into())),
                            },
                            Err(e) => return Some(Err(e.into())),
                        }
                    }
                    return Some(Ok(commit_id));
                }
                Err(e) => return Some(Err(e.into())),
            }
        }
        None
    }
}


// Implement for CommitTreeIterator
impl<'a> Walker for CommitTreeIterator<'a> {
    fn visit<F>(&mut self, mut f: F) -> Result<(), GitError>
    where
        F: FnMut(&ObjectId) -> Result<(), GitError>
    {
        for commit_result in self {
            let commit_id = commit_result?;
            f(&commit_id)?;
        }
        Ok(())
    }
}

impl fmt::Display for CommitDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Commit diff {} -> {}", self.old_commit.bright_yellow(), self.new_commit.bright_yellow())?;
        
        for change in &self.changes {
            write!(f, "{}", change)?;
        }
        Ok(())
    }
}

impl fmt::Display for FileChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Header with file path and stats
        let change_symbol = match self.change_type {
            ChangeType::Added => "+".green(),
            ChangeType::Deleted => "-".red(),
            ChangeType::Modified => "M".yellow(),
            ChangeType::Renamed => "R".blue(),
            ChangeType::Copied => "C".cyan(),
        };

        write!(f, "{} {} ", change_symbol, self.path.bright_white())?;
        
        // Show original path for renamed/copied files
        if let Some(source) = &self.source_path {
            write!(f, "(from: {}) ", source.bright_black())?;
        }
        
        writeln!(f, "({} +{}, -{})", 
            "changes:".bright_black(),
            self.lines_added.to_string().green(),
            self.lines_removed.to_string().red()
        )?;

        // Print diff lines
        for line in &self.diff_lines {
            writeln!(f, "{}", line)?;
        }
        
        writeln!(f) // Extra newline between files
    }
}

impl fmt::Display for DiffLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.line_type {
            DiffLineType::Added => "+".green(),
            DiffLineType::Removed => "-".red(),
            DiffLineType::Context => " ".normal(),
        };
        
        // Use syntax highlighted content if available, otherwise use plain content
        let content = if let Some(highlighted) = &self.syntax_highlighted {
            highlighted.to_string()
        } else {
            match self.line_type {
                DiffLineType::Added => self.content.green(),
                DiffLineType::Removed => self.content.red(),
                DiffLineType::Context => self.content.normal(),
            }.to_string()
        };
        
        write!(f, "{} {}", prefix, content)
    }
}

fn syntax_highlight_content(content: &str, file_path: &BStr) -> Result<String, GitError> {
    lazy_static! {
        static ref PS: SyntaxSet = SyntaxSet::load_defaults_newlines();
        static ref TS: ThemeSet = ThemeSet::load_defaults();
    }
    
    let file_path = file_path.to_str()?;
    let extension = std::path::Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
        
    let syntax = PS.find_syntax_by_extension(extension)
        .unwrap_or_else(|| PS.find_syntax_plain_text());
    
    let mut h = HighlightLines::new(syntax, &TS.themes["base16-ocean.dark"]);
    
    let ranges = h.highlight_line(content, &PS)
        .map_err(|_| GitError::StdError("Failed to highlight line".into()))?;
    let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
    
    Ok(escaped)
}
