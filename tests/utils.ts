import * as anchor from "@coral-xyz/anchor";
import { STATE } from "./constant"

/**
 * Derives the PDA (Program Derived Address) for a Marinade ticket account.
 *
 * @param pubkey - The burnMsolAuthority public key (user's authority)
 * @param ticketId - The ticket ID as an anchor.BN (u64)
 * @param programId - The program ID for the Anchor program
 * @returns The PDA for the new_ticket_account
 *
 * The seeds used are:
 *   - "ticket" (as Buffer)
 *   - STATE (program state account, as Buffer)
 *   - pubkey (burnMsolAuthority, as Buffer)
 *   - ticketId (as u64 LE Buffer)
 */
export const getTicketAccount = (
  pubkey: anchor.web3.PublicKey,
  ticketId: anchor.BN,
  programId: anchor.web3.PublicKey
): anchor.web3.PublicKey => {
  const [newTicketAccount, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("ticket"), // static seed
      STATE.toBuffer(),      // program state account
      pubkey.toBuffer(),     // burnMsolAuthority
      Buffer.from(ticketId.toArray("le", 8)) // ticketId as u64 LE
    ],
    programId
  );
  return newTicketAccount;
}