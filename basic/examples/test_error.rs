use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("{0}")]
    Error(String),

    #[error("未知错误, 请联系开发人员.")]
    Unknown,

    #[error("UserError, E: {0}")]
    UserError(#[from] UserError),
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("E: {0}")]
    Error(String),

    #[error("未知错误, 请联系开发人员.")]
    Unknown,
}

fn test_error() -> Result<(), PlayerError> {
    let _ = Err(UserError::Unknown)?;
    Ok(())
}

fn main() {
    let e = PlayerError::UserError(UserError::Unknown);
    println!("{}", e);
    println!("{:?}", test_error());
}
