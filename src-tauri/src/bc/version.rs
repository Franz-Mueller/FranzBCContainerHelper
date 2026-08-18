use std::fmt;
use std::num::ParseIntError;
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
        let mut parts = version.split('.');

        let major = parts.next().ok_or(BcVersionError::InvalidFormat)?.parse()?;

        let minor = parts.next().ok_or(BcVersionError::InvalidFormat)?.parse()?;

        let build = parts.next().ok_or(BcVersionError::InvalidFormat)?.parse()?;

        let revision = parts.next().ok_or(BcVersionError::InvalidFormat)?.parse()?;

        if parts.next().is_some() {
            return Err(BcVersionError::InvalidFormat);
        }

        Ok(Self {
            major,
            minor,
            build,
            revision,
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
    #[error("invalid BC version format; expected major.minor.build.revision")]
    InvalidFormat,

    #[error("invalid numeric version component: {0}")]
    InvalidComponent(#[from] ParseIntError),
}
