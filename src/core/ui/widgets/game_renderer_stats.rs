use crate::core::{riot::r3d::render_layer::R3dRenderLayerStats, ui::widgets::Widget};
use arraydeque::{ArrayDeque, Wrapping};
use imgui::Ui;

pub struct GameRendererStatsWidget {
    texture_memory_values: ArrayDeque<[f32; 100], Wrapping>,
    buffer_memory_values: ArrayDeque<[f32; 100], Wrapping>,
    screen_buffer_memory_values: ArrayDeque<[f32; 100], Wrapping>,
    material_change_count_values: ArrayDeque<[f32; 100], Wrapping>,
    mode_changes_count_values: ArrayDeque<[f32; 100], Wrapping>,
    texture_changes_count_values: ArrayDeque<[f32; 100], Wrapping>,
    triangles_rendered_count_values: ArrayDeque<[f32; 100], Wrapping>,
    average_strip_length_values: ArrayDeque<[f32; 100], Wrapping>,
    draw_count_values: ArrayDeque<[f32; 100], Wrapping>,
}

impl GameRendererStatsWidget {
    pub fn new() -> Self {
        GameRendererStatsWidget {
            texture_memory_values: ArrayDeque::default(),
            buffer_memory_values: ArrayDeque::default(),
            screen_buffer_memory_values: ArrayDeque::default(),
            material_change_count_values: ArrayDeque::default(),
            mode_changes_count_values: ArrayDeque::default(),
            texture_changes_count_values: ArrayDeque::default(),
            triangles_rendered_count_values: ArrayDeque::default(),
            average_strip_length_values: ArrayDeque::default(),
            draw_count_values: ArrayDeque::default(),
        }
    }

    pub fn update(&mut self, stats: &R3dRenderLayerStats) {
        self.texture_memory_values.push_back(stats.texture_memory() as f32);
        self.buffer_memory_values.push_back(stats.buffer_memory() as f32);
        self.screen_buffer_memory_values.push_back(stats.screen_buffer_memory() as f32);
        self.material_change_count_values.push_back(stats.material_change_count() as f32);
        self.mode_changes_count_values.push_back(stats.mode_changes_count() as f32);
        self.texture_changes_count_values.push_back(stats.texture_changes_count() as f32);
        self.triangles_rendered_count_values
            .push_back(stats.triangles_rendered_count() as f32);
        self.average_strip_length_values.push_back(stats.average_strip_length() as f32);
        self.draw_count_values.push_back(stats.draw_count() as f32);
    }
}

impl Widget for GameRendererStatsWidget {
    fn render<'ui>(&mut self, ui: &'ui imgui::Ui) {
        ui.plot_histogram(im_str!("Texture Memoey"), self.texture_memory_values.as_slices().0)
            .scale_min(0.0)
            .scale_max(1073741824.0 / 2.0)
            .graph_size([150.0, 30.0])
            .build();

        ui.plot_histogram(im_str!("Buffer Memory"), self.buffer_memory_values.as_slices().0)
            .scale_min(0.0)
            .scale_max(1073741824.0 / 2.0)
            .graph_size([150.0, 30.0])
            .build();

        ui.plot_histogram(im_str!("Screen Buffer Memory"), self.screen_buffer_memory_values.as_slices().0)
            .scale_min(0.0)
            .scale_max(1073741824.0 / 2.0)
            .graph_size([150.0, 30.0])
            .build();

        ui.plot_histogram(im_str!("Material Changes"), self.material_change_count_values.as_slices().0)
            .scale_min(0.0)
            .scale_max(1000.0)
            .graph_size([150.0, 30.0])
            .build();

        ui.plot_histogram(im_str!("Mode Changes"), self.mode_changes_count_values.as_slices().0)
            .scale_min(0.0)
            .scale_max(1000.0)
            .graph_size([150.0, 30.0])
            .build();

        ui.plot_histogram(im_str!("Texture Changes"), self.texture_changes_count_values.as_slices().0)
            .scale_min(0.0)
            .scale_max(5000.0)
            .graph_size([150.0, 30.0])
            .build();

        ui.plot_histogram(im_str!("Tris Rendered"), self.triangles_rendered_count_values.as_slices().0)
            .scale_min(0.0)
            .scale_max(u16::MAX as f32)
            .graph_size([150.0, 30.0])
            .build();

        ui.plot_histogram(im_str!("Average Strip Length"), self.average_strip_length_values.as_slices().0)
            .scale_min(0.0)
            .scale_max(u16::MAX as f32)
            .graph_size([150.0, 30.0])
            .build();

        ui.plot_histogram(im_str!("Draws"), self.draw_count_values.as_slices().0)
            .scale_min(0.0)
            .scale_max(10000.0)
            .graph_size([150.0, 30.0])
            .build();
    }
}
