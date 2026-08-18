use crate::bc::artifact::{BcArtifact, BcArtifactRequest};
use crate::bc::version::BcVersion;
use crate::docker::image::BcImage;
use crate::AppState;
use tauri::State;

use std::str::FromStr;

#[tauri::command]
pub async fn create_docker_container(
    state: State<'_, AppState>,
    deployment_type: String,
    version: String,
    country: String,
    container_name: String,
) -> Result<(), String> {
    let version = BcVersion::from_str(&version).map_err(|err| err.to_string())?;
    let artifact: BcArtifact = state
        .artifact_resolver
        .resolve(BcArtifactRequest {
            deployment_type,
            version: version,
            country,
        })
        .await
        .map_err(|err| err.to_string())?;
    let image: BcImage = state
        .image_builder
        .build(&artifact)
        .await
        .map_err(|err| err.to_string())?;
    state
        .container_builder
        .build(&image, &container_name)
        .await
        .map_err(|err| err.to_string())?;
    Ok(())
}
