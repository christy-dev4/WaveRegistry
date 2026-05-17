use crate::errors::RegistryError;
use crate::events::RegistryEvent;
use crate::state::{ContributorProfile, Ecosystem, WaveRegistry};
use crate::validation;

impl WaveRegistry {
    /// Creates a new registry with the authorized app address.
    pub fn new(app_address: String) -> Self {
        Self {
            registry: Vec::new(),
            authorized_wave_app: app_address,
            ecosystems: Vec::new(),
        }
    }

    /// Returns a mutable reference to a developer's profile, creating one if absent,
    /// along with a bool indicating whether the profile was newly created.
    fn get_or_create_profile(&mut self, developer: &str) -> (&mut ContributorProfile, bool) {
        let idx = self.registry.iter().position(|(addr, _)| addr == developer);
        match idx {
            Some(i) => (&mut self.registry[i].1, false),
            None => {
                self.registry.push((
                    developer.to_string(),
                    ContributorProfile {
                        total_points_earned: 0,
                        sprints_completed: 0,
                        ecosystem_badges: Vec::new(),
                    },
                ));
                (self.registry.last_mut().unwrap().1, true)
            }
        }
    }

    /// Records a contribution after a Wave / Sprint completes.
    /// Only the globally authorized app may call this.
    pub fn record_contribution(
        &mut self,
        caller: &str,
        developer: &str,
        points: u64,
        wave_tag: &str,
    ) -> Result<RegistryEvent, RegistryError> {
        if caller != self.authorized_wave_app {
            return Err(RegistryError::Unauthorized);
        }

        validation::validate_badge_tag(wave_tag)?;
        validation::validate_points(points)?;
        validation::validate_address(developer)?;

        // Extract ecosystem prefix from wave_tag ("Stellar-Wave-1" -> "Stellar")
        let ecosystem_name = wave_tag.split('-').next().unwrap_or("");
        if !ecosystem_name.is_empty() {
            let is_whitelisted = self
                .ecosystems
                .iter()
                .any(|e| e.name == ecosystem_name && e.active);
            if !is_whitelisted {
                return Err(RegistryError::EcosystemNotWhitelisted(
                    ecosystem_name.to_string(),
                ));
            }
        }

        let (profile, is_new) = self.get_or_create_profile(developer);

        let new_total = profile
            .total_points_earned
            .checked_add(points)
            .ok_or(RegistryError::PointsOverflow(points))?;

        let new_sprints = profile
            .sprints_completed
            .checked_add(1)
            .ok_or(RegistryError::SprintOverflow)?;

        profile.total_points_earned = new_total;
        profile.sprints_completed = new_sprints;
        profile.ecosystem_badges.push(wave_tag.to_string());

        Ok(RegistryEvent::ContributionRecorded {
            developer: developer.to_string(),
            points,
            wave_tag: wave_tag.to_string(),
            new_total,
            sprints_completed: new_sprints,
            profile_created: is_new,
        })
    }

    /// Whitelists a new ecosystem.
    pub fn whitelist_ecosystem(
        &mut self,
        caller: &str,
        name: &str,
        authorized_address: &str,
    ) -> Result<RegistryEvent, RegistryError> {
        if caller != self.authorized_wave_app {
            return Err(RegistryError::Unauthorized);
        }

        validation::validate_ecosystem_name(name)?;
        validation::validate_address(authorized_address)?;

        if self.ecosystems.iter().any(|e| e.name == name) {
            return Err(RegistryError::EcosystemAlreadyWhitelisted(
                name.to_string(),
            ));
        }

        self.ecosystems.push(Ecosystem {
            name: name.to_string(),
            authorized_address: authorized_address.to_string(),
            active: true,
        });

        Ok(RegistryEvent::EcosystemWhitelisted {
            name: name.to_string(),
            authorized_address: authorized_address.to_string(),
        })
    }

    /// Deactivates an ecosystem (soft-removal).
    pub fn deactivate_ecosystem(
        &mut self,
        caller: &str,
        name: &str,
    ) -> Result<RegistryEvent, RegistryError> {
        if caller != self.authorized_wave_app {
            return Err(RegistryError::Unauthorized);
        }

        let eco = self
            .ecosystems
            .iter_mut()
            .find(|e| e.name == name)
            .ok_or_else(|| RegistryError::EcosystemNotWhitelisted(name.to_string()))?;

        eco.active = false;

        Ok(RegistryEvent::EcosystemDeactivated {
            name: name.to_string(),
        })
    }

    /// Queries a developer's profile.
    pub fn get_profile(&self, developer: &str) -> Result<&ContributorProfile, RegistryError> {
        let addr = developer;
        self.registry
            .iter()
            .find(|(a, _)| a == addr)
            .map(|(_, p)| p)
            .ok_or_else(|| RegistryError::ProfileNotFound(addr.to_string()))
    }

    /// Returns all developers whose points exceed a threshold.
    pub fn get_high_tier_contributors(&self, min_points: u64) -> Vec<&ContributorProfile> {
        self.registry
            .iter()
            .filter(|(_, p)| p.total_points_earned >= min_points)
            .map(|(_, p)| p)
            .collect()
    }

    /// Returns developers who hold a specific badge.
    pub fn get_contributors_by_badge(&self, badge: &str) -> Vec<&ContributorProfile> {
        self.registry
            .iter()
            .filter(|(_, p)| p.ecosystem_badges.iter().any(|b| b == badge))
            .map(|(_, p)| p)
            .collect()
    }

    /// Returns the total number of registered developers.
    pub fn total_developers(&self) -> usize {
        self.registry.len()
    }

    /// Test helper — gives direct mutable access to a profile.
    #[cfg(test)]
    pub fn get_or_create_profile_for_test(&mut self, developer: &str) -> &mut ContributorProfile {
        self.get_or_create_profile(developer).0
    }
}
