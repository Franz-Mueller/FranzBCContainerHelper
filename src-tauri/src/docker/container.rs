use crate::docker::{
    artifact::{build_bc_artifact_url, download_artifact},
    image::build_image,
};
use crate::utils::file_handling::get_data_dir;
use crate::AppState;
use bollard::query_parameters::ListContainersOptionsBuilder;
use bollard::Docker;
use std::error::Error;
use tauri::State;

#[tauri::command]
pub async fn create_container(
    state: State<'_, AppState>,
    deployment_type: String,
    version: String,
    country: String,
) -> Result<(), String> {
    let app_base_paths = state.application_base_paths.lock().unwrap();

    Ok(())
}

#[cfg(test)]
mod test_container_creation {
    use super::*;

    #[test]
    #[ignore = "expensive, windows only, file creation, file copying, big downloads, archive extractions, ..."]
    fn e2e_create_container() {}
}
