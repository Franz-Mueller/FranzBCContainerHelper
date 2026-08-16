use crate::bc::artifact::{BcArtifact, BcArtifactRequest};
use crate::bc::version::BcVersion;
use crate::docker::image::BcImage;
use crate::AppState;
use std::str::FromStr;
use tauri::State;

#[tauri::command]
pub async fn create_container(
    // TODO move into new command module
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
    let image: BcImage = state.image_builder.build(&artifact).await.unwrap();
    // let container: BcContainer = state.container_builder(&image).await.unwrap();
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
