mod deposit_data_tree;
mod get_attestation_participation;
mod get_attesting_indices;
mod initiate_validator_exit;
mod slash_validator;

pub mod altair;
pub mod base;
pub mod update_progressive_balances_cache;

pub use deposit_data_tree::DepositDataTree;
pub use get_attestation_participation::get_attestation_participation_flag_indices;
pub use get_attesting_indices::{
    attesting_indices_base, attesting_indices_electra, get_attesting_indices_from_state,
};
pub use initiate_validator_exit::initiate_validator_exit;
pub use slash_validator::slash_validator;

use safe_arith::SafeArith;
use types::{BeaconState, BeaconStateError, EthSpec};

/// Get a mutable reference to the balance of a single validator.
fn get_balance_mut<E: EthSpec>(state: &mut BeaconState<E>, validator_index: usize) -> Result<&mut u64, BeaconStateError> {
    state.balances_mut()
        .get_mut(validator_index)
        .ok_or(BeaconStateError::BalancesOutOfBounds(validator_index))
}

/// Increase the balance of a validator, erroring upon overflow, as per the spec.
pub fn increase_balance<E: EthSpec>(
    state: &mut BeaconState<E>,
    index: usize,
    delta: u64,
) -> Result<(), BeaconStateError> {
    increase_balance_directly(get_balance_mut(state, index)?, delta)
}

/// Decrease the balance of a validator, saturating upon overflow, as per the spec.
pub fn decrease_balance<E: EthSpec>(
    state: &mut BeaconState<E>,
    index: usize,
    delta: u64,
) -> Result<(), BeaconStateError> {
    decrease_balance_directly(get_balance_mut(state, index)?, delta)
}

/// Increase the balance of a validator, erroring upon overflow, as per the spec.
pub fn increase_balance_directly(balance: &mut u64, delta: u64) -> Result<(), BeaconStateError> {
    balance.safe_add_assign(delta)?;
    Ok(())
}

/// Decrease the balance of a validator, saturating upon overflow, as per the spec.
pub fn decrease_balance_directly(balance: &mut u64, delta: u64) -> Result<(), BeaconStateError> {
    *balance = balance.saturating_sub(delta);
    Ok(())
}
