use std::path::{Path, PathBuf};
use url::Url;

struct Repo {
    url: Url,
    path: PathBuf,
}

impl Repo {
    pub async fn clone(url: Url, repos_folder: PathBuf) -> Result<Self, RepoError> {
        Ok(Self {
            url: url,
            path: repos_folder,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RepoError {}
