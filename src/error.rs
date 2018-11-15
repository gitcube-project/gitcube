use std::error;
use std::fmt;
use std::io;

#[derive(Eq, PartialEq, Clone)]
pub struct GitcubeError{
    state: String,
    message: String,
    code: u16,
}

impl fmt::Display for GitcubeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR {} ({}): {}", self.code, self.state, self.message)
    }
}

impl fmt::Debug for GitcubeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl error::Error for GitcubeError {
    fn description(&self) -> &str {
        "Error returned by a server"
    }
}

pub enum Error {
    IoError(io::Error),
    MySqlError(mysql::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => write!(f, "IoError {{ {} }}", err),
            Error::MySqlError(ref err) => write!(f, "MySqlError {{ {} }}", err),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(_) => "I/O Error",
            Error::MySqlError(_) => "MySql server error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::MySqlError(ref err) => Some(err),
            _ => None,
        }
    }
}


impl From<mysql::error::Error> for Error {
    fn from(err: mysql::error::Error) -> Error {
        Error::MySqlError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}