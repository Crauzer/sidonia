use crate::core::{
    msvc::string::StdString,
    riot::r3d::{color::R3dColor, light::R3dLight},
};

#[repr(C)]
pub struct R3dSun {
    light: R3dLight,
    pub ambient_color: R3dColor,
    atmosphere_mutator: StdString,
}

impl R3dSun {
    pub fn light(&self) -> &R3dLight {
        &self.light
    }
    pub fn light_mut(&mut self) -> &mut R3dLight {
        &mut self.light
    }
    pub fn atmosphere_mutator(&self) -> String {
        self.atmosphere_mutator.to_string()
    }
}
