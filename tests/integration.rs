use wave_registry_core::errors::RegistryError;
use wave_registry_core::events::RegistryEvent;
use wave_registry_core::state::WaveRegistry;

/// Returns a test registry with a dummy app address.
fn setup() -> WaveRegistry {
    WaveRegistry::new("DripsApp111111111111111111111111111111111".to_string())
}

#[test]
fn test_new_registry_is_empty() {
    let reg = setup();
    assert_eq!(reg.total_developers(), 0);
    assert_eq!(reg.authorized_wave_app, "DripsApp111111111111111111111111111111111");
    assert!(reg.ecosystems.is_empty());
}

#[test]
fn test_record_contribution_creates_profile() {
    let mut reg = setup();
    let app = reg.authorized_wave_app.clone();
    let dev = "DevWallet1111111111111111111111111111111111";

    // Must whitelist ecosystem first
    reg.whitelist_ecosystem(&app, "Stellar", "StellarAuth11111111111111111111111111")
        .unwrap();

    let event = reg
        .record_contribution(&app, dev, 100, "Stellar-Wave-1")
        .unwrap();

    assert_eq!(
        event,
        RegistryEvent::ContributionRecorded {
            developer: dev.to_string(),
            points: 100,
            wave_tag: "Stellar-Wave-1".to_string(),
            new_total: 100,
            sprints_completed: 1,
            profile_created: true,
        }
    );

    let profile = reg.get_profile(dev).unwrap();
    assert_eq!(profile.total_points_earned, 100);
    assert_eq!(profile.sprints_completed, 1);
    assert_eq!(profile.ecosystem_badges, vec!["Stellar-Wave-1"]);
}

#[test]
fn test_record_contribution_accumulates_points() {
    let mut reg = setup();
    let app = reg.authorized_wave_app.clone();
    let dev = "DevWallet1111111111111111111111111111111111";

    reg.whitelist_ecosystem(&app, "Stellar", "StellarAuth11111111111111111111111111")
        .unwrap();
    reg.whitelist_ecosystem(&app, "Arbitrum", "ArbitrumAuth1111111111111111111111111")
        .unwrap();

    reg.record_contribution(&app, dev, 50, "Stellar-Wave-1").unwrap();
    reg.record_contribution(&app, dev, 75, "Arbitrum-Wave-2").unwrap();

    let profile = reg.get_profile(dev).unwrap();
    assert_eq!(profile.total_points_earned, 125);
    assert_eq!(profile.sprints_completed, 2);
    assert_eq!(
        profile.ecosystem_badges,
        vec!["Stellar-Wave-1", "Arbitrum-Wave-2"]
    );
}

#[test]
fn test_unauthorized_caller_rejected() {
    let mut reg = setup();
    let dev = "DevWallet1111111111111111111111111111111111";

    let err = reg
        .record_contribution("EvilApp111111111111111111111111111111111", dev, 100, "Stellar-Wave-1")
        .unwrap_err();

    assert_eq!(err, RegistryError::Unauthorized);
}

#[test]
fn test_ecosystem_not_whitelisted_rejected() {
    let mut reg = setup();
    let app = reg.authorized_wave_app.clone();
    let dev = "DevWallet1111111111111111111111111111111111";

    let err = reg
        .record_contribution(&app, dev, 100, "RogueEco-Wave-1")
        .unwrap_err();

    assert_eq!(
        err,
        RegistryError::EcosystemNotWhitelisted("RogueEco".to_string())
    );
}

#[test]
fn test_ecosystem_lifecycle() {
    let mut reg = setup();
    let app = reg.authorized_wave_app.clone();

    // Whitelist
    let event = reg
        .whitelist_ecosystem(&app, "Solana", "SolanaAuth111111111111111111111111111")
        .unwrap();
    assert_eq!(
        event,
        RegistryEvent::EcosystemWhitelisted {
            name: "Solana".to_string(),
            authorized_address: "SolanaAuth111111111111111111111111111".to_string(),
        }
    );

    // Duplicate reject
    let err = reg
        .whitelist_ecosystem(&app, "Solana", "Other1111111111111111111111111111111111")
        .unwrap_err();
    assert_eq!(
        err,
        RegistryError::EcosystemAlreadyWhitelisted("Solana".to_string())
    );

    // Deactivate
    let event = reg.deactivate_ecosystem(&app, "Solana").unwrap();
    assert_eq!(
        event,
        RegistryEvent::EcosystemDeactivated {
            name: "Solana".to_string()
        }
    );

    // Contributions to deactivated ecosystem fail
    let err = reg.record_contribution(
        &app,
        "DevWallet1111111111111111111111111111111111",
        10,
        "Solana-Wave-3",
    );
    assert_eq!(
        err,
        Err(RegistryError::EcosystemNotWhitelisted("Solana".to_string()))
    );
}

