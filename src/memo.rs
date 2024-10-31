use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::DiskVec;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Memo {
    pub text: String,
    pub status: Status,
}

impl Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.text)
    }
}

pub type Memos = DiskVec<Memo>;

impl Memos {
    pub fn add(&mut self, text: &str) {
        self.inner.push(Memo {
            text: text.to_owned(),
            status: Status::Pending,
        });
    }

    pub fn find_all(&mut self, text: &str) -> Vec<&mut Memo> {
        self.inner
            .iter_mut()
            .filter(|m| m.text.contains(text))
            .collect()
    }
    
    pub fn purge_done(&mut self) {
        self.inner.retain(|m| m.status != Status::Done);
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("no match")]
    NoMatch,
    #[error("not unique")]
    NotUnique(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
    Pending,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pending => "-",
                Self::Done => "x",
            }
        )
    }
}
