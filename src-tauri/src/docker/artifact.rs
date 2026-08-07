use reqwest;
use std::collections::HashMap;
use std::error::Error;

// TODO Refactoring
// TODO Error Handling
// TODO Testing

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct Version {
    major: u32,
    minor: u32,
    build: u32,
    revision: u32,
}

impl Version {
    fn new(version: &str) -> Version {
        let v: Vec<&str> = version.split(".").collect();

        Version {
            major: v[0].parse().unwrap(),
            minor: v[1].parse().unwrap(),
            build: v[2].parse().unwrap(),
            revision: v[3].parse().unwrap(),
        }
    }

    fn get_version_string(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.revision
        )
    }
}

/// Returns a artifact url.
/// If the provided version isn't available, it will look for the closest version within the major.
/// This means another minor, build or revision could be choosen.
async fn build_bc_artifact_url(
    deployment_type: &str,
    version: &str,
    country: &str,
) -> Result<String, Box<dyn Error>> {
    if !["sandbox", "onprem"].contains(&deployment_type) {
        return Err("cannot build artifact url, deployment type not valid".into());
    }
    let version = Version::new(version);

    let storage_account = "bcartifacts-exdbf9fwegejdqak.b02.azurefd.net".to_string();

    let base_url = format!("https://{storage_account}/{deployment_type}");
    let country_index_url = format!("{base_url}/indexes/{country}.json");

    let version = get_best_bc_artifact_version(&country_index_url, version).await?;

    Ok(format!("{base_url}/{version}/{country}"))
}

async fn get_best_bc_artifact_version(
    country_index_url: &str,
    searched_version: Version,
) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(country_index_url).await?.text().await?;
    // Data is expected to arrive in this format:
    // [{"Version":"15.4.41023.43755","CreationTime":"2020-06-26T00:13:59Z"},
    // {"Version":"16.0.11240.31204","CreationTime":"2021-10-11T08:49:00Z"}]
    let version_data: Vec<HashMap<String, String>> = serde_json::from_str(&body)?; // IDEA Cache?

    let available_versions: Vec<Version> = extract_available_versions(version_data);
    let closest_available_version: Version =
        search_closest_available_version(&searched_version, available_versions);

    Ok(closest_available_version.get_version_string())
}

fn extract_available_versions(version_data: Vec<HashMap<String, String>>) -> Vec<Version> {
    // TODO solve with map
    let mut available_versions: Vec<Version> = Vec::new();
    for d in version_data.iter() {
        available_versions.push(Version::new(d.get(&"Version".to_string()).unwrap()));
    }
    available_versions
}

// IDEA Provide Setting on how strict to be with version selection
fn search_closest_available_version(
    searched_version: &Version,
    mut available_versions: Vec<Version>,
) -> Version {
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
mod tests {
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
