import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CiblGamble } from "../target/types/cibl_gamble";
import { expect } from "chai";

describe("cibl_gamble", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CiblGamble as Program<CiblGamble>;

  it("Creates a challenge and escrow funds!", async () => {
    const challengeKeypair = anchor.web3.Keypair.generate();
    const amount = new anchor.BN(1000000); // 1 SOL in lamports (example)

    await program.methods
      .createChallenge(amount)
      .accounts({
        challenge: challengeKeypair.publicKey,
        creator: provider.wallet.publicKey,
        escrowAccount: provider.wallet.publicKey, // For testing we use provider
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([challengeKeypair])
      .rpc();

    const account = await program.account.challenge.fetch(challengeKeypair.publicKey);
    expect(account.amount.toString()).to.equal(amount.toString());
  });
});
