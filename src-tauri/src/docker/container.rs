use bollard::query_parameters::ListContainersOptionsBuilder;
use bollard::Docker;

#[tauri::command]
pub async fn get_version() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let version = docker.version().await.unwrap();
    println!("{:?}", version);
}

#[tauri::command]
pub async fn list_containers() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let params = ListContainersOptionsBuilder::new().all(true).build();
    let containers = docker.list_containers(Some(params)).await.unwrap();
    dbg!(containers);
}
