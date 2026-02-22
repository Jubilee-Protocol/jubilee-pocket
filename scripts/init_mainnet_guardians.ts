/**
 * Guardian Registry Initialization Script
 * Populates the Guardian list with Mainnet validator partners
 * 
 * Usage: npx ts-node scripts/init_mainnet_guardians.ts
 */

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";
import * as fs from "fs";

// Guardian Vault Program ID
const PROGRAM_ID = new PublicKey("DwuGR9qYkgYUPxR6jZSkAHdv23YPeqaAwxLAG593L1ar");

// =============================================================================
// MAINNET GUARDIAN CONFIGURATION
// Update these with actual partner validator pubkeys before mainnet launch
// =============================================================================

interface GuardianConfig {
    pubkey: string;
    name: string;
    commissionBps: number; // Max 700 (7%)
}

const MAINNET_GUARDIANS: GuardianConfig[] = [
    {
        pubkey: "STAKEHOUSE_VALIDATOR_PUBKEY_HERE", // TODO: Replace with actual
        name: "Stakehouse",
        commissionBps: 500, // 5%
    },
    {
        pubkey: "JITO_VALIDATOR_PUBKEY_HERE", // TODO: Replace with actual
        name: "Jito MEV",
        commissionBps: 600, // 6%
    },
    {
        pubkey: "HELIUS_VALIDATOR_PUBKEY_HERE", // TODO: Replace with actual
        name: "Helius",
        commissionBps: 500, // 5%
    },
];

// =============================================================================
// SCRIPT EXECUTION
// =============================================================================

async function main() {
    // Load wallet from default Solana config
    const walletPath = process.env.ANCHOR_WALLET ||
        `${process.env.HOME}/.config/solana/id.json`;

    const walletKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync(walletPath, "utf-8")))
    );

    // Connect to cluster (default: devnet for testing, change for mainnet)
    const cluster = process.env.ANCHOR_PROVIDER_URL || "https://api.devnet.solana.com";
    const connection = new Connection(cluster, "confirmed");

    console.log("🔧 Guardian Registry Initialization");
    console.log(`   Cluster: ${cluster}`);
    console.log(`   Wallet:  ${walletKeypair.publicKey.toBase58()}`);
    console.log(`   Program: ${PROGRAM_ID.toBase58()}`);
    console.log("");

    // Set up Anchor provider
    const provider = new anchor.AnchorProvider(
        connection,
        new anchor.Wallet(walletKeypair),
        { commitment: "confirmed" }
    );
    anchor.setProvider(provider);

    // Load program IDL (assumes it's been generated)
    const idlPath = "./target/idl/guardian_vault.json";
    if (!fs.existsSync(idlPath)) {
        console.error("❌ IDL not found. Run 'anchor build' first.");
        process.exit(1);
    }
    const idl = JSON.parse(fs.readFileSync(idlPath, "utf-8"));
    const program = new Program(idl, PROGRAM_ID, provider);

    // Derive PDAs
    const [vaultStatePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault_state")],
        PROGRAM_ID
    );
    const [guardianListPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("guardian_list")],
        PROGRAM_ID
    );

    console.log("📍 PDAs:");
    console.log(`   Vault State:   ${vaultStatePda.toBase58()}`);
    console.log(`   Guardian List: ${guardianListPda.toBase58()}`);
    console.log("");

    // Add each guardian
    for (const guardian of MAINNET_GUARDIANS) {
        // Skip placeholders
        if (guardian.pubkey.includes("_HERE")) {
            console.log(`⏭️  Skipping ${guardian.name} (placeholder pubkey)`);
            continue;
        }

        try {
            const guardianPubkey = new PublicKey(guardian.pubkey);

            console.log(`➕ Adding Guardian: ${guardian.name}`);
            console.log(`   Pubkey:     ${guardianPubkey.toBase58()}`);
            console.log(`   Commission: ${guardian.commissionBps / 100}%`);

            const tx = await program.methods
                .addGuardian(guardianPubkey, guardian.name, guardian.commissionBps)
                .accounts({
                    authority: walletKeypair.publicKey,
                    vaultState: vaultStatePda,
                    guardianList: guardianListPda,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([walletKeypair])
                .rpc();

            console.log(`   ✅ TX: ${tx}`);
        } catch (err: any) {
            if (err.message?.includes("GuardianAlreadyWhitelisted")) {
                console.log(`   ⚠️  Already exists, skipping.`);
            } else {
                console.error(`   ❌ Failed: ${err.message}`);
            }
        }
        console.log("");
    }

    console.log("🎉 Guardian initialization complete!");
}

main().catch(console.error);
