use thiserror::Error;

#[derive(Error,Debug,Clone)]
pub Enum Error {
    #[error("Usage: triscanner <awebsite.com>")]
    CliUsage,
}
