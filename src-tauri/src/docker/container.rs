use crate::bc::artifact::{BcArtifact, BcArtifactRequest};
use crate::AppState;
use crate::{bc::version::BcVersion, docker::image::build_image};
use std::str::FromStr;
use tauri::State;

#[tauri::command]
pub async fn create_container(
    state: State<'_, AppState>,
    deployment_type: String,
    version: String,
    country: String,
) -> Result<(), String> {
    let artifact: BcArtifact = state
        .artifact_resolver
        .resolve(BcArtifactRequest {
            deployment_type,
            version: BcVersion::from_str(&version).unwrap(),
            country,
        })
        .await
        .unwrap();
    Ok(())
    // get artifact
    // build image with artifact
    // start container
}

#[cfg(test)]
mod test_container_creation {
    use super::*;

    #[test]
    #[ignore = "expensive, windows only, file creation, file copying, big downloads, archive extractions, ..."]
    fn e2e_create_container() {}
}
