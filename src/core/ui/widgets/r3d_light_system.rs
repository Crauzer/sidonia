use crate::core::{
    riot::r3d::{light::R3dLight, light_system::R3dLightSystem},
    ui::widgets::{r3d_light::R3dLightWidget, Widget},
};
use imgui::Ui;

pub struct R3dLightSystemWidget {
    lights: Vec<R3dLightWidget>,
    visible_lights: Vec<R3dLightWidget>,
}

impl R3dLightSystemWidget {
    pub fn new() -> Self {
        R3dLightSystemWidget {
            lights: Vec::new(),
            visible_lights: Vec::with_capacity(1024),
        }
    }

    pub fn fetch_data(&self, light_system: &mut R3dLightSystem) {
        // We can assume that the widgets and their respective lights are in the correct order
        let mut lights: Vec<&mut R3dLight> = light_system.lights_mut().iter().map(|light| light).collect();
        for i in 0..lights.len() {
            self.lights[i].fetch_data(lights.get_mut(i).unwrap());
        }
    }

    pub fn update(&mut self, light_system: &R3dLightSystem) {
        self.lights = light_system.lights().iter().map(|light| R3dLightWidget::from(&*light)).collect();
        if let Some(visible_lights) = light_system.visible_lights() {
            self.visible_lights = visible_lights.iter().map(|light| R3dLightWidget::from(light)).collect();
        }
    }
}

impl Widget for R3dLightSystemWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        imgui::Window::new(im_str!("Light System"))
            .size([200.0, 300.0], imgui::Condition::Appearing)
            .always_auto_resize(true)
            .build(&ui, || {
                ui.text(format!("Light Count: {}", self.lights.len()));
                imgui::TreeNode::new(im_str!("Lights")).build(ui, || {
                    for i in 0..self.lights.len() {
                        imgui::TreeNode::new(&imgui::ImString::new(format!("{}", i))).build(ui, || {
                            self.lights[i].render(ui);
                        });
                    }
                });

                ui.text(format!("Visible Light Count: {}", self.visible_lights.len()));
                imgui::TreeNode::new(im_str!("Visible Lights")).build(ui, || {
                    for i in 0..self.visible_lights.len() {
                        imgui::TreeNode::new(&imgui::ImString::new(format!("{}", i))).build(ui, || {
                            self.visible_lights[i].render(ui);
                        });
                    }
                });
            });
    }
}
