use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, HermesSsdLlmError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExitCode {
    Success = 0,
    General = 1,
    SsdMissing = 10,
    IdentityMismatch = 11,
    ReadOnlyVolume = 12,
    InsufficientSpace = 13,
    DirectoryInitFailed = 14,
    HermesMissing = 15,
    InvalidConfig = 16,
    LockConflict = 17,
    RuntimeFailure = 18,
    FallbackRefused = 19,
}

impl ExitCode {
    pub fn code(self) -> i32 {
        self as i32
    }
}

impl From<ExitCode> for i32 {
    fn from(value: ExitCode) -> Self {
        value.code()
    }
}

#[derive(Debug, Error)]
pub enum HermesSsdLlmError {
    #[error("Hermes SSD LLM could not find the registered external volume.\nConnect the SSD and retry: hermes ssd")]
    SsdMissing,

    #[error("Hermes SSD LLM found a volume but its identity does not match registration.\nExpected volume UUID {expected}, found {found}.")]
    IdentityMismatch { expected: String, found: String },

    #[error("Hermes SSD LLM found the volume, but it is read-only.\nRepair or remount the drive before continuing.")]
    ReadOnlyVolume,

    #[error(
        "Hermes SSD LLM requires at least {required_gb} GB free.\nAvailable: {available_gb} GB."
    )]
    InsufficientSpace { required_gb: u64, available_gb: u64 },

    #[error("Hermes SSD LLM failed to initialize directory {path}: {reason}")]
    DirectoryInitFailed { path: String, reason: String },

    #[error("Hermes SSD LLM could not locate the real Hermes executable.\nRun ./install.sh or set hermes_executable in ~/.config/hermes-ssd-llm/config.toml")]
    HermesMissing,

    #[error("Hermes SSD LLM configuration error: {0}")]
    InvalidConfig(String),

    #[error("Hermes SSD LLM refused to start: another SSD-mode session is active (PID {pid}).")]
    LockConflict { pid: u32 },

    #[error("Hermes SSD LLM refused to fall back to internal storage.")]
    FallbackRefused,

    #[error("{0}")]
    Other(String),
}

impl HermesSsdLlmError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::SsdMissing => ExitCode::SsdMissing,
            Self::IdentityMismatch { .. } => ExitCode::IdentityMismatch,
            Self::ReadOnlyVolume => ExitCode::ReadOnlyVolume,
            Self::InsufficientSpace { .. } => ExitCode::InsufficientSpace,
            Self::DirectoryInitFailed { .. } => ExitCode::DirectoryInitFailed,
            Self::HermesMissing => ExitCode::HermesMissing,
            Self::InvalidConfig(_) => ExitCode::InvalidConfig,
            Self::LockConflict { .. } => ExitCode::LockConflict,
            Self::FallbackRefused => ExitCode::FallbackRefused,
            Self::Other(_) => ExitCode::RuntimeFailure,
        }
    }
}

impl fmt::Display for ExitCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as i32)
    }
}
