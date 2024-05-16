#![no_std]
use soroban_sdk::{ contract, contractimpl, contracttype, token, Address, Env };

#[contracttype]
pub enum StorageKey {
    Admin,
  /// Savings account address
  SavingsAccount,
}

#[contract]
pub struct AutomaticSavings;

#[contractimpl]
impl AutomaticSavings {
    pub fn set_admin(env: Env , new_admin: Address) {
    if let Some(admin) = env.storage().instance().get::<_, Address> (&StorageKey::Admin) {
    admin.require_auth();
    }
 env.storage().instance().set(&StorageKey::Admin, &new_admin);
 }

/// Return the admin address.
    pub fn admin(env: Env) -> Address {
        env.storage().instance().get::<_, Address>(&StorageKey::Admin).unwrap()
    }

// only admin of the contract can set the savings account here
    pub fn set_savings_account(env: Env, account: Address) {
// only contract admin can set savings account here
Self::admin(env.clone()).require_auth();
env.storage().instance().set(&StorageKey::SavingsAccount, &account);
}

// token transfer function to transfer tokens from user's account to another and it should automatically round up the transferred amount and send the difference to the savings account


pub fn transact(
env: Env,
from: Address,
token: Address,
amount: i128,
to: Address
) {
// Make sure `from` address authorized the deposit call with all the
// arguments.
from.require_auth();

// Calculate the remainder after rounding up to nearest dollar
let remainder = amount % 100;
let transfer_amount = amount - remainder;

let savings_account_option: Option<Address> = env.storage()
.instance()
.get(&StorageKey::SavingsAccount)
.unwrap();
if let Some(savings_account_address) = savings_account_option {
token::Client::new(&env, &token)
.transfer(&from, &savings_account_address, &remainder);
token::Client::new(&env, &token).transfer(&from, &to, &transfer_amount);
}
}
}

mod test;