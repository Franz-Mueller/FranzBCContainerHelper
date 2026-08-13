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
    let manifest_path = format!("{}/{}", artifact_path.to_str().unwrap(), "manifest.json");
    let manifest = Manifest::from_file(manifest_path);

    let temp_build_folder = format!("images/{}/{}", manifest.country, manifest.version);
    let temp_build_folder = Path::new(&temp_build_folder);
    fs::create_dir(temp_build_folder).unwrap(); // TODO check if Path exists in case deletion is not handled PROPERLY

    let temp_navdvd_folder = format!("{}/NAVDVD", temp_build_folder.to_str().unwrap());
    let temp_navdvd_folder = Path::new(&temp_navdvd_folder);
    fs::create_dir(temp_navdvd_folder).unwrap();

    let dbpath =
        format!("{}/{}", artifact_path.to_str().unwrap(), manifest.database).replace("\\", "/");

    let commondata = if BcVersion::from_str(&manifest.version) < BcVersion::from_str("27.0.33344.0")
    {
        "CommonAppData"
    } else {
        "CommApp"
    };

    let demo_db_dir = format!(
        "./{}/SQLDemoDatabase/{}/Microsoft/Microsoft Dynamics NAV/ver/Database",
        temp_navdvd_folder.to_str().unwrap(),
        commondata
    );

    fs::create_dir_all(&demo_db_dir).unwrap();

    let demo_db_path = format!("{}/database.bak", demo_db_dir);

    dbg!(&dbpath);
    dbg!(&demo_db_path);

    fs::copy(&dbpath, &demo_db_path).unwrap();

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
                    dbg!(&file_name);
                    let destination = format!(
                        "{}/{}",
                        temp_navdvd_folder.to_str().unwrap(),
                        file_name.into_string().unwrap()
                    );
                    println!("entry: {entry:?} | destination: {destination:?}");
                    if entry.path().is_dir() {
                        copy_dir_all(entry.path(), destination).unwrap();
                    } else {
                        fs::copy(entry.path(), &destination).unwrap();
                    }
                }
                println!("{:?}", entry.path());
            }
            Err(e) => panic!("{e}"),
        }
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
    writeln!(dockerfile, "      nav=\"{}\" \\", manifest.nav).unwrap(); // TODO make this section more linux friendly
    writeln!(dockerfile, "      cu=\"{}\" \\", manifest.cu).unwrap();
    writeln!(dockerfile, "      country=\"{}\" \\", manifest.country).unwrap();
    writeln!(dockerfile, "      version=\"{}\" \\", manifest.version).unwrap();
    writeln!(dockerfile, "      platform=\"{}\"", manifest.platform).unwrap();

    let docker = Docker::connect_with_local_defaults().unwrap();

    let image_name = format!("bs{}winltsc2025:latest", manifest.version);

    let options = BuildImageOptionsBuilder::default()
        .dockerfile("dockerfile")
        .t(&image_name)
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
                let p = Path::new("./artifacts/de/15.4.41023.43755");
                let u = "https://bcartifacts-exdbf9fwegejdqak.b02.azurefd.net/sandbox/15.4.41023.43755/de";
                build_image(p, u).await;
            })
    }
}
