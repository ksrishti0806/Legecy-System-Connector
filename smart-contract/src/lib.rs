#![no_std]
use soroban_sdk::{contract, contractimpl, Symbol, Env};

#[contract]
pub struct LegacyConnectorContract;

#[contractimpl]
impl LegacyConnectorContract {
    pub fn hello(env: Env, to: Symbol) -> (Symbol, Symbol) {
        (Symbol::new(&env, "Hello"), to)
    }
}
