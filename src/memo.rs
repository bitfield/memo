use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::DiskVec;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn add_fn_adds_memo_to_memos_with_pending_status() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("test.json");
        let mut memos = Memos::open(&path).unwrap();
        assert!(memos.is_empty(), "unexpected data");
        memos.add("buy milk");
        assert_eq!(memos.len(), 1, "wrong number of memos");
        let memo = memos.first().expect("no memos");
        assert_eq!(
            memo,
            &Memo {
                text: String::from("buy milk"),
                status: Status::Pending
            }
        );
    }
}
