use std::env;
use std::path::PathBuf;

use bpi_rs::{Account, AccountProfile, BpiClient, BpiResult};

pub fn account_path() -> PathBuf {
    env::var("BPI_ACCOUNT_TOML")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("account.toml"))
}

pub fn account_profile() -> BpiResult<AccountProfile> {
    let profile = env::var("BPI_ACCOUNT_PROFILE").unwrap_or_else(|_| "vip".to_string());

    AccountProfile::parse(&profile)
}

pub fn load_account() -> BpiResult<Account> {
    Account::load_profile_from_path(account_path(), account_profile()?)
}

pub fn authenticated_client() -> BpiResult<BpiClient> {
    BpiClient::builder().account(load_account()?).build()
}