#[test]
fn test_high_tier_query() {
    let mut reg = setup();
    let app = reg.authorized_wave_app.clone();

    reg.whitelist_ecosystem(&app, "Stellar", "StellarAuth11111111111111111111111111")
        .unwrap();

    reg.record_contribution(&app, "Dev1001111111111111111111111111111111111", 200, "Stellar-Wave-1")
        .unwrap();
    reg.record_contribution(&app, "Dev0501111111111111111111111111111111111", 50, "Stellar-Wave-1")
        .unwrap();

    let high = reg.get_high_tier_contributors(100);
    assert_eq!(high.len(), 1);
    assert_eq!(high[0].total_points_earned, 200);

    let all = reg.get_high_tier_contributors(0);
    assert_eq!(all.len(), 2);
}

#[test]
fn test_contributors_by_badge() {
    let mut reg = setup();
    let app = reg.authorized_wave_app.clone();

    reg.whitelist_ecosystem(&app, "Stellar", "StellarAuth11111111111111111111111111")
        .unwrap();

    reg.record_contribution(&app, "DevA111111111111111111111111111111111111", 10, "Stellar-Wave-1")
        .unwrap();
    reg.record_contribution(&app, "DevB111111111111111111111111111111111111", 20, "Stellar-Wave-1")
        .unwrap();
    reg.record_contribution(&app, "DevA111111111111111111111111111111111111", 30, "Stellar-Wave-2")
        .unwrap();

    let wave1 = reg.get_contributors_by_badge("Stellar-Wave-1");
    assert_eq!(wave1.len(), 2);

    let wave2 = reg.get_contributors_by_badge("Stellar-Wave-2");
    assert_eq!(wave2.len(), 1);
}

#[test]
fn test_profile_not_found() {
    let reg = setup();
    let err = reg
        .get_profile("Nobody11111111111111111111111111111111111")
        .unwrap_err();
    assert_eq!(
        err,
        RegistryError::ProfileNotFound("Nobody11111111111111111111111111111111111".to_string())
    );
}

#[test]
fn test_unauthorized_ecosystem_whitelist() {
    let mut reg = setup();
    let err = reg
        .whitelist_ecosystem(
            "EvilApp1111111111111111111111111111111111",
            "Stellar",
            "StellarAuth11111111111111111111111111",
        )
        .unwrap_err();
    assert_eq!(err, RegistryError::Unauthorized);
}

#[test]
fn test_points_overflow_safe() {
    let mut reg = setup();
    let app = reg.authorized_wave_app.clone();
    let dev = "DevWallet1111111111111111111111111111111111";

    reg.whitelist_ecosystem(&app, "Stellar", "StellarAuth11111111111111111111111111")
        .unwrap();

    // Manually set profile to near max
    {
        let profile = reg.get_or_create_profile_for_test(dev);
        profile.total_points_earned = u64::MAX - 50;
    }

    let err = reg
        .record_contribution(&app, dev, 100, "Stellar-Wave-1")
        .unwrap_err();
    assert_eq!(err, RegistryError::PointsOverflow(100));
}

#[test]
fn test_validation_badge_tag_too_long() {
    let mut reg = setup();
    let app = reg.authorized_wave_app.clone();
    let dev = "DevWallet1111111111111111111111111111111111";

    let long_tag = "A".repeat(65);
    let err = reg
        .record_contribution(&app, dev, 10, &long_tag)
        .unwrap_err();
    assert!(matches!(err, RegistryError::BadgeTagTooLong(_, _)));
}

#[test]
fn test_validation_zero_points() {
    let mut reg = setup();
    let app = reg.authorized_wave_app.clone();
    let dev = "DevWallet1111111111111111111111111111111111";

    reg.whitelist_ecosystem(&app, "Stellar", "StellarAuth11111111111111111111111111")
        .unwrap();

    let err = reg.record_contribution(&app, dev, 0, "Stellar-Wave-1").unwrap_err();
    assert_eq!(err, RegistryError::InvalidPoints(0));
}
