const anchor = require("@coral-xyz/anchor");
const { SystemProgram, Keypair, PublicKey } = anchor.web3;
const { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount } = require("@solana/spl-token");
const { assert } = require("chai");

describe("guardian-vault", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.GuardianVault;

    let vaultStatePda, vaultBump;
    let mockSkrMint;
    let jusdiMint;
    let userSkrAccount;
    const treasury = Keypair.generate();

    it("Is initialized!", async () => {
        [vaultStatePda, vaultBump] = await PublicKey.findProgramAddress(
            [Buffer.from("vault_state")],
            program.programId
        );

        // 1. Create Mock SKR Mint with Vault Authority
        // Note: createMint from spl-token creates and initializes.
        // We need to pass authority.
        mockSkrMint = await createMint(
            provider.connection,
            provider.wallet.payer,
            vaultStatePda, // Mint Authority
            null,          // Freeze Authority
            6              // Decimals
        );
        console.log("Created Mock SKR Mint:", mockSkrMint.toBase58());

        // 2. Create jUSDi Mint with Vault Authority
        jusdiMint = await createMint(
            provider.connection,
            provider.wallet.payer,
            vaultStatePda,
            null,
            6
        );

        // 3. Initialize Program
        await program.methods.initialize(
            new anchor.BN(1000), // 10% Fee
            new anchor.BN(5000), // 50% LTV
            new anchor.BN(500),  // 5% Bonus
            new anchor.BN(2),    // 2 sec cooldown
            new anchor.BN(8000), // 80% Liq Threshold
            new anchor.BN(500)   // 5% Penalty
        )
            .accounts({
                authority: provider.wallet.publicKey,
                vaultState: vaultStatePda,
                labsTreasury: treasury.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        const state = await program.account.vaultState.fetch(vaultStatePda);
        assert.equal(state.harvestFeeBps, 1000);
    });

    it("Mints Mock SKR", async () => {
        // User ATA
        userSkrAccount = (await getOrCreateAssociatedTokenAccount(
            provider.connection,
            provider.wallet.payer,
            mockSkrMint,
            provider.wallet.publicKey
        )).address;

        await program.methods.mintMockSkr(new anchor.BN(1000000000)) // 1000 SKR
            .accounts({
                user: provider.wallet.publicKey,
                mockSkrMint: mockSkrMint,
                userSkrAccount: userSkrAccount,
                vaultState: vaultStatePda,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        const balance = await provider.connection.getTokenAccountBalance(userSkrAccount);
        assert.equal(balance.value.amount, "1000000000");
    });

    it("Deposits SKR and Borrows jUSDi", async () => {
        // 1. Get User Loan PDA
        const [userLoanPda] = await PublicKey.findProgramAddress(
            [Buffer.from("user_loan"), provider.wallet.publicKey.toBuffer()],
            program.programId
        );

        // 2. Init User Loan
        await program.methods.initUserLoan()
            .accounts({
                user: provider.wallet.publicKey,
                userLoan: userLoanPda,
                systemProgram: SystemProgram.programId,
            })
            .rpc();

        // 3. Get Vault Token Accounts (ATAs)
        const vaultSkrAccount = (await getOrCreateAssociatedTokenAccount(
            provider.connection,
            provider.wallet.payer,
            mockSkrMint,
            vaultStatePda,
            true // allowOwnerOffCurve
        )).address;

        const vaultJusdiAccount = (await getOrCreateAssociatedTokenAccount(
            provider.connection,
            provider.wallet.payer,
            jusdiMint,
            vaultStatePda,
            true // allowOwnerOffCurve
        )).address;

        // User jUSDi Account
        const userJusdiAccount = (await getOrCreateAssociatedTokenAccount(
            provider.connection,
            provider.wallet.payer,
            jusdiMint,
            provider.wallet.publicKey
        )).address;

        // 3. Deposit 100 SKR (100 * 10^6)
        // Borrow 500 jUSDi (500 * 10^6) -> $500.
        // Collateral Value = 100 * $10 = $1000.
        // LTV = 55% -> Max Borrow $550.
        // 500 < 550, should succeed.

        // Initialize Oracle Price Feed
        const priceFeed = Keypair.generate();
        console.log("Price Feed:", priceFeed.publicKey.toBase58());

        await program.methods.updateOracle(priceFeed.publicKey)
            .accounts({
                admin: provider.wallet.publicKey,
                vaultState: vaultStatePda,
            })
            .rpc();

        // 3. Deposit 100 SKR (100 * 10^6)
        // Borrow 500 jUSDi (500 * 10^6) -> $500.
        // Collateral Value = 100 * $10 = $1000.
        // LTV = 55% -> Max Borrow $550.
        await program.methods.depositSkrAndBorrow(
            new anchor.BN(100000000) // 100 SKR
        )
            .accounts({
                user: provider.wallet.publicKey,
                vaultState: vaultStatePda,
                userLoan: userLoanPda,
                userSkrAccount: userSkrAccount,
                vaultSkrAccount: vaultSkrAccount,
                vaultJusdiAccount: vaultJusdiAccount,
                userJusdiAccount: userJusdiAccount,
                jusdiMint: jusdiMint,
                mockSkrMint: mockSkrMint,
                skrPriceFeed: priceFeed.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
                associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
            })
            .rpc();

        const loan = await program.account.userLoan.fetch(userLoanPda);
        console.log("Loan Debt:", loan.debtAmount.toString());
        assert.equal(loan.debtAmount.toString(), "550000000");
    });
});
