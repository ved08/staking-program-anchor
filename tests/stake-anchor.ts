import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakeAnchor } from "../target/types/stake_anchor";
import { expect } from "chai";

describe("stake-anchor", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.StakeAnchor as Program<StakeAnchor>;
  const stakingProgram = anchor.web3.Keypair.generate()
  const poolOwner = (program.provider as anchor.AnchorProvider).wallet

  
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize()
    .accounts({
      pool: stakingProgram.publicKey,
      authority: poolOwner.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([stakingProgram])
    .rpc();
    console.log("Your transaction signature", tx);
    const state = await program.account.pool.fetch(stakingProgram.publicKey)
    expect(state.authority.toString()).to.equal(poolOwner.publicKey.toString())
    expect(state.authority.toString()).to.equal(poolOwner.publicKey.toString())
    expect(state.userCount).to.equal(0)
  });

  it('is user created!', async() => {
    const userAccount = provider.wallet
    const [userPDA, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("user"), userAccount.publicKey.toBuffer()],
      program.programId
    )
    const tx = await program.methods.createUser()
    .accounts({
      user: userPDA,
      authority: userAccount.publicKey,
      pool: stakingProgram.publicKey
    })
    .rpc()

    console.log("tx: ", tx)

    const state = await program.account.pool.fetch(stakingProgram.publicKey)
    expect(state.userCount).to.equal(1)
    
  })

});
