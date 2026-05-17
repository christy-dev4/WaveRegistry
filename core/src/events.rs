use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum RegistryEvent {
    ContributionRecorded {
        developer: String,
        points: u64,
        wave_tag: String,
        new_total: u64,
        sprints_completed: u32,
        profile_created: bool,
    },
    EcosystemWhitelisted {
        name: String,
        authorized_address: String,
    },
    EcosystemDeactivated {
        name: String,
    },
}
