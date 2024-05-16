#![no_std]
use soroban_sdk::{ test, contract, Env , Address};
use soroban_sdk::testing::AccountResource;

const ALICE: Address = AccountResource::new([0x01; 32]);
const BOB: Address = AccountResource::new([0x02; 32]);
const SAVINGS_ACCOUNT: Address = AccountResource::new([0x03; 32]);

#[contract]
pub mod AutomaticSavingsTest {
  #[test]
  fn set_and_get_admin_works() {
    // Create a new test environment
    let mut env = Env::default();

    // Initially, there should be no admin
    assert!(AutomaticSavings::admin(&env) == AccountResource::DEFAULT);

    // Set alice as admin
    AutomaticSavings::set_admin(&env, ALICE);

    // Verify alice is now the admin
    assert!(AutomaticSavings::admin(&env) == ALICE);

    // Bob shouldn't be able to set a new admin
    let result = AutomaticSavings::set_admin(&env, BOB);
    assert!(!result.is_ok());
  }

  #[test]
  fn set_and_get_savings_account_works() {
    // Create a new test environment
    let mut env = Env::default();

    // Set the admin
    AutomaticSavings::set_admin(&env, ALICE);

    // Initially, there should be no savings account
    assert!(env.storage().instance().get::<_, Address>(&StorageKey::SavingsAccount).is_none());

    // Set the savings account
    AutomaticSavings::set_savings_account(&env, SAVINGS_ACCOUNT);

    // Verify the savings account is stored
    let stored_account = env.storage().instance().get::<_, Address>(&StorageKey::SavingsAccount).unwrap();
    assert!(stored_account == SAVINGS_ACCOUNT);
  }

  #[test]
  fn transact_without_savings_account_works() {
    // Create a new test environment
    let mut env = Env::default();
    let token = env.create_account(); // Simulate a token address
    let amount = 1000;

    // Set the admin (not required for transact, but good practice)
    AutomaticSavings::set_admin(&env, ALICE);

    // Perform the transaction (no savings account set)
    AutomaticSavings::transact(&env, ALICE, token, amount, BOB);

    // Verify the full amount is transferred to the recipient
    let recipient_balance = env.accounts().get(&BOB).unwrap().balance;
    assert!(recipient_balance == amount);
  }

  #[test]
  fn transact_with_savings_account_works() {
    // Create a new test environment
    let mut env = Env::default();
    let token = env.create_account(); // Simulate a token address
    let amount = 1023;

    // Set the admin
    AutomaticSavings::set_admin(&env, ALICE);

    // Set the savings account
    AutomaticSavings::set_savings_account(&env, SAVINGS_ACCOUNT);

    // Perform the transaction
    AutomaticSavings::transact(&env, ALICE, token, amount, BOB);

    // Verify the transferred amount reaches the recipient
    let recipient_balance = env.accounts().get(&BOB).unwrap().balance;
    assert!(recipient_balance == amount - 23); // 1000 transferred, 23 rounded up to savings

    // Verify the remainder is sent to the savings account
    let savings_balance = env.accounts().get(&SAVINGS_ACCOUNT).unwrap().balance;
    assert!(savings_balance == 23);
  }
}
