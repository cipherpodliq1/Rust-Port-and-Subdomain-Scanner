use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error{
    #[error("Usage : tricoder <kenkour.com>")]
    CliUsage,
}