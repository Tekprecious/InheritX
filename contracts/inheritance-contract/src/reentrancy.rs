use soroban_sdk::Env;
use crate::{DataKey, InheritanceError};

pub struct ReentrancyGuard<'a> {
    env: &'a Env,
}

impl<'a> ReentrancyGuard<'a> {
    pub fn new(env: &'a Env) -> Result<Self, InheritanceError> {
        if env.storage().temporary().has(&DataKey::ReentrancyGuard) {
            return Err(InheritanceError::ReentrantCall);
        }
        env.storage().temporary().set(&DataKey::ReentrancyGuard, &true);
        Ok(Self { env })
    }
}

impl<'a> Drop for ReentrancyGuard<'a> {
    fn drop(&mut self) {
        self.env.storage().temporary().remove(&DataKey::ReentrancyGuard);
    }
}
