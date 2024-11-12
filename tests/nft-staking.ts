import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";
import {
  PublicKey,
  Keypair,
  SYSVAR_INSTRUCTIONS_PUBKEY,
} from "@solana/web3.js";

describe("nft-staking", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.NftStaking as Program<NftStaking>;
  it("create reward token", async () => {
    const reward_mint_metadata = Keypair.generate();
    console.log(
      "reward_mint_metadata",
      reward_mint_metadata.publicKey.toBase58()
    );

    console.log("payer", payer.publicKey.toBase58());
    const tx = await program.methods
      .createRewardToken()
      .accounts({
        admin: payer.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
