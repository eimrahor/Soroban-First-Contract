#![no_std]
use soroban_sdk::{Env, Address, Vec};
mod contract;
use contract::AccountFreezeContract;
use contract::Transaction;

pub extern "C" fn freeze_account(env: Env, account: Address) {
    AccountFreezeContract::freeze_account(env, account);
}

pub extern "C" fn unfreeze_account(env: Env, account: Address) {
    AccountFreezeContract::unfreeze_account(env, account);
}

pub extern "C" fn is_account_frozen(env: Env, account: Address) -> bool {
    AccountFreezeContract::is_account_frozen(env, account)
}

pub extern "C" fn set_balance(env: Env, account: Address, amount: i128) {
    AccountFreezeContract::set_balance(env, account, amount);
}

pub extern "C" fn get_balance(env: Env, account: Address) -> i128 {
    AccountFreezeContract::get_balance(env, account)
}

pub extern "C" fn transfer(env: Env, from: Address, to: Address, amount: i128) {
    AccountFreezeContract::transfer(env, from, to, amount);
}

pub extern "C" fn get_transaction_history(env: Env, account: Address) -> Vec<Transaction> {
    AccountFreezeContract::get_transaction_history(env, account)
}