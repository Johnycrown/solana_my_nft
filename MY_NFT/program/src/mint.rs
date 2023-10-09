use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	Account,
	AccountPDA,
	GemMetadata,
};


/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable, signer]` mint: [Mint] 
/// 2. `[writable]` gem: [GemMetadata] 
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 4. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 5. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 6. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 7. `[]` token_program: [AccountInfo] SPL Token program
/// 8. `[signer]` owner: [AccountInfo] The mint's minting authority.
/// 9. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
/// 10. `[]` csl_spl_assoc_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
///
/// Data:
/// - color: [String] 
/// - rarity: [String] 
/// - short_description: [String] 
pub fn mint(
	program_id: &Pubkey,
	for_initialize_mint_2: &[&AccountInfo],
	for_create: &[&AccountInfo],
	for_mint_to: &[&AccountInfo],
	for_set_authority: &[&AccountInfo],
	mint: &Account<spl_token::state::Mint>,
	gem: &mut AccountPDA<GemMetadata>,
	funding: &AccountInfo,
	assoc_token_account: &AccountInfo,
	wallet: &AccountInfo,
	owner: &AccountInfo,
	color: String,
	rarity: String,
	short_description: String,
) -> ProgramResult {
   
	
    if mint.mint_authority != *owner.key {
        return Err(ProgramError::InvalidArgument);
    }
    csl_spl_token::src::cpi::initialize_mint_2(
        for_initialize_mint_2,
        Default::default(),
        mint.owner,
        Some(mint.mint_authority),
    )?;

    csl_spl_assoc_token::src::cpi::create(
        for_create,
    )?;

    csl_spl_token::src::cpi::mint_to(
        for_mint_to,
        Default::default(),
    )?;

    csl_spl_token::src::cpi::set_authority(
        for_set_authority,
        Default::default(),
        Some(mint.mint_authority),
    )?;

    gem.color = color;
    gem.rarity = rarity;
    gem.short_description = short_description;

    Ok(())
}