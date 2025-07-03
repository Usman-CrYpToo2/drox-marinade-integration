import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DroxProject } from "../target/types/drox_project";
import {
   STATE, 
   LIQ_POOL_MSOL_LEG, 
   LIQ_POOL_MSOL_LEG_AUTHORITY,
   LIQ_POOL_SOL_LEG_PDA, 
   RESERVE_PDA, 
   MSOL_MINT_AUTHORITY,
   TREASURY_MSOL_ACCOUNT
 } from "./constant"
import fs from "fs";
import { homedir } from "os";
import {getTicketAccount} from "./utils";

// Load the local wallet keypair for signing transactions
const scrt = JSON.parse(fs.readFileSync(`${homedir()}/.config/solana/id.json`, "utf-8"));
const signer = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(scrt));
const pubkey = signer.publicKey;

// Main test suite for drox_project
// Each test exercises a different instruction in the Anchor program

describe("drox_project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Get the Anchor program client
  const program = anchor.workspace.droxProject as Program<DroxProject>;

 it("deposit sol!", async () => {
    //Test depositing SOL and receiving mSOL
    const tx = await program.methods.deposit(new anchor.BN(0.2 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
       state : STATE,
       transferFrom : pubkey,
       liqPoolMsolLeg: LIQ_POOL_MSOL_LEG,
       liqPoolMsolLegAuthority:LIQ_POOL_MSOL_LEG_AUTHORITY,
       liqPoolSolLegPda: LIQ_POOL_SOL_LEG_PDA,
       reservePda: RESERVE_PDA,
       msolMintAuthority: MSOL_MINT_AUTHORITY,
    }).signers([signer]).rpc();

     console.log("Your transaction signature", tx);
  });


  it("unstake sol through liquid unstake ", async() => {
      // Test liquid unstaking mSOL for SOL
      const tx = await program.methods.liquidUnstake(new anchor.BN(0.1 * anchor.web3.LAMPORTS_PER_SOL)).accounts({
          state : STATE,
          liqPoolMsolLeg: LIQ_POOL_MSOL_LEG,
          liqPoolSolLegPda: LIQ_POOL_SOL_LEG_PDA,
          getMsolFromAuthority: pubkey,
          treasuryMsolAccount: TREASURY_MSOL_ACCOUNT,
          transferSolTo: pubkey,
      }).signers([signer]).rpc();
      console.log("Your transaction signature", tx);
  })

  it("unstake sol through order unstake", async() => {
//       // Test delayed unstake (order ticket)
      const ticketId = new anchor.BN(1); // or whatever ticket id you want
      const newTicketAccount = getTicketAccount(pubkey, ticketId, program.programId);
      const tx = await program.methods.orderUnstake(new anchor.BN(0.04 * anchor.web3.LAMPORTS_PER_SOL), ticketId).accountsPartial({
          state: STATE,
          burnMsolAuthority: pubkey,
          payer: pubkey,
          newTicketAccount: newTicketAccount
      }).signers([signer]).rpc();
      console.log("Your transaction signature", tx);
   
  })

     it("claim sol", async() => {
        // Test claiming SOL from a completed unstake ticket
        const ticketId = new anchor.BN(1); // or whatever ticket id you want
        const newTicketAccount = getTicketAccount(pubkey, ticketId, program.programId);
        const tx = await program.methods.claim(ticketId).accountsPartial({
          state: STATE,
          reservePda: RESERVE_PDA,
         transferSolTo: pubkey,
         ticketAccount: newTicketAccount
        }).signers([signer]).rpc();
        console.log("Your transaction signature", tx);
     
     });

});
