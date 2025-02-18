use std::process::{ExitCode, Termination};

#[derive(Debug)]
pub enum ReturnCode {
    Success,
    ArgsError,
    LoginError,
    LogoutError,
    ConfigError,
}

impl Termination for ReturnCode {
    fn report(self) -> ExitCode {
        match self {
            ReturnCode::Success => ExitCode::SUCCESS,
            ReturnCode::ArgsError => ExitCode::from(1),
            ReturnCode::LoginError => ExitCode::from(2),
            ReturnCode::LogoutError => ExitCode::from(3),
            ReturnCode::ConfigError => ExitCode::from(4),
        }
    }
}
