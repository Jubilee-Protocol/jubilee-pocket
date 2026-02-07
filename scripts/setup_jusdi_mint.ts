/**
 * Setup jUSDi Mint Script
 * 1. Creates a new Mint for jUSDi
 * 2. Adds Metaplex Metadata (Title, Symbol, Icon)
 * 3. Transfers Mint Authority to the Guardian Vault PDA
 * 
 * Usage: npx ts-node scripts/setup_jusdi_mint.ts
 */

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
    createMint,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    setAuthority,
    AuthorityType,
    TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import {
    createCreateMetadataAccountV3Instruction,
    PROGRAM_ID as METADATA_PROGRAM_ID
} from "@metaplex-foundation/mpl-token-metadata";
import { PublicKey, Keypair, Connection, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import * as fs from "fs";

// Guardian Vault Program ID
const PROGRAM_ID = new PublicKey("wy7kkPnizRCbXvrG6fBkuat6q8AwbwTgnjxhZWcg3Si");

// Metadata Config
const JUSDI_METADATA = {
    name: "Jubilee USD Index",
    symbol: "jUSDi",
    uri: "https://jubileeprotocol.xyz/metadata/jusdi.json", // Host the updated jusdi.json here
};

async function main() {
    // 1. Setup Connection & Wallet
    const walletPath = process.env.ANCHOR_WALLET || `${process.env.HOME}/.config/solana/id.json`;
    const walletKeypair = Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync(walletPath, "utf-8")))
    );

    // Use Devnet for this Setup
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
    console.log("🔧 Setting up jUSDi Mint on Devnet");
    console.log(`   Wallet: ${walletKeypair.publicKey.toBase58()}`);

    // 2. Create Mint
    console.log("   Creating Mint...");
    const mintKeypair = Keypair.generate();
    const mintAddress = await createMint(
        connection,
        walletKeypair,
        walletKeypair.publicKey, // Initial Authority
        null, // Freeze Authority
        6, // Decimals
        mintKeypair
    );
    console.log(`   ✅ Mint Created: ${mintAddress.toBase58()}`);

    // 3. Add Metadata
    console.log("   Adding Metadata...");
    const [metadataPda] = PublicKey.findProgramAddressSync(
        [
            Buffer.from("metadata"),
            METADATA_PROGRAM_ID.toBuffer(),
            mintAddress.toBuffer(),
        ],
        METADATA_PROGRAM_ID
    );

    const metadataTx = new Transaction().add(
        createCreateMetadataAccountV3Instruction(
            {
                metadata: metadataPda,
                mint: mintAddress,
                mintAuthority: walletKeypair.publicKey,
                payer: walletKeypair.publicKey,
                updateAuthority: walletKeypair.publicKey,
            },
            {
                createMetadataAccountArgsV3: {
                    data: {
                        name: JUSDI_METADATA.name,
                        symbol: JUSDI_METADATA.symbol,
                        uri: JUSDI_METADATA.uri,
                        sellerFeeBasisPoints: 0,
                        creators: null,
                        collection: null,
                        uses: null,
                    },
                    isMutable: true,
                    collectionDetails: null,
                },
            }
        )
    );

    await sendAndConfirmTransaction(connection, metadataTx, [walletKeypair]);
    console.log(`   ✅ Metadata Initialized: ${metadataPda.toBase58()}`);

    // 4. Derive Vault PDA
    const [vaultStatePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault_state")],
        PROGRAM_ID
    );
    console.log(`   Vault PDA: ${vaultStatePda.toBase58()}`);

    // 5. Transfer Authority to Vault PDA
    console.log("   Transferring Mint Authority to Vault...");
    await setAuthority(
        connection,
        walletKeypair,
        mintAddress,
        walletKeypair.publicKey,
        AuthorityType.MintTokens,
        vaultStatePda
    );
    console.log("   ✅ Authority Transferred");

    console.log("");
    console.log("🎉 jUSDi Setup Complete!");
    console.log("----------------------------------------");
    console.log(`Mint Address: ${mintAddress.toBase58()}`);
    console.log(`Vault PDA:    ${vaultStatePda.toBase58()}`);
    console.log("----------------------------------------");
}

main().catch(console.error);
