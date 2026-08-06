use std::error::Error;

fn build_bc_artifact_url(
    deployment_type: &str,
    country: &str,
    version: &str,
) -> Result<String, Box<dyn Error>> {
    let available_deployment_types = ["sandbox", "onprem"];
    if !available_deployment_types.contains(&deployment_type) {
        return Err("cannot build artifact url, deployment type not valid".into());
    }
    let storage_account = "bcartifacts-exdbf9fwegejdqak.b02.azurefd.net".to_string();
    let version = "115.4.41023.43755"; // TODO Version query
    Ok(format!(
        "https://{storage_account}/{deployment_type}/{version}/{country}"
    ))
}

fn get_best_bc_artifact_version(version: &str) -> Result<String, Box<dyn Error>> {
    Ok("Test".to_string())
}
