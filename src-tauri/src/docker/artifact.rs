use crate::utils::bc_version::BcVersion;
use reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::ZipArchive;

// TODO Refactoring
// TODO Error Handling
// TODO Testing

// region download artifact
/// Returns the path to the requestet artifact.
/// If the path allready exists in cache the download is skipped.
/// The path should be in format: ./artifacts/{country}/{version}
pub async fn download_artifact<'a>(
    url: &'a str,
    path_name: &'a str,
) -> Result<&'a Path, Box<dyn Error>> {
    // IDEA Check path before downloading in case the desired version is in cache but cannot be found in the index
    // TODO move into container creation function. could be solved otherwise, but later the container function should manage cache interaction anyways
    // path_name = {
    //     let url_parts: Vec<&str> = url.split("/").clone().collect();
    //     let url_parts_len = url_parts.len();
    //     let country = url_parts.get(url_parts_len - 1).unwrap();
    //     let version = url_parts.get(url_parts_len - 2).unwrap();
    //     &format!("./artifacts/{country}/{version}")
    // };
    let path = Path::new(path_name);
    if path.try_exists()? {
        return Ok(path);
    }

    let zip_path_name = format!("{path_name}.zip");
    let zip_path = Path::new(&zip_path_name);
    if zip_path.try_exists()? {
        unzip(&zip_path, &path)?;
        return Ok(path);
    }

    let response = reqwest::get(url).await?;
    let mut file = match File::create(&zip_path) {
        Err(e) => panic!("could not create file: {e}"),
        Ok(file) => file,
    };
    let content = response.bytes().await?;
    file.write_all(&content)?;

    unzip(&zip_path, &path)?;
    Ok(path)
}

fn unzip(zip_path: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file).unwrap();
    archive.extract(path)?;
    fs::remove_file(zip_path)?;
    Ok(())
}

#[cfg(test)]
mod test_artifact_download {
    use super::*;

    #[test]
    fn my_test() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let result = download_artifact("https://bcartifacts-exdbf9fwegejdqak.b02.azurefd.net/sandbox/15.4.41023.43755/de", "./artifacts/de/15.4.41023.43755").await; // expected version to find = 15.4.41023.43755
                match result {
                    Ok(_) => println!("success"),
                    Err(e) => panic!("unexpected error: {e}"),
                };
            })
    }
}
// endregion download artifact

// region get artifact url
/// Returns a artifact url.
/// If the provided version isn't available, it will look for the closest version within the major.
/// This means another minor, build or revision could be choosen.
pub async fn build_bc_artifact_url(
    deployment_type: &str,
    version: &str,
    country: &str,
) -> Result<String, Box<dyn Error>> {
    if !["sandbox", "onprem"].contains(&deployment_type) {
        return Err("cannot build artifact url, deployment type not valid".into());
    }
    let version = BcVersion::from_str(version);

    let storage_account = "bcartifacts-exdbf9fwegejdqak.b02.azurefd.net".to_string();

    let base_url = format!("https://{storage_account}/{deployment_type}");
    let country_index_url = format!("{base_url}/indexes/{country}.json");

    let version = get_best_bc_artifact_version(&country_index_url, version).await?;

    Ok(format!("{base_url}/{version}/{country}"))
}

async fn get_best_bc_artifact_version(
    country_index_url: &str,
    searched_version: BcVersion,
) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(country_index_url).await?.text().await?;
    // Data is expected to arrive in this format:
    // [{"Version":"15.4.41023.43755","CreationTime":"2020-06-26T00:13:59Z"},
    // {"Version":"16.0.11240.31204","CreationTime":"2021-10-11T08:49:00Z"}]
    let version_data: Vec<HashMap<String, String>> = serde_json::from_str(&body)?; // IDEA Cache?

    let available_versions: Vec<BcVersion> = extract_available_versions(version_data);
    let closest_available_version: BcVersion =
        search_closest_available_version(&searched_version, available_versions);

    Ok(closest_available_version.get_version_string())
}

fn extract_available_versions(version_data: Vec<HashMap<String, String>>) -> Vec<BcVersion> {
    // TODO solve with map
    let mut available_versions: Vec<BcVersion> = Vec::new();
    for d in version_data.iter() {
        available_versions.push(BcVersion::from_str(d.get(&"Version".to_string()).unwrap()));
    }
    available_versions
}

// IDEA Provide Setting on how strict to be with version selection
fn search_closest_available_version(
    searched_version: &BcVersion,
    mut available_versions: Vec<BcVersion>,
) -> BcVersion {
    if available_versions.contains(searched_version) {
        return *searched_version;
    }

    available_versions.retain(|v| v.major == searched_version.major);

    if available_versions.is_empty() {
        panic!("major not available");
    }

    if available_versions
        .iter()
        .any(|v| v.minor == searched_version.minor)
    {
        available_versions.retain(|v| v.minor == searched_version.minor);
        if available_versions
            .iter()
            .any(|v| v.build == searched_version.build)
        {
            available_versions.retain(|v| v.build == searched_version.build);
        }
    }

    available_versions
        .iter()
        .filter(|v| *v > searched_version)
        .max()
        .copied()
        .unwrap()
}

#[cfg(test)]
mod test_artifact_url_building {
    use super::*;

    #[test]
    fn my_test() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let result = build_bc_artifact_url("sandbox", "15.4.34.24553", "de").await; // expected version to find = 15.4.41023.43755
                match result {
                    Ok(s) => println!("{s}"),
                    Err(e) => panic!("unexpected error: {e}"),
                };
            })
    }
}
// endregion get artifact url
