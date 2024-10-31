use serde::{de::DeserializeOwned, Deserialize, Serialize};

use std::{
    fmt::Display, fs::{self, File}, io::{self, BufReader, BufWriter}, ops::{Deref, DerefMut}, path::{Path, PathBuf}
};

#[derive(Debug, Deserialize, Serialize)]
pub struct DiskVec<T> {
    pub(crate) path: PathBuf,
    pub(crate) inner: Vec<T>,
}

impl<T> DiskVec<T>
where
    T: DeserializeOwned + Serialize,
{
    /// Open (or create) the `DiskMap` stored in `path`.
    ///
    /// # Errors
    ///
    /// Returns any error from [`File::open`] or
    /// [`serde_json::from_reader`].
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut memos = Self {
            path: PathBuf::from(&path.as_ref()),
            ..Default::default()
        };
        if fs::exists(&path)? {
            let file = File::open(path)?;
            memos.inner = serde_json::from_reader(BufReader::new(file))?;
        }
        Ok(memos)
    }

    /// Save the `DiskMap` contents to a file at `path`.
    ///
    /// # Errors
    ///
    /// Returns any errors from [`File::create`] or
    /// [`serde_json::to_writer`].
    pub fn sync(&self) -> io::Result<()> {
        let file = File::create(&self.path)?;
        serde_json::to_writer(BufWriter::new(file), &self.inner)?;
        Ok(())
    }
}

impl<T> Default for DiskVec<T> {
    fn default() -> Self {
        Self {
            path: PathBuf::from("memos.txt"),
            inner: Vec::new(),
        }
    }
}

impl<T> Deref for DiskVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for DiskVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Display for DiskVec<T>
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for elem in self.iter() {
            writeln!(f, "{elem}")?;
        }
        Ok(())
    }
}