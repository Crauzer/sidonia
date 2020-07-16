use crate::core::riot::r3d::light::R3dLight;
use std::mem;

#[repr(C)]
#[derive(Debug)]
pub struct R3dLightLinkedList {
    start: *mut R3dLight,
}

#[repr(C)]
#[derive(Debug)]
pub struct R3dLightSystem {
    lights: R3dLightLinkedList,
    light_count: u32,
    visible_lights: *mut [R3dLight; 1024],
    visible_light_count: u32,
    /*
    r3dLightGraph m_LightGraph;
    r3dLocalLightShadows m_LocalShadows;
    r3dLightGrid m_LightGrid;
     */
}

impl R3dLightSystem {
    pub fn lights(&self) -> &R3dLightLinkedList {
        &self.lights
    }
    pub fn lights_mut(&mut self) -> &mut R3dLightLinkedList {
        &mut self.lights
    }
    pub fn light_count(&self) -> u32 {
        self.light_count
    }
    pub fn visible_lights(&self) -> Option<&[R3dLight; 1024]> {
        unsafe { self.visible_lights.as_ref() }
    }
    pub fn visible_lights_mut(&self) -> Option<&mut [R3dLight; 1024]> {
        unsafe { self.visible_lights.as_mut() }
    }
}

impl R3dLightLinkedList {
    pub fn iter(&self) -> R3dLightLinkedListIterator {
        R3dLightLinkedListIterator { current: self.start }
    }
}

pub struct R3dLightLinkedListIterator {
    current: *mut R3dLight,
}

impl Iterator for R3dLightLinkedListIterator {
    type Item = &'static mut R3dLight;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if !self.current.is_null() {
                let current = self.current;
                self.current = (*current).next;
                current.as_mut()
            } else {
                None
            }
        }
    }
}
