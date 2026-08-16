use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct BcVersion {
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub revision: u32,
}

impl FromStr for BcVersion {
    type Err = BcVersionError;
    fn from_str(version: &str) -> Result<Self, Self::Err> {
        // IDEA use std::str::FromStr Trait to impl from_str
        let v: Vec<&str> = version.split(".").collect();

        Ok(BcVersion {
            major: v[0].parse().unwrap(),
            minor: v[1].parse().unwrap(),
            build: v[2].parse().unwrap(),
            revision: v[3].parse().unwrap(),
        })
    }
}

impl fmt::Display for BcVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.build, self.revision
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BcVersionError {
    #[error("could not parse version from str")]
    ParseFromString(),
}
