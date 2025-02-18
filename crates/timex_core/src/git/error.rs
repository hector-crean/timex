use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use gix::{self};
use colored::*;

// Each git commit has a unique hash


#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error(transparent)]
    Decode(#[from] gix::objs::decode::Error),
    #[error(transparent)]
    Object(#[from] gix::object::commit::Error),
    #[error(transparent)]
    OpenRepo(#[from] gix::open::Error),
    #[error(transparent)]
    HeadCommit(#[from] gix::reference::head_commit::Error),
    #[error(transparent)]
    Walk(#[from] gix::revision::walk::Error),
    #[error(transparent)]
    WalkIter(#[from] gix::revision::walk::iter::Error),
    #[error(transparent)]
    FindObject(#[from] gix::object::find::existing::Error),
    #[error(transparent)]
    ObjectConversion(#[from] gix::object::conversion::Error),
    #[error(transparent)]
    DiffError(#[from] gix::diff::tree::Error),
    #[error(transparent)]
    RevParse(#[from] gix::revision::spec::parse::single::Error),
    #[error(transparent)]
    PeelToKind(#[from] gix::object::peel::to_kind::Error),
    #[error(transparent)]
    DiffCache(#[from] gix::repository::diff_resource_cache::Error),
    #[error(transparent)]
    HashDecode(#[from] gix::hash::decode::Error),
    #[error(transparent)]
    DiffOptionsInit(#[from] gix::diff::options::init::Error),
    #[error(transparent)]
    DiffForEach(#[from] gix::object::tree::diff::for_each::Error),
    #[error(transparent)]
    TryInto(#[from] gix::object::try_into::Error),
    #[error(transparent)]
    Reference(#[from] gix::reference::find::existing::Error),
    #[error(transparent)]
    FindWithConversion(#[from] gix::object::find::existing::with_conversion::Error),
    #[error(transparent)]
    ReferenceIter(#[from] gix::reference::iter::Error),
    #[error(transparent)]
    ReferenceIterInit(#[from] gix::reference::iter::init::Error),
    #[error(transparent)]
    StdError(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    Utf8Error(#[from] gix::bstr::Utf8Error),
}
