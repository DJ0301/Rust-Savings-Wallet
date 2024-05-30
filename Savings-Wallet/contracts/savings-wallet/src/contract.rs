#![no_std]
use soroban_sdk::{ contract, contractimpl, contracttype, token, Address, Env };

#[contracttype]
pub enum StorageKey {
    /// Admin. Value is an Address.
    Admin,
}

#[contract]
pub struct AutomaticCharityDonations;

#[contractimpl]
impl AutomaticCharityDonations {
    pub fn set_admin(env: Env, new_admin: Address) {
        if let Some(admin) = env.storage().instance().get::<_, Address>(&StorageKey::Admin) {
            admin.require_auth();
        }
        env.storage().instance().set(&StorageKey::Admin, &new_admin);
    }

    /// Return the admin address.
    pub fn admin(env: Env) -> Address {
        env.storage().instance().get::<_, Address>(&StorageKey::Admin).unwrap()
    }

    // only admin of the contract can add charity account here
    pub fn add_charity_account(env: Env, id: u32, account: Address) {
        // only contract admin can add charity account here
        Self::admin(env.clone()).require_auth();
        env.storage().instance().set(&id, &account);
    }

    pub fn remove_charity_account(env: Env, id: u32) {
        // only contract admin can remove charity account here
        Self::admin(env.clone()).require_auth();
        env.storage().instance().remove(&id);
    }

    // token transfer function to transfer tokens from user's account to another and it should automatically donate 2% of tokens to the charity contract
    pub fn transact(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        to: Address,
        charity_account_id: u32
    ) {
        // Make sure `from` address authorized the deposit call with all the
        // arguments.
        from.require_auth();

        let percent = 2;
        let donation_amount = (amount * percent) / 100;
        let transfer_amount = amount - donation_amount;

        let charity_account_address_option: Option<Address> = env
            .storage()
            .instance()
            .get(&charity_account_id)
            .unwrap();
        if let Some(charity_account_address) = charity_account_address_option {
            // Address found, proceed with the transfer
            token::Client
                ::new(&env, &token)
                .transfer(&from, &charity_account_address, &donation_amount);
            token::Client::new(&env, &token).transfer(&from, &to, &transfer_amount);

            // Update donation amounts
            Self::set_user_donation_amount(env.clone(), from, &token, donation_amount);
            Self::set_charity_donation_amount(
                env.clone(),
                charity_account_id,
                &token,
                donation_amount
            );
        }
    }

    fn set_user_donation_amount(env: Env, from: Address, token: &Address, donation_amount: i128) {
        env.storage().instance().set(&(from, token), &donation_amount);
    }

    fn set_charity_donation_amount(
        env: Env,
        charity_account_id: u32,
        token: &Address,
        donation_amount: i128
    ) {
        env.storage().instance().set(&(charity_account_id, token), &donation_amount);
    }

    pub fn get_user_donation_amount(env: Env, from: Address, token: Address) -> i128 {
        let donation_amount = env.storage().instance().get(&(from, token)).unwrap();

        donation_amount
    }

    pub fn get_charity_donation_amoutn(env: Env, charity_account_id: u32, token: Address) -> i128 {
        let donation_amount = env.storage().instance().get(&(charity_account_id, token)).unwrap();

        donation_amount
    }
}

mod test;