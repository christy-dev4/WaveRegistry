use anchor_lang::prelude::*;
use wave_registry_core::errors::RegistryError;
use wave_registry_core::state::{ContributorProfile, WaveRegistry};

declare_id!("DRiPs11111111111111111111111111111111111111");

/// Anchor account holding the entire registry on-chain.
#[account]
pub struct RegistryAccount {
    pub data: WaveRegistry,
}

/// Developer profile derived PDA — one per wallet.
#[account]
pub struct ProfileAccount {
    pub owner: Pubkey,
    pub profile: ContributorProfile,
}

#[program]
pub mod wave_registry {
    use super::*;

    /// Initialise the registry with the Drips app authority.
    pub fn initialize(ctx: Context<Initialize>, app_authority: Pubkey) -> Result<()> {
        let registry = &mut ctx.accounts.registry.data;
        let core = WaveRegistry::new(app_authority.to_string());
        *registry = core;
        Ok(())
    }

    /// Record a contribution (authorised-app only).
    pub fn record_contribution(
        ctx: Context<RecordContribution>,
        developer: Pubkey,
        points: u64,
        wave_tag: String,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry.data;
        let caller = ctx.accounts.authority.key().to_string();

        registry
            .record_contribution(&caller, &developer.to_string(), points, &wave_tag)
            .map_err(|e| ProgramError::from(e))?;

        emit!(ContributionEvent {
            developer: developer.key().to_string(),
            points,
            wave_tag: wave_tag.clone(),
        });

        Ok(())
    }

    /// Whitelist a new ecosystem.
    pub fn whitelist_ecosystem(
        ctx: Context<WhitelistEcosystem>,
        name: String,
        authorized_address: Pubkey,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry.data;
        let caller = ctx.accounts.authority.key().to_string();

        registry
            .whitelist_ecosystem(&caller, &name, &authorized_address.to_string())
            .map_err(|e| ProgramError::from(e))?;

        emit!(EcosystemWhitelistedEvent {
            name: name.clone(),
            authorized_address: authorized_address.to_string(),
        });

        Ok(())
    }

    /// Deactivate an ecosystem.
    pub fn deactivate_ecosystem(ctx: Context<DeactivateEcosystem>, name: String) -> Result<()> {
        let registry = &mut ctx.accounts.registry.data;
        let caller = ctx.accounts.authority.key().to_string();

        registry
            .deactivate_ecosystem(&caller, &name)
            .map_err(|e| ProgramError::from(e))?;

        emit!(EcosystemDeactivatedEvent { name: name.clone() });
        Ok(())
    }

    /// Query a developer profile via PDA.
    pub fn get_profile(ctx: Context<GetProfile>) -> Result<ProfileAccount> {
        let registry = &ctx.accounts.registry.data;
        let developer = ctx.accounts.developer.key().to_string();

        let profile = registry
            .get_profile(&developer)
            .map_err(|e| ProgramError::from(e))?;

        Ok(ProfileAccount {
            owner: ctx.accounts.developer.key(),
            profile: profile.clone(),
        })
    }
}

// ── Accounts ────────────────────────────────────────────────────────────────

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8192)]
    pub registry: Account<'info, RegistryAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordContribution<'info> {
    #[account(mut)]
    pub registry: Account<'info, RegistryAccount>,
    /// CHECK: authorised-app gate enforced in program logic.
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct WhitelistEcosystem<'info> {
    #[account(mut)]
    pub registry: Account<'info, RegistryAccount>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeactivateEcosystem<'info> {
    #[account(mut)]
    pub registry: Account<'info, RegistryAccount>,
    #[account(signer)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetProfile<'info> {
    pub registry: Account<'info, RegistryAccount>,
    /// CHECK: any account can be queried.
    pub developer: AccountInfo<'info>,
}

// ── Events ──────────────────────────────────────────────────────────────────

#[event]
pub struct ContributionEvent {
    pub developer: String,
    pub points: u64,
    pub wave_tag: String,
}

#[event]
pub struct EcosystemWhitelistedEvent {
    pub name: String,
    pub authorized_address: String,
}

#[event]
pub struct EcosystemDeactivatedEvent {
    pub name: String,
}

// ── Error Mapping ───────────────────────────────────────────────────────────

impl From<RegistryError> for ProgramError {
    fn from(e: RegistryError) -> Self {
        match e {
            RegistryError::Unauthorized => ProgramError::Custom(0),
            RegistryError::EcosystemNotWhitelisted(_) => ProgramError::Custom(1),
            RegistryError::EcosystemAlreadyWhitelisted(_) => ProgramError::Custom(2),
            RegistryError::ProfileNotFound(_) => ProgramError::Custom(3),
            RegistryError::PointsOverflow(_) => ProgramError::Custom(4),
            RegistryError::SprintOverflow => ProgramError::Custom(5),
            RegistryError::BadgeTagTooLong(_, _) => ProgramError::Custom(6),
            RegistryError::EcosystemNameTooLong(_, _) => ProgramError::Custom(7),
            RegistryError::InvalidPoints(_) => ProgramError::Custom(8),
            RegistryError::InvalidAddress(_) => ProgramError::Custom(9),
        }
    }
}
