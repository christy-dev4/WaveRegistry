use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    #[error("caller is not authorized to perform this action")]
    Unauthorized,

    #[error("ecosystem '{0}' is not whitelisted")]
    EcosystemNotWhitelisted(String),

    #[error("ecosystem '{0}' is already whitelisted")]
    EcosystemAlreadyWhitelisted(String),

    #[error("developer profile not found for address {0}")]
    ProfileNotFound(String),

    #[error("points overflow: attempted to add {0} but total would exceed u64::MAX")]
    PointsOverflow(u64),

    #[error("sprint count overflow")]
    SprintOverflow,

    #[error("badge tag '{0}' exceeds maximum length of {1}")]
    BadgeTagTooLong(String, usize),

    #[error("ecosystem name '{0}' exceeds maximum length of {1}")]
    EcosystemNameTooLong(String, usize),

    #[error("points value must be positive, got {0}")]
    InvalidPoints(u64),

    #[error("address '{0}' is not a valid base58 address")]
    InvalidAddress(String),
}
