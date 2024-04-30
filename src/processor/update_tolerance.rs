use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    error::OreError, instruction::UpdateToleranceArgs, loaders::*, state::Config,
    utils::AccountDeserialize,
};

pub fn process_update_tolerance<'a, 'info>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = UpdateToleranceArgs::try_from_bytes(data)?;

    // Load accounts
    let [signer, config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_config(config_info, true)?;

    // Validate signer is admin
    let mut config_data = config_info.data.borrow_mut();
    let config = Config::try_from_bytes_mut(&mut config_data)?;
    if config.admin.ne(&signer.key) {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Overflow checks
    if args.tolerance_liveness.gt(&(i64::MAX as u64)) {
        return Err(OreError::ToleranceInvalid.into());
    }
    if args.tolerance_spam.gt(&(i64::MAX as u64)) {
        return Err(OreError::ToleranceInvalid.into());
    }

    // Update tolerances
    config.tolerance_liveness = args.tolerance_liveness as i64;
    config.tolerance_spam = args.tolerance_spam as i64;

    Ok(())
}
