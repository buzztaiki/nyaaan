#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0}")]
    Nya(String),
}
