import { PublicKey } from "@solana/web3.js";

// Marinade state account (program state)
export const STATE = new PublicKey("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC");
// Marinade mSOL mint address
export const MSOL_MINT = new PublicKey("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");
// Marinade liquidity pool SOL leg PDA
export const LIQ_POOL_SOL_LEG_PDA = new PublicKey("UefNb6z6yvArqe4cJHTXCqStRsKmWhGxnZzuHbikP5Q");
// Marinade liquidity pool mSOL leg
export const LIQ_POOL_MSOL_LEG = new PublicKey("7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE");
// Marinade liquidity pool mSOL leg authority
export const LIQ_POOL_MSOL_LEG_AUTHORITY = new PublicKey("EyaSjUtSgo9aRD1f8LWXwdvkpDTmXAW54yoSHZRF14WL");
// Marinade reserve PDA (SOL reserve)
export const RESERVE_PDA = new PublicKey("Du3Ysj1wKbxPKkuPPnvzQLQh8oMSVifs3jGZjJWXFmHN");
// Marinade mSOL mint authority
export const MSOL_MINT_AUTHORITY = new PublicKey("3JLPCS1qM2zRw3Dp6V4hZnYHd4toMNPkNesXdX9tg6KM");
// Marinade treasury mSOL account (for liquid unstake)
export const TREASURY_MSOL_ACCOUNT = new PublicKey("8ZUcztoAEhpAeC2ixWewJKQJsSUGYSGPVAjkhDJYf5Gd");