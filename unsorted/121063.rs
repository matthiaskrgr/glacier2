use std::{
    fmt, ops,
    path::{Component, Path, PathBuf},
};

pub struct AbsPathBuf(PathBuf);

impl TryFrom<PathBuf> for AbsPathBuf {
    type Error = PathBuf;
    fn try_from(path: impl AsRef<Path>) -> Result<AbsPathBuf, PathBuf> {}
}

impl TryFrom<&str> for AbsPathBuf {
    fn try_from(path: &str) -> Result<AbsPathBuf, PathBuf> {
        AbsPathBuf::try_from(PathBuf::from(path))
    }
}
