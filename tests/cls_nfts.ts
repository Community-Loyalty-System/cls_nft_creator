import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ClsNfts } from "../target/types/cls_nfts";

describe("cls_nfts", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ClsNfts as Program<ClsNfts>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
