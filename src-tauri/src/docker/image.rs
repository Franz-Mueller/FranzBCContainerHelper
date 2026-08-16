use crate::utils::bc_version::BcVersion;
use crate::utils::file_handling::copy_dir_all;
use bollard::{body_full, query_parameters::BuildImageOptionsBuilder, Docker};
use bytes::Bytes;
use chrono::Local;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
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
    // TODO License
    let manifest = Manifest::from_file(artifact_path.join(Path::new("manifest.json")));
    let build_folder = create_temp_build_folder(artifact_path, artifact_url, &manifest);
    create_docker_image(manifest, build_folder).await;
}

async fn create_docker_image(manifest: Manifest, build_folder: std::path::PathBuf) {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let image_name = format!("bs{}winltsc2025:latest", manifest.version);
    let options = BuildImageOptionsBuilder::default()
        .dockerfile("dockerfile")
        .t(&image_name)
        .rm(true)
        .build();

    let tar_data = create_archive_from_build_folder(build_folder);

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

fn create_archive_from_build_folder(build_folder: std::path::PathBuf) -> Vec<u8> {
    let mut archive = Builder::new(Vec::new());
    archive.append_dir_all("", build_folder).unwrap();
    archive.finish().unwrap();

    let tar_data = archive.into_inner().unwrap();
    tar_data
}

fn create_temp_build_folder(
    artifact_path: &Path,
    artifact_url: &str,
    manifest: &Manifest,
) -> std::path::PathBuf {
    let build_folder = Path::new("images")
        .join(&manifest.country)
        .join(&manifest.version);
    let navdvd_folder = build_folder.join("NAVDVD");

    fs::create_dir_all(&build_folder).unwrap();
    fs::create_dir(&navdvd_folder).unwrap();

    populate_navdvd(artifact_path, manifest, navdvd_folder);

    let dockerfile = File::create(build_folder.join("/dockerfile")).unwrap();

    let base_image = "mcr.microsoft.com/businesscentral:ltsc2025-dev";
    let datetime = Local::now().format("%Y%m%d%H%M").to_string();
    let is_bc_sandbox = if manifest.isBcSandbox { "Y" } else { "N" };

    write_dockerfile(
        artifact_url,
        manifest,
        dockerfile,
        base_image,
        datetime,
        is_bc_sandbox,
    );
    build_folder
}

fn write_dockerfile(
    artifact_url: &str,
    manifest: &Manifest,
    mut dockerfile: File,
    base_image: &str,
    datetime: String,
    is_bc_sandbox: &str,
) {
    writeln!(dockerfile, "FROM {}", base_image).unwrap();
    writeln!(dockerfile, "ENV DatabaseServer=localhost DatabaseInstance=SQLEXPRESS DatabaseName=CRONUS IsBcSandbox={} artifactUrl={} filesOnly={}", is_bc_sandbox, artifact_url, false).unwrap();
    writeln!(dockerfile, "").unwrap();
    writeln!(dockerfile, "COPY NAVDVD /NAVDVD/").unwrap();
    writeln!(dockerfile, "").unwrap();
    writeln!(dockerfile, "RUN \\Run\\start.ps1 -installOnly").unwrap();
    writeln!(dockerfile, "").unwrap();
    writeln!(
        dockerfile,
        "LABEL legal=\"http://go.microsoft.com/fwlink/?LinkId=837447\" \\"
    )
    .unwrap();
    writeln!(dockerfile, "      created=\"{}\" \\", datetime).unwrap();
    writeln!(dockerfile, "      nav=\"{}\" \\", manifest.nav).unwrap();
    // TODO make this section more linux friendly
    writeln!(dockerfile, "      cu=\"{}\" \\", manifest.cu).unwrap();
    writeln!(dockerfile, "      country=\"{}\" \\", manifest.country).unwrap();
    writeln!(dockerfile, "      version=\"{}\" \\", manifest.version).unwrap();
    writeln!(dockerfile, "      platform=\"{}\"", manifest.platform).unwrap();
}

fn populate_navdvd(artifact_path: &Path, manifest: &Manifest, navdvd_folder: std::path::PathBuf) {
    copy_demo_db_into_navdvd(&navdvd_folder, artifact_path, manifest);

    // Copies files required for build from artifact folder into build folder
    for entry in artifact_path.read_dir().unwrap() {
        match entry {
            Ok(entry) => {
                let file_name = entry.file_name();
                if [
                    "Installers",
                    "ConfigurationPackages",
                    "TestToolKit",
                    "UpgradeToolKit",
                    "Extensions",
                ]
                .contains(&file_name.to_str().unwrap())
                    || file_name.to_str().unwrap().starts_with("Applications")
                {
                    let destination = navdvd_folder.join(file_name);
                    if entry.path().is_dir() {
                        copy_dir_all(entry.path(), destination).unwrap();
                    } else {
                        fs::copy(entry.path(), &destination).unwrap();
                    }
                }
            }
            Err(e) => panic!("{e}"),
        }
    }
}

fn copy_demo_db_into_navdvd(navdvd_folder: &Path, artifact_path: &Path, manifest: &Manifest) {
    let db_path = artifact_path.join(manifest.database.replace("\\", "/"));
    let commondata = if BcVersion::from_str(&manifest.version).unwrap()
        < BcVersion::from_str("27.0.33344.0").unwrap()
    {
        "CommonAppData"
    } else {
        "CommApp"
    };

    let demo_db_dir = navdvd_folder
        .join("SQLDemoDatabase")
        .join(commondata)
        .join("Microsoft")
        .join("Microsoft Dynamics NAV")
        .join("ver")
        .join("Database");

    fs::create_dir_all(&demo_db_dir).unwrap();

    let demo_db_path = demo_db_dir.join("database.bak");

    fs::copy(&db_path, &demo_db_path).unwrap();
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
                let p = Path::new("./artifacts/de/15.4.41023.43755");
                let u = "https://bcartifacts-exdbf9fwegejdqak.b02.azurefd.net/sandbox/15.4.41023.43755/de";
                build_image(p, u).await;
            })
    }
}
