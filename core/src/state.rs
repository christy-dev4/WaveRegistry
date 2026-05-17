use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// On-chain profile for a developer contributor.
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub struct ContributorProfile {
    /// Cumulative wave points earned across all ecosystems.
    pub total_points_earned: u64,
    /// Number of sprints/waves the developer has completed.
    pub sprints_completed: u32,
    /// Verifiable badges e.g. "Stellar-Wave-1", "Drips-Core".
    pub ecosystem_badges: Vec<String>,
}

/// A registered ecosystem that can issue waves.
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub struct Ecosystem {
    /// Unique name / identifier.
    pub name: String,
    /// Wallet or app address authorized to record contributions for this ecosystem.
    pub authorized_address: String,
    /// Whether the ecosystem is currently active.
    pub active: bool,
}

/// Core on-chain registry that tracks contributor reputation across ecosystems.
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub struct WaveRegistry {
    /// Maps developer wallet address to their aggregated profile.
    pub registry: Vec<(String, ContributorProfile)>,
    /// Globally authorized app address (Drips GitHub App wallet).
    pub authorized_wave_app: String,
    /// Whitelisted ecosystems.
    pub ecosystems: Vec<Ecosystem>,
}
