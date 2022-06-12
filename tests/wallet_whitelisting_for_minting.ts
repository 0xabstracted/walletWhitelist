import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { WalletWhitelistingForMinting } from "../target/types/wallet_whitelisting_for_minting";

describe("wallet_whitelisting_for_minting", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.WalletWhitelistingForMinting as Program<WalletWhitelistingForMinting>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
