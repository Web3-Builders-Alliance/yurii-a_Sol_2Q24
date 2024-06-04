import * as anchor from "@coral-xyz/anchor";
import {Address, Program, Wallet} from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import BN from 'bn.js';
import NodeWallet from "@coral-xyz/anchor/dist/browser/src/nodewallet";


describe("vault", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Vault as Program<Vault>;
  let user = provider.wallet as NodeWallet;
  console.log('user pk: '+ user);

  const vaultStateAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("state"), user.publicKey.toBytes()], program.programId)[0];
  const vaultAccount = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault"), vaultStateAccount.toBytes()], program.programId)[0];

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize()
  //       .accountsPartial({
  //           user: user.publicKey,
  //           vault: vaultAccount,
  //           vaultState: vaultStateAccount,
  //           systemProgram: anchor.web3.SystemProgram.programId
  //           }
  //       )
  //       .rpc();
  //   console.log("Your transaction signature", tx);
  //
  //   const vaultState = await program.account.vaultState.fetch(vaultStateAccount);
  // });

  it("Is deposited!", async () => {
      // Add your test here.
      const tx = await program.methods.deposit(new BN(1e9))
          .accountsPartial({
                  user: user.publicKey,
                  vault: vaultAccount,
                  vaultState: vaultStateAccount,
              systemProgram: anchor.web3.SystemProgram.programId
          }
          )
          .rpc();
      console.log("Your transaction signature", tx);
      const vaultState = await program.account.vaultState.fetch(vaultStateAccount);
      console.log(
          `State Bump: ${vaultState.stateBump}, ` +
          `Vault Bump: ${vaultState.vaultBump}, `
          // `Freeze Until: ${vaultState.freezeUntil}`
      );
  });
});
