import {
  PublicKey,
  Transaction,
  TransactionInstruction,
  Connection,
  sendAndConfirmTransaction,
  Keypair,
} from "@solana/web3.js";

/** Mirror of the on-chain ContributorProfile. */
export interface ContributorProfile {
  totalPointsEarned: number;
  sprintsCompleted: number;
  ecosystemBadges: string[];
}

/** Mirror of the on-chain Ecosystem. */
export interface Ecosystem {
  name: string;
  authorizedAddress: string;
  active: boolean;
}

/** Mirror of the on-chain WaveRegistry. */
export interface WaveRegistryData {
  registry: Array<{ address: string; profile: ContributorProfile }>;
  authorizedWaveApp: string;
  ecosystems: Ecosystem[];
}

export class WaveRegistrySDK {
  constructor(
    readonly connection: Connection,
    readonly programId: PublicKey,
    readonly registryAddress: PublicKey,
  ) {}

  // ── Instructions ──────────────────────────────────────────────────────────

  /** Build a `record_contribution` instruction. */
  recordContribution(
    authority: PublicKey,
    developer: PublicKey,
    points: number,
    waveTag: string,
  ): TransactionInstruction {
    const data = Buffer.alloc(8 + 32 + 8 + 4 + waveTag.length);
    // Anchor discriminator placeholder
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: this.registryAddress, isSigner: false, isWritable: true },
        { pubkey: authority, isSigner: true, isWritable: false },
      ],
      programId: this.programId,
      data,
    });
    return ix;
  }

  /** Build a `whitelist_ecosystem` instruction. */
  whitelistEcosystem(
    authority: PublicKey,
    name: string,
    authorizedAddress: PublicKey,
  ): TransactionInstruction {
    // stub — real implementation uses anchor Borsh serialization
    return new TransactionInstruction({
      keys: [
        { pubkey: this.registryAddress, isSigner: false, isWritable: true },
        { pubkey: authority, isSigner: true, isWritable: false },
      ],
      programId: this.programId,
      data: Buffer.alloc(8),
    });
  }

  // ── Accounts ──────────────────────────────────────────────────────────────

  /** Fetch and decode the registry account. */
  async fetchRegistry(): Promise<WaveRegistryData> {
    const accountInfo = await this.connection.getAccountInfo(this.registryAddress);
    if (!accountInfo) throw new Error("Registry account not found");
    // Stub — real impl uses @coral-xyz/anchor for Borsh deserialization
    return {
      registry: [],
      authorizedWaveApp: "",
      ecosystems: [],
    };
  }
}
