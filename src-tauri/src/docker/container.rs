use crate::docker::{
    artifact::{build_bc_artifact_url, download_artifact},
    image::build_image,
};

use bollard::query_parameters::ListContainersOptionsBuilder;
use bollard::Docker;

#[tauri::command]
pub async fn create_container() {
    let artifact_url = build_bc_artifact_url("sandbox");
}
