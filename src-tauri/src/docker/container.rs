use bollard::Docker;

#[tauri::command]
pub async fn my_custom_command() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let version = docker.version().await.unwrap();
    println!("{:?}", version);
}
