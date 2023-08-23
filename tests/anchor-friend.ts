import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {AnchorFriend} from "../target/types/anchor_friend";
import BN from "bn.js";

describe("anchor-friend", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.AnchorFriend as Program<AnchorFriend>;

    const admin = anchor.web3.Keypair.generate();
    const user1 = anchor.web3.Keypair.generate();

    const conn = anchor.getProvider().connection;

    it("Is initialized!", async () => {

        await conn.requestAirdrop(admin.publicKey, 1e9).then(sig => conn.confirmTransaction(sig));
        await conn.requestAirdrop(user1.publicKey, 1e9).then(sig => conn.confirmTransaction(sig));
        await conn.requestAirdrop(program.provider.publicKey, 1e9).then(sig => conn.confirmTransaction(sig));

        const [state, stateBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(anchor.utils.bytes.utf8.encode("state"))],
            program.programId
        )
        const [vault, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(anchor.utils.bytes.utf8.encode("vault"))],
            program.programId
        )


        const tx = await program.methods
            .initAdmin(stateBump).accounts({
                state: state,
                signer: admin.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY
            }).signers([admin]).rpc();

        console.log("Your transaction signature", tx);


        const [ownerShareState, ownerStateBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(anchor.utils.bytes.utf8.encode("owner_share_state")), user1.publicKey.toBuffer()],
            program.programId
        )

        const [holding, holdingBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(anchor.utils.bytes.utf8.encode("holding")), user1.publicKey.toBuffer(), user1.publicKey.toBuffer()],
            program.programId
        )

        const [newHolding, newHoldingBump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from(anchor.utils.bytes.utf8.encode("holding")), user1.publicKey.toBuffer(), admin.publicKey.toBuffer()],
            program.programId
        )


        await program.methods
            .initOwnerShareState(ownerStateBump, stateBump).accounts({
                state: state,
                ownerShareState: ownerShareState,
                ownerPubkey: user1.publicKey,
                socialMediaHandle: user1.publicKey,
                signer: admin.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY
            }).signers([admin]).rpc().then(sig => conn.confirmTransaction(sig)).catch(console.log);

        await program.methods
            .initHolding(holdingBump, stateBump).accounts({
                state: state,
                ownerShareState: ownerShareState,
                holding: holding,
                ownerPubkey: user1.publicKey,
                signer: admin.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY
            }).signers([admin]).rpc().then(sig => conn.confirmTransaction(sig)).catch(console.log);

        // @ts-ignore
        await program.methods
            .buyHolding(newHoldingBump, vaultBump, stateBump, 1, new anchor.BN(10)).accounts({
                ownerShareState: ownerShareState,
                holding: newHolding,
                ownerPubkey: user1.publicKey,
                vault: vault,
                signer: admin.publicKey,
                state: state,
                admin: admin.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY
            }).signers([admin]).rpc().then(sig => conn.confirmTransaction(sig)).then(console.log).catch(console.log);
        await conn.getBalance(vault).then(console.log)
        await program.methods
            .buyHolding(newHoldingBump, vaultBump, stateBump, 11, new anchor.BN(10)).accounts({
                ownerShareState: ownerShareState,
                holding: newHolding,
                ownerPubkey: user1.publicKey,
                vault: vault,
                signer: admin.publicKey,
                state: state,
                admin: admin.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY
            }).signers([admin]).rpc().then(sig => conn.confirmTransaction(sig)).then(console.log).catch(console.log);
        await conn.getBalance(vault).then(console.log)
        await program.methods
            .sellHolding(newHoldingBump, vaultBump, stateBump, 21, new anchor.BN(1)).accounts({
                ownerShareState: ownerShareState,
                holding: newHolding,
                ownerPubkey: user1.publicKey,
                vault: vault,
                signer: admin.publicKey,
                state: state,
                admin: admin.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY
            }).signers([admin]).rpc().then(sig => conn.confirmTransaction(sig)).then(console.log).catch(console.log);

        await program.account.ownerShareState.fetch(ownerShareState).then(console.log)
        await program.account.holding.fetch(holding).then(console.log)
        await conn.getBalance(vault).then(console.log)

    });
});
