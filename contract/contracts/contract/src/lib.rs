#![no_std]

use soroban_sdk::{
    contract, contractimpl, Env, Symbol, String, Map
};

#[contract]
pub struct ContractTemplates;

#[contractimpl]
impl ContractTemplates {

    /// Internal function to get storage key safely
    fn templates_key(env: &Env) -> Symbol {
        // MUST be <= 9 chars
        Symbol::short("temps")
    }

    /// Store a template
    pub fn set_template(env: Env, name: Symbol, content: String) {
        let key = Self::templates_key(&env);

        // Load existing map or create new one
        let mut templates: Map<Symbol, String> =
            env.storage()
                .instance()
                .get(&key)
                .unwrap_or(Map::new(&env));

        // Insert/update template
        templates.set(name, content);

        // Save back to storage
        env.storage().instance().set(&key, &templates);
    }

    /// Retrieve a template
    pub fn get_template(env: Env, name: Symbol) -> Option<String> {
        let key = Self::templates_key(&env);

        let templates: Map<Symbol, String> =
            env.storage()
                .instance()
                .get(&key)
                .unwrap_or(Map::new(&env));

        templates.get(name)
    }
}