use std::error::Error;
use std::fmt;

//#region Terminal Input Parsing Errors
#[derive(Debug)]
pub struct UnknownCommand {
    pub false_command: String,
}

impl fmt::Display for UnknownCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown command: {}", self.false_command)
    }
}

impl Error for UnknownCommand {}

#[derive(Debug)]
pub struct NoCommand;

impl fmt::Display for NoCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "please provide a command. use -h for help.")
    }
}

impl Error for NoCommand {}
//#endregion Terminal Input Parsing Errors

//#region Docker Errors
#[derive(Debug)]
pub struct FailedBCContainerCreation {
    pub status: String,
    pub stderr: String,
}

impl fmt::Display for FailedBCContainerCreation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "could not create the bc container with {}\n{}",
            self.status,
            self.stderr.trim()
        )
    }
}

impl Error for FailedBCContainerCreation {}

#[derive(Debug)]
pub struct FailedContainerRemoval {
    pub status: String,
    pub stderr: String,
}

impl fmt::Display for FailedContainerRemoval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "could not remove the container with: {}\n{}",
            self.status,
            self.stderr.trim()
        )
    }
}

impl Error for FailedContainerRemoval {}
//#endregion Docker Errors

//#region compatability errors
#[derive(Debug)]
pub struct OnlyAvailableOnWindows {
    pub command: String,
}

impl fmt::Display for OnlyAvailableOnWindows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "this command is only available on windows: {}",
            self.command
        )
    }
}

impl Error for OnlyAvailableOnWindows {}
//#endregion compatability errors
