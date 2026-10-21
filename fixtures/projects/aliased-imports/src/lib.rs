// Fixture: aliased imports around storage and a cross-contract client.
use soroban_sdk::{contractimpl, Address as Addr, Env as SEnv};

use crate::token::Client as C;
use crate::types::Key as StorageKey;

#[contractimpl]
impl Vault {
    pub fn store(env: SEnv, id: Addr, value: u32) {
        env.storage().persistent().set(&StorageKey::Value, &value);
        let client = C::new(&env, &id);
        client.transfer(&env.current_contract_address(), &id, &value);
    }
}
