/**
 * Oracle Configuration Script
 * Updates the vault's price feed to mainnet Pyth addresses
 * 
 * Usage: npx ts-node scripts/update_oracle_config.ts
 */

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";
import * as fs from "fs";

// Guardian Vault Program ID
const PROGRAM_ID = new PublicKey("DwuGR9qYkgYUPxR6jZSkAHdv23YPeqaAwxLAG593L1ar");

// =============================================================================
// PYTH ORACLE CONFIGURATION
// Reference: https://pyth.network/developers/price-feed-ids
// =============================================================================

interface OracleConfig {
    name: string;
    devnet: string;
    mainnet: string;
}

const ORACLE_FEEDS: Record<string, OracleConfig> = {
    // SKR/USD - Need to verify if Pyth has this feed
    SKR_USD: {
        name: "SKR/USD",
        devnet: "TBD_SKR_USD_DEVNET", // TODO: Get from Pyth
        mainnet: "TBD_SKR_USD_MAINNET", // TODO: Get from Pyth
    },
    // USDC/USD
    USDC_USD: {
        name: "USDC/USD",
        devnet: "5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7",
        mainnet: "Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD",
    },
    // SOL/USD (for reference)
    SOL_USD: {
        name: "SOL/USD",
        devnet: "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix",
        mainnet: "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG",
    },
};

// =============================================================================
// SCRIPT EXECUTION
// =============================================================================

async function main() {
    const isMainnet = process.env.CLUSTER === "mainnet";
    const walletPath = process.env.ANCHOR_WALLET ||
        `${process.env.HOME}/.config/solana/id.json`;

    const walletKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync(walletPath, "utf-8")))
    );

    const cluster = isMainnet
        ? "https://api.mainnet-beta.solana.com"
        : "https://api.devnet.solana.com";

    const connection = new Connection(cluster, "confirmed");

    console.log("🔧 Oracle Configuration Update");
    console.log(`   Cluster: ${isMainnet ? "MAINNET" : "Devnet"}`);
    console.log(`   Wallet:  ${walletKeypair.publicKey.toBase58()}`);
    console.log("");

    const provider = new anchor.AnchorProvider(
        connection,
        new anchor.Wallet(walletKeypair),
        { commitment: "confirmed" }
    );
    anchor.setProvider(provider);

    const idlPath = "./target/idl/guardian_vault.json";
    if (!fs.existsSync(idlPath)) {
        console.error("❌ IDL not found. Run 'anchor build' first.");
        process.exit(1);
    }
    const idl = JSON.parse(fs.readFileSync(idlPath, "utf-8"));
    const program = new Program(idl, PROGRAM_ID, provider);

    const [vaultStatePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault_state")],
        PROGRAM_ID
    );

    // Update SKR/USD oracle
    const skrFeedConfig = ORACLE_FEEDS.SKR_USD;
    const skrFeedAddress = isMainnet ? skrFeedConfig.mainnet : skrFeedConfig.devnet;

    if (skrFeedAddress.startsWith("TBD")) {
        console.log(`⚠️  SKR/USD feed not configured yet. Skipping.`);
        console.log("   Please update ORACLE_FEEDS.SKR_USD with actual Pyth addresses.");
        return;
    }

    try {
        const priceFeedPubkey = new PublicKey(skrFeedAddress);

        console.log(`📊 Updating Oracle: ${skrFeedConfig.name}`);
        console.log(`   New Feed: ${priceFeedPubkey.toBase58()}`);

        const tx = await program.methods
            .updateOracle(priceFeedPubkey)
            .accounts({
                authority: walletKeypair.publicKey,
                vaultState: vaultStatePda,
            })
            .signers([walletKeypair])
            .rpc();

        console.log(`   ✅ TX: ${tx}`);
    } catch (err: any) {
        console.error(`   ❌ Failed: ${err.message}`);
    }

    console.log("");
    console.log("🎉 Oracle configuration complete!");
}

main().catch(console.error);
