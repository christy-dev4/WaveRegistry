use crate::errors::RegistryError;

pub const MAX_BADGE_TAG_LENGTH: usize = 64;
pub const MAX_ECOSYSTEM_NAME_LENGTH: usize = 64;
pub const MAX_ADDRESS_LENGTH: usize = 44; // base58 pubkey

/// Validates a badge/tag string.
pub fn validate_badge_tag(tag: &str) -> Result<(), RegistryError> {
    if tag.is_empty() {
        return Err(RegistryError::BadgeTagTooLong(tag.to_string(), MAX_BADGE_TAG_LENGTH));
    }
    if tag.len() > MAX_BADGE_TAG_LENGTH {
        return Err(RegistryError::BadgeTagTooLong(
            tag.to_string(),
            MAX_BADGE_TAG_LENGTH,
        ));
    }
    Ok(())
}

/// Validates an ecosystem name.
pub fn validate_ecosystem_name(name: &str) -> Result<(), RegistryError> {
    if name.is_empty() {
        return Err(RegistryError::EcosystemNameTooLong(
            name.to_string(),
            MAX_ECOSYSTEM_NAME_LENGTH,
        ));
    }
    if name.len() > MAX_ECOSYSTEM_NAME_LENGTH {
        return Err(RegistryError::EcosystemNameTooLong(
            name.to_string(),
            MAX_ECOSYSTEM_NAME_LENGTH,
        ));
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(RegistryError::EcosystemNameTooLong(
            name.to_string(),
            MAX_ECOSYSTEM_NAME_LENGTH,
        ));
    }
    Ok(())
}

/// Validates a base58 address (length check only — full checksum requires solana libs).
pub fn validate_address(addr: &str) -> Result<(), RegistryError> {
    if addr.is_empty() || addr.len() > MAX_ADDRESS_LENGTH {
        return Err(RegistryError::InvalidAddress(addr.to_string()));
    }
    if !addr
        .chars()
        .all(|c| c.is_alphanumeric() || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7' || c == '8' || c == '9')
    {
        return Err(RegistryError::InvalidAddress(addr.to_string()));
    }
    Ok(())
}

/// Validates points are positive.
pub fn validate_points(points: u64) -> Result<(), RegistryError> {
    if points == 0 {
        return Err(RegistryError::InvalidPoints(0));
    }
    Ok(())
}
