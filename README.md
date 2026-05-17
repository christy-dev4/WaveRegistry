# WaveRegistry — On-Chain Contributor Reputation

A tamper-proof on-chain registry that tracks developer contributions across open-source ecosystems. Each completed Wave/Sprint mints a "Proof of Contribution" badge and accumulates verifiable reputation points, giving developers a portable on-chain resume.

## Architecture

```
┌──────────────────────────────────────────────┐
│                 Protocol Layer               │
│  (queried by other protocols for gating)     │
└──────────────┬───────────────────────────────┘
               │ get_profile / get_high_tier
┌──────────────▼───────────────────────────────┐
│           wave-registry-core                  │
│  Pure Rust library — business logic + state   │
│  - ContributorProfile, Ecosystem, WaveRegistry│
│  - record_contribution, whitelist, query      │
│  - Input validation, typed errors, events     │
└──────┬───────────────────────────┬────────────┘
       │                           │
       ▼                           ▼
┌──────────────┐    ┌──────────────────────────┐
│ Solana       │    │ TypeScript SDK           │
│ Anchor       │    │ (sdk/)                   │
│ Program      │    │ dApp / indexer client    │
└──────────────┘    └──────────────────────────┘
```

## Repository Structure

```
WaveRegistry/
├── core/                          # Pure Rust library (no blockchain deps)
│   ├── src/
│   │   ├── lib.rs                 # Crate entry point
│   │   ├── state.rs               # ContributorProfile, Ecosystem, WaveRegistry
│   │   ├── registry.rs            # Core business logic
│   │   ├── errors.rs              # 10 typed error variants
│   │   ├── events.rs              # Domain events
│   │   └── validation.rs          # Input validation (badge, address, points)
│   └── Cargo.toml
├── program/                       # Solana Anchor program (on-chain adapter)
│   ├── src/lib.rs                 # 5 instructions: init, record, whitelist,
│   │                              #   deactivate, get_profile
│   └── Cargo.toml
├── sdk/                           # TypeScript client SDK
│   ├── src/index.ts
│   ├── package.json
│   └── tsconfig.json
├── tests/
│   └── integration.rs             # Integration tests for core library
├── .github/workflows/ci.yml       # GitHub Actions: fmt → clippy → test → audit
├── Anchor.toml                    # Solana Anchor deployment config
├── Cargo.toml                     # Workspace root
├── deny.toml                      # cargo-deny security audit config
├── rust-toolchain.toml            # Pinned toolchain + BPF target
└── LICENSE
```

## Getting Started

### Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | stable (≥1.75) | Core library compilation |
| Solana CLI | 2.2 | Localnet testing |
| Anchor CLI | 0.30.1 | Program deployment |
| Node.js | ≥18 | SDK development |

### Build & Test (Core Library)

The core library is pure Rust with zero blockchain dependencies.

```bash
# Build
cargo build -p wave-registry-core

# Run all tests
cargo test -p wave-registry-core

# Run with full output
cargo test -p wave-registry-core -- --nocapture
```

### Build Full Workspace (with Solana program)

```bash
# Install Solana + Anchor first, then:
cargo build --workspace
cargo test --all-features
```

### Deploy to Solana

```bash
anchor build
anchor deploy
anchor test
```

## Core API

### `WaveRegistry`

| Method | Description | Auth |
|--------|-------------|------|
| `new(app_address)` | Creates an empty registry | — |
| `record_contribution(caller, developer, points, wave_tag)` | Records points + badge after a sprint | `caller == authorized_wave_app` |
| `whitelist_ecosystem(caller, name, authorized_address)` | Registers a new ecosystem | `caller == authorized_wave_app` |
| `deactivate_ecosystem(caller, name)` | Soft-removes an ecosystem | `caller == authorized_wave_app` |
| `get_profile(developer)` | Returns a developer's profile | Public |
| `get_high_tier_contributors(min_points)` | Filters by point threshold | Public |
| `get_contributors_by_badge(badge)` | Filters by badge name | Public |
| `total_developers()` | Total registered developers | Public |

### Error Types

| Error | Code | Description |
|-------|------|-------------|
| `Unauthorized` | 0x0 | Caller is not the authorized app |
| `EcosystemNotWhitelisted` | 0x1 | Ecosystem is unknown or inactive |
| `EcosystemAlreadyWhitelisted` | 0x2 | Duplicate ecosystem registration |
| `ProfileNotFound` | 0x3 | Developer has no profile |
| `PointsOverflow` | 0x4 | u64 overflow on point accumulation |
| `SprintOverflow` | 0x5 | u32 overflow on sprint count |
| `BadgeTagTooLong` | 0x6 | Badge exceeds 64 chars |
| `EcosystemNameTooLong` | 0x7 | Ecosystem name exceeds 64 chars |
| `InvalidPoints` | 0x8 | Points must be > 0 |
| `InvalidAddress` | 0x9 | Address fails basic validation |

## Usage Examples

### Rust (Core Library)

```rust
use wave_registry_core::state::WaveRegistry;

let mut registry = WaveRegistry::new("DripsApp...".to_string());
let app = registry.authorized_wave_app.clone();

// 1. Whitelist an ecosystem
registry.whitelist_ecosystem(&app, "Stellar", "StellarAuth...").unwrap();

// 2. Record a contribution
registry.record_contribution(&app, "DevWallet...", 150, "Stellar-Wave-1").unwrap();

// 3. Query high-tier contributors
let top = registry.get_high_tier_contributors(100);
assert_eq!(top.len(), 1);
```

### TypeScript SDK

```typescript
import { Connection, PublicKey } from "@solana/web3.js";
import { WaveRegistrySDK } from "@drips-network/wave-registry-sdk";

const sdk = new WaveRegistrySDK(
  new Connection("https://api.mainnet-beta.solana.com"),
  new PublicKey("DRiPs11111111111111111111111111111111111111"),
  new PublicKey("REGISTRY_ACCOUNT_ADDRESS"),
);

const data = await sdk.fetchRegistry();
console.log(data.ecosystems);
```

## CI Pipeline

The `.github/workflows/ci.yml` runs on every push/PR to `main`:

1. **Lint** — `cargo fmt --check` + `cargo clippy -- -D warnings`
2. **Test** — `cargo test --all-features`
3. **Audit** — `cargo deny check` (vulnerabilities, license compliance, duplicate deps)

## Security

- Checked arithmetic (`checked_add`) prevents numeric overflow
- All mutation requires `authorized_wave_app` signature
- Ecosystem whitelisting prevents unapproved badge minting
- Input validation rejects malformed addresses, tags, and ecosystem names
- `cargo deny` enforces dependency audit in CI

## License

MIT
