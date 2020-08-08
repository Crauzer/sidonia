use crate::core::hook_manager::HookManagerError::HookNotFound;
use detour::{Function, GenericDetour, StaticDetour};
use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    fmt,
    path::Display,
};
use winapi::_core::fmt::Formatter;

pub struct HookManager<F: Function> {
    hooks: HashMap<String, GenericDetour<F>>,
}

pub enum HookManagerError {
    HookAlreadyRegistered(String),
    HookNotFound(String),
    HookDisableFailed(String, detour::Error),
}

impl<F: Function> HookManager<F> {
    pub fn new() -> Self {
        HookManager { hooks: HashMap::new() }
    }

    pub fn register_hook(&mut self, name: &str, hook: GenericDetour<F>) -> Result<(), HookManagerError> {
        match self.hooks.entry(String::from(name)) {
            Entry::Occupied(entry) => {
                log::info!("Failed to register hook {} because it's already registered", name);

                return Err(HookManagerError::HookAlreadyRegistered(String::from(name)));
            }
            Entry::Vacant(entry) => {
                log::info!("Registering hook: {}", name);

                entry.insert(hook);
            }
        };

        Ok(())
    }
    pub fn deregister_hook(&mut self, name: &str) -> Result<(), HookManagerError> {
        match self.hooks.entry(String::from(name)) {
            Entry::Occupied(entry) => {
                log::info!("Deregistering hook: {}", name);

                unsafe {
                    entry
                        .get()
                        .disable()
                        .or_else(|err| return Err(HookManagerError::HookDisableFailed(String::from(name), err)));
                }

                entry.remove_entry();
            }
            Entry::Vacant(entry) => {
                log::info!("Failed to deregister hook {} because it doesn't exist", name);

                return Err(HookNotFound(String::from(name)));
            }
        };

        Ok(())
    }

    pub fn initialize_hooks() -> Result<(), HookManagerError> {
        Ok(())
    }

    pub fn enable_hooks() -> Result<(), HookManagerError> {
        Ok(())
    }

    pub fn disable_hooks() -> Result<(), HookManagerError> {
        Ok(())
    }
}

impl fmt::Display for HookManagerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            HookManagerError::HookAlreadyRegistered(name) => write!(f, "{} is already registered", name),
            HookManagerError::HookNotFound(name) => write!(f, "{} was not found in the map of registered hooks", name),
            HookManagerError::HookDisableFailed(name, err) => write!(f, "Failed to disable hook {}, reason: {}", name, err),
        }
    }
}
