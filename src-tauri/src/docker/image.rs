use bollard::{body_full, query_parameters::BuildImageOptionsBuilder, Docker};
use bytes::Bytes;
use chrono::Local;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tar::Builder;

// TODO Refactoring
// TODO Error Handling
// TODO Testing

#[derive(Deserialize, Serialize, Debug)]
struct Manifest {
    version: String, // IDEA use version struct from artifact
    platformUrl: String,
    licenseFile: String,
    isBcSandbox: bool,
    nav: String,
    cu: String,
    country: String,
    platform: String,
    database: String,
}

impl Manifest {
    fn from_file<P>(path: P) -> Manifest
    where
        P: AsRef<Path>,
    {
        let data = fs::read_to_string(path).expect("Should have been able to read the file");
        serde_json::from_str(&data).unwrap()
    }
}

pub async fn build_image(artifact_path: &Path, artifact_url: &str) {
    let manifest_path = format!("{}/{}", artifact_path.to_str().unwrap(), "manifest.json");
    let manifest = Manifest::from_file(manifest_path);

    let temp_build_folder = format!("images/{}/{}", manifest.country, manifest.version);
    let temp_build_folder = Path::new(&temp_build_folder);
    // TODO check if PAth exists in case deletion is not handled PROPERLY
    if !temp_build_folder.try_exists().unwrap() {
        fs::create_dir(temp_build_folder).unwrap();
    }

    let mut dockerfile = File::create(format!(
        "{}/dockerfile",
        temp_build_folder.to_str().unwrap()
    ))
    .unwrap();

    let base_image = "mcr.microsoft.com/businesscentral:ltsc2025-dev";
    let datetime = Local::now().format("%Y%m%d%H%M").to_string();
    let is_bc_sandbox = if manifest.isBcSandbox { "Y" } else { "N" };

    writeln!(dockerfile, "FROM {}", base_image).unwrap();
    writeln!(dockerfile, "ENV DatabaseServer=localhost DatabaseInstance=SQLEXPRESS DatabaseName=CRONUS IsBcSandbox={} artifactUrl={} filesOnly={}", is_bc_sandbox, artifact_url, false).unwrap();
    writeln!(dockerfile, "").unwrap();
    writeln!(dockerfile, "COPY my /run/").unwrap();
    writeln!(dockerfile, "COPY NAVDVD /NAVDVD/").unwrap();
    writeln!(dockerfile, "$DockerFileAddFonts").unwrap(); // TODO $DockerFileAddFonts
    writeln!(dockerfile, "").unwrap();
    writeln!(dockerfile, "RUN \\Run\\start.ps1 -installOnly").unwrap();
    writeln!(dockerfile, "").unwrap();
    writeln!(
        dockerfile,
        "LABEL legal=\"http://go.microsoft.com/fwlink/?LinkId=837447\" \\"
    )
    .unwrap();
    writeln!(dockerfile, "      created=\"{}\" \\", datetime).unwrap();
    writeln!(dockerfile, "      nav=\"{}\" \\", manifest.nav).unwrap(); // TODO make this section more linux friendly
    writeln!(dockerfile, "      cu=\"{}\" \\", manifest.cu).unwrap();
    writeln!(dockerfile, "      country=\"{}\" \\", manifest.country).unwrap();
    writeln!(dockerfile, "      version=\"{}\" \\", manifest.version).unwrap();
    writeln!(dockerfile, "      platform=\"{}\"", manifest.platform).unwrap();

    let docker = Docker::connect_with_local_defaults().unwrap();

    let options = BuildImageOptionsBuilder::default()
        .dockerfile("dockerfile")
        .t("my-image:latest")
        .rm(true)
        .build();

    let mut archive = Builder::new(Vec::new());
    archive.append_dir_all("", temp_build_folder).unwrap();
    archive.finish().unwrap();

    let tar_data = archive.into_inner().unwrap();

    // `tar_data` must contain your Docker build context as a tar archive.
    let mut stream = docker.build_image(options, None, Some(body_full(Bytes::from(tar_data))));

    while let Some(result) = stream.next().await {
        match result {
            Ok(info) => {
                println!("{info:?}");
            }
            Err(err) => {
                eprintln!("Docker build failed: {err:?}");
            }
        }
    }
}

#[cfg(test)]
mod test_build_image {
    use super::*;

    #[test]
    fn my_test() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let p = Path::new("/home/franz/Repos/FranzBCContainerHelper/src-tauri/artifacts/de/15.4.41023.43755");
                build_image(p).await;
            })
    }
}
