use crate::core::hook_manager::HookManagerError::{HookEnableFailed, HookNotFound};
use detour::{Function, GenericDetour, StaticDetour, RawDetour};
use std::{collections::{hash_map::Entry, HashMap}, error::Error, fmt, path::Display, mem};
use std::any::Any;

pub struct HookManager {
    hooks: HashMap<String, RawDetour>,
}

#[derive(Debug)]
pub enum HookManagerError {
    HookAlreadyRegistered(String),
    HookNotFound(String),
    HookEnableFailed(String, detour::Error),
    HookDisableFailed(String, detour::Error),
}

impl HookManager {
    pub fn new() -> Self {
        HookManager { hooks: HashMap::new() }
    }

    pub fn get_hook(&self, name: &str) -> Option<&RawDetour> {
        self.hooks.get(&String::from(name))
    }
    pub fn get_hook_critical(&self, name: &str) -> &RawDetour {
        self.get_hook(name).expect(&format!("Failed to get hook: {}", name))
    }
    //pub unsafe fn get_hook_trampoline<F: Sized>(&self, name: &str) -> Option<F> {
    //    self.get_hook(name).and_then(|hook| Some(mem::transmute::<*const (), F>(hook.trampoline() as *const ())))
    //}
    //pub unsafe fn get_hook_trampoline_critical<F: Sized>(&self, name: &str) -> F {
    //    self.get_hook_trampoline(name).expect(&format!("Failed to get trampoline for hook: {}", name))
    //}

    pub fn register_hook(&mut self, name: &str, hook: RawDetour) -> Result<(), HookManagerError> {
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

    pub fn enable_hooks(&self) -> Result<(), HookManagerError> {
        for (hook_name, hook) in self.hooks.iter().filter(|hook_entry| !hook_entry.1.is_enabled()) {
            unsafe {
                hook.enable()
                    .or_else(|err| return Err(HookManagerError::HookEnableFailed(hook_name.clone(), err)));
            }

            log::info!("Enabled hook: {}", hook_name);
        }

        Ok(())
    }
    pub fn disable_hooks(&self) -> Result<(), HookManagerError> {
        for (hook_name, hook) in self.hooks.iter().filter(|hook_entry| hook_entry.1.is_enabled()) {
            unsafe {
                hook.enable()
                    .or_else(|err| return Err(HookManagerError::HookDisableFailed(hook_name.clone(), err)));
            }

            log::info!("Disabled hook {}", hook_name);
        }

        Ok(())
    }

    pub fn enable_hook(&self, hook_name: &str) -> Result<(), HookManagerError> {
        log::info!("Enabling hook: {}", hook_name);

        // TODO: disgusting pls fix
        self.hooks
            .get(hook_name)
            .map_or(Err(HookManagerError::HookNotFound(String::from(hook_name))), |hook| unsafe {
                if !hook.is_enabled() {
                    hook.enable()
                        .or_else(|err| return Err(HookManagerError::HookEnableFailed(String::from(hook_name), err)))
                } else {
                    Ok(())
                }
            })
    }
    pub fn disable_hook(&self, hook_name: &str) -> Result<(), HookManagerError> {
        log::info!("Disabling hook: {}", hook_name);

        // TODO: disgusting pls fix
        self.hooks
            .get(hook_name)
            .map_or(Err(HookManagerError::HookNotFound(String::from(hook_name))), |hook| unsafe {
                if hook.is_enabled() {
                    hook.disable()
                        .or_else(|err| return Err(HookManagerError::HookDisableFailed(String::from(hook_name), err)))
                } else {
                    Ok(())
                }
            })
    }
}

impl fmt::Display for HookManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HookManagerError::HookAlreadyRegistered(name) => write!(f, "{} is already registered", name),
            HookManagerError::HookNotFound(name) => write!(f, "{} was not found in the map of registered hooks", name),
            HookManagerError::HookEnableFailed(name, err) => write!(f, "Failed to enable hook {}, reason: {}", name, err),
            HookManagerError::HookDisableFailed(name, err) => write!(f, "Failed to disable hook {}, reason: {}", name, err),
        }
    }
}
