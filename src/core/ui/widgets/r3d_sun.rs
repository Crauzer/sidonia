use crate::core::{
    riot::r3d::sun::R3dSun,
    ui::widgets::{r3d_light::R3dLightWidget, Widget},
};
use imgui::Ui;

pub struct R3dSunWidget {
    atmosphere_mutator: String,
    ambient_color: [f32; 4],
    light_widget: R3dLightWidget,
}

impl R3dSunWidget {
    pub fn new() -> Self {
        R3dSunWidget {
            atmosphere_mutator: String::new(),
            ambient_color: [0.0, 0.0, 0.0, 0.0],
            light_widget: R3dLightWidget::new(),
        }
    }

    pub fn update(&mut self, sun: &R3dSun) {
        self.light_widget.update(sun.light());
        self.ambient_color = [
            sun.ambient_color.r as f32 / 255.0,
            sun.ambient_color.g as f32 / 255.0,
            sun.ambient_color.b as f32 / 255.0,
            sun.ambient_color.a as f32 / 255.0,
        ];
        self.atmosphere_mutator = sun.atmosphere_mutator();
    }
}

impl Widget for R3dSunWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        imgui::Window::new(im_str!("Sun"))
            .size([200.0, 300.0], imgui::Condition::Appearing)
            .always_auto_resize(true)
            .build(&ui, || {
                ui.text(im_str!("Atmosphere mutator: "));
                ui.same_line(50.0);
                ui.text(&imgui::ImString::new(&self.atmosphere_mutator));

                imgui::ColorEdit::new(im_str!("Ambient Color"), &mut self.ambient_color)
                    .picker(false)
                    .small_preview(true)
                    .alpha(true)
                    .build(ui);

                if imgui::CollapsingHeader::new(im_str!("Light")).default_open(false).build(&ui) {
                    self.light_widget.render(&ui);
                }
            });
    }
}
