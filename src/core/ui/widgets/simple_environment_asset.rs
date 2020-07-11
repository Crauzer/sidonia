use crate::core::{
    riot::simple_environment::RiotSimpleEnvironmentAsset,
    ui::widgets::{simple_environment_material::SimpleEnvironmentMaterialWidget, Widget},
};
use imgui::Ui;
use std::ptr;
use winapi::shared::minwindef::{LPCVOID, LPVOID};

pub struct SimpleEnvironmentAssetWidget {
    name: String,
    package: LPCVOID,
    materials: Vec<SimpleEnvironmentMaterialWidget>,
}

impl SimpleEnvironmentAssetWidget {
    pub fn new() -> Self {
        SimpleEnvironmentAssetWidget {
            name: String::with_capacity(260),
            package: ptr::null_mut(),
            materials: Vec::new(),
        }
    }

    pub fn fetch_data(&self, simple_environment_asset: &mut RiotSimpleEnvironmentAsset) {
        for i in 0..simple_environment_asset.materials().len() {
            if let Some(material) = &mut simple_environment_asset.materials_mut()[i] {
                &self.materials[i].fetch_data(*material);
            }
        }
    }

    pub fn update(&mut self, simple_environment_asset: &RiotSimpleEnvironmentAsset) {
        self.name = simple_environment_asset.name();
        self.package = simple_environment_asset.package_interface();

        self.materials.clear();
        for material in simple_environment_asset.materials() {
            let mut material_widget = SimpleEnvironmentMaterialWidget::new();

            if let Some(material) = material {
                material_widget.update(material);
            }

            self.materials.push(material_widget);
        }
    }
}

impl Widget for SimpleEnvironmentAssetWidget {
    fn render<'ui>(&mut self, ui: &'ui Ui<'ui>) {
        imgui::Window::new(im_str!("Simple Environment"))
            .size([200.0, 300.0], imgui::Condition::Appearing)
            .build(&ui, || {
                ui.bullet_text(&imgui::ImString::new(&self.name));
                ui.text(format!("Package: {:#?}", self.package));

                imgui::TreeNode::new(im_str!("Materials")).build(ui, || {
                    for material_widget in &mut self.materials {
                        material_widget.render(ui);
                    }
                });
            });
    }
}
