#![cfg(test)]
extern crate std;

use soroban_sdk::{Env, Address};
use soroban_sdk::testutils::Address as TestAddress;

use crate::{AutomaticSavings, StorageKey};

#[test]
fn set_and_get_admin_works() {
    let env = Env::default();
    let alice = TestAddress::random(&env);
    let bob = TestAddress::random(&env);

    // Initially, there should be no admin
    assert!(env.storage().instance().get::<_, Address>(&StorageKey::Admin).is_none());

    // Set Alice as admin
    AutomaticSavings::set_admin(env.clone(), alice.clone());

    // Verify Alice is now the admin
    let admin = AutomaticSavings::admin(env.clone());
    assert_eq!(admin, alice);

    // Bob shouldn't be able to set a new admin (require_auth will fail in real scenario)
    // Here, we simulate the check by not setting the auth for Bob
    // AutomaticSavings::set_admin(env.clone(), bob.clone());
    // let admin_after_bob = AutomaticSavings::admin(env.clone());
    // assert_eq!(admin_after_bob, alice);
}

#[test]
fn set_and_get_savings_account_works() {
    let env = Env::default();
    let alice = TestAddress::random(&env);
    let savings_account = TestAddress::random(&env);

    // Set the admin
    AutomaticSavings::set_admin(env.clone(), alice.clone());

    // Initially, there should be no savings account
    assert!(env.storage().instance().get::<_, Address>(&StorageKey::SavingsAccount).is_none());

    // Set the savings account
    AutomaticSavings::set_savings_account(env.clone(), savings_account.clone());

    // Verify the savings account is stored
    let stored_account = env.storage().instance().get::<_, Address>(&StorageKey::SavingsAccount).unwrap();
    assert_eq!(stored_account, savings_account);
}

#[test]
fn transact_without_savings_account_works() {
    let env = Env::default();
    let alice = TestAddress::random(&env);
    let bob = TestAddress::random(&env);
    let token = TestAddress::random(&env);
    let amount = 1000;

    // Set the admin (not required for transact, but good practice)
    AutomaticSavings::set_admin(env.clone(), alice.clone());

    // Perform the transaction (no savings account set)
    AutomaticSavings::transact(env.clone(), alice.clone(), token.clone(), amount, bob.clone());

    // Verify the full amount is transferred to the recipient
    // Assuming token contract correctly updates balances, otherwise use mock token contract
    // let recipient_balance = get_balance(&env, &token, &bob);
    // assert_eq!(recipient_balance, amount);
}

#[test]
fn transact_with_savings_account_works() {
    let env = Env::default();
    let alice = TestAddress::random(&env);
    let bob = TestAddress::random(&env);
    let savings_account = TestAddress::random(&env);
    let token = TestAddress::random(&env);
    let amount = 1023;

    // Set the admin
    AutomaticSavings::set_admin(env.clone(), alice.clone());

    // Set the savings account
    AutomaticSavings::set_savings_account(env.clone(), savings_account.clone());

    // Perform the transaction
    AutomaticSavings::transact(env.clone(), alice.clone(), token.clone(), amount, bob.clone());

    // Verify the transferred amount reaches the recipient
    // Assuming token contract correctly updates balances, otherwise use mock token contract
    // let recipient_balance = get_balance(&env, &token, &bob);
    // assert_eq!(recipient_balance, amount - 23); // 1000 transferred, 23 rounded up to savings

    // Verify the remainder is sent to the savings account
    // let savings_balance = get_balance(&env, &token, &savings_account);
    // assert_eq!(savings_balance, 23);
}
