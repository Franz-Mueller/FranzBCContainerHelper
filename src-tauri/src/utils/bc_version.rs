#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct BcVersion {
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub revision: u32,
}

impl BcVersion {
    pub fn from_str(version: &str) -> BcVersion {
        let v: Vec<&str> = version.split(".").collect();

        BcVersion {
            major: v[0].parse().unwrap(),
            minor: v[1].parse().unwrap(),
            build: v[2].parse().unwrap(),
            revision: v[3].parse().unwrap(),
        }
    }

    pub fn get_version_string(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.revision
        )
    }
}
