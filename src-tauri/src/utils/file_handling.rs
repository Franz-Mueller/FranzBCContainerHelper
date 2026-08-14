use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::{fs, io};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

struct LocalAppData {
    base_path: Path,
}

pub fn get_data_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        let local_app_data =
            env::var("LOCALAPPDATA").expect("LOCALAPPDATA environment variable not set");

        PathBuf::from(local_app_data).join("FranzBCDevHelper")
    } else if cfg!(target_os = "linux") {
        if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
            PathBuf::from(xdg_data_home).join("franzbcdevhelper")
        } else {
            let home = env::var("HOME").expect("HOME environment variable not set");

            PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("franzbcdevhelper")
        }
    } else {
        panic!("Unsupported operating system");
    }
}
