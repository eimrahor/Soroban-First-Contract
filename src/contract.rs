#![no_std]

use soroban_sdk::{contract, contracttype, Address, Env, Vec, log, contractimpl};

#[contracttype]
pub enum DataKey {
    FrozenAccount(Address),
    Balance(Address),
    TransactionHistory(Address),
}

#[derive(Clone)]
#[contracttype]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct Account {
    pub address: Address,
}

#[contract]
pub struct AccountFreezeContract;

#[contractimpl]
impl AccountFreezeContract {
    pub fn freeze_account(env: Env, account: Address) {
        env.storage().persistent().set(&DataKey::FrozenAccount(account.clone()), &true);
        log!(&env,"Freeze {}",account);
    }

    pub fn unfreeze_account(env: Env, account: Address) {
        env.storage().persistent().remove(&DataKey::FrozenAccount(account.clone()));
        log!(&env,"Unfreeze {}",account);
    }

    pub fn is_account_frozen(env: Env, account: Address) -> bool {
        env.storage().persistent().get(&DataKey::FrozenAccount(account)).unwrap_or(false)
    }


    pub fn set_balance(env: Env, account: Address, amount: i128) {
        env.storage().persistent().set(&DataKey::Balance(account.clone()), &amount);
        log!(&env,"{} has {} balance", account, amount);
    }

    pub fn get_balance(env: Env, account: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(account)).unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {

        if from == to || amount <= 0 {
            panic!("Invalid submission!");
        }

        if Self::is_account_frozen(env.clone(), from.clone()) {
            panic!("Sender account is frozen");
        }

        if Self::is_account_frozen(env.clone(), to.clone()) {
            panic!("Recipient account is frozen");
        }

        let from_balance = Self::get_balance(env.clone(), from.clone());
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance = Self::get_balance(env.clone(), to.clone());

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        let transaction = Transaction {
            from: from.clone(),
            to: to.clone(),
            amount,
            timestamp: env.ledger().timestamp(),
        };

        Self::add_to_history(env.clone(), from.clone(), transaction.clone());
        Self::add_to_history(env.clone(), to.clone(), transaction);
        log!(&env,"Transfer is succeeded from {} to {}", from, to);
    }

    fn add_to_history(env: Env, account: Address, transaction: Transaction) {
        let mut history: Vec<Transaction> = env.storage().persistent().get(&DataKey::TransactionHistory(account.clone())).unwrap_or(Vec::new(&env));
        history.push_back(transaction);
        env.storage().persistent().set(&DataKey::TransactionHistory(account), &history);
    }

    pub fn get_transaction_history(env: Env, account: Address) -> Vec<Transaction> {
        env.storage().persistent().get(&DataKey::TransactionHistory(account)).unwrap_or(Vec::new(&env))
    }
}