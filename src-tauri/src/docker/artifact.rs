use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

struct Version {
    major: String,
    minor: String,
    build: String,
    revision: String,
}

impl Version {
    fn new(version: &str) -> Version {
        let mut v: Vec<&str> = version.split(".").collect();
        match v.len() {
            1 => {
                v.push("0");
                v.push("0");
                v.push("0");
            }
            2 => {
                v.push("0");
                v.push("0");
            }
            3 => {
                v.push("0");
            }
            4 => {}
            _ => {
                panic!("wrong format for version")
            }
        };
        Version {
            major: v[0].to_string(),
            minor: v[1].to_string(),
            build: v[2].to_string(),
            revision: v[3].to_string(),
        }
    }

    fn get_version_string(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.revision
        )
    }
}

async fn build_bc_artifact_url(
    deployment_type: &str,
    country: &str,
    version: &str,
) -> Result<String, Box<dyn Error>> {
    let available_deployment_types = ["sandbox", "onprem"];
    if !available_deployment_types.contains(&deployment_type) {
        return Err("cannot build artifact url, deployment type not valid".into());
    }
    let storage_account = "bcartifacts-exdbf9fwegejdqak.b02.azurefd.net".to_string();
    let base_url = format!("https://{storage_account}/{deployment_type}");
    let lang_index_url = format!("{base_url}/indexes/{country}.json");
    let version = get_best_bc_artifact_version(&lang_index_url, version).await?; // TODO Version query
    Ok(format!("{base_url}/{version}/{country}"))
}

async fn get_best_bc_artifact_version(
    lang_index_url: &str,
    version: &str,
) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(lang_index_url).await?.text().await?;
    let data: Vec<HashMap<String, String>> = serde_json::from_str(&body)?;
    let available_versions = extract_available_versions(data);
    let searched_version = Version::new(&version);
    let available_versions = filter_available_versions(&searched_version, available_versions);
    Ok(available_versions.get_version_string())
}

fn filter_available_versions(
    searched_version: &Version,
    available_versions: Vec<Version>,
) -> Version {
    let mut lvl_version_percision: u32 = 0;
    for i in &available_versions {
        if i.major == searched_version.major {
            lvl_version_percision = 1;
            if i.minor == searched_version.minor {
                lvl_version_percision = 2;
                if i.build == searched_version.build {
                    lvl_version_percision = 3;
                    if i.revision == searched_version.revision {
                        return Version {
                            major: i.major.to_string(),
                            minor: i.minor.to_string(),
                            build: i.build.to_string(),
                            revision: i.revision.to_string(),
                        };
                    }
                }
            }
        }
    }

    let mut new_available_versions: Vec<Version> = Vec::new();

    if lvl_version_percision == 1 {
        for i in available_versions {
            if i.major == searched_version.major {
                new_available_versions.push(i);
            }
        }
    } else if lvl_version_percision == 2 {
        for i in available_versions {
            if i.major == searched_version.major && i.minor == searched_version.minor {
                new_available_versions.push(i);
            }
        }
    } else if lvl_version_percision == 3 {
        for i in available_versions {
            if i.major == searched_version.major
                && i.minor == searched_version.minor
                && i.build == searched_version.build
            {
                new_available_versions.push(i);
            }
        }
    } else {
        panic!("Version format not valid");
    }

    let v = new_available_versions.last().unwrap();

    Version {
        major: v.major.to_string(),
        minor: v.minor.to_string(),
        build: v.build.to_string(),
        revision: v.revision.to_string(),
    }
}

fn extract_available_versions(data: Vec<HashMap<String, String>>) -> Vec<Version> {
    let mut available_versions: Vec<Version> = Vec::new();
    for d in data.iter() {
        available_versions.push(Version::new(d.get(&"Version".to_string()).unwrap()));
    }
    available_versions
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
                let result = build_bc_artifact_url("sandbox", "de", "15").await;
                match result {
                    Ok(s) => println!("{s}"),
                    Err(e) => panic!("unexpected error: {e}"),
                };
            })
    }
}
