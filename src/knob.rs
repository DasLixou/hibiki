use eframe::{
    egui::{CursorIcon, Response, Sense, Ui, Widget},
    epaint::{Color32, Stroke, Vec2},
};

pub struct Knob<'a> {
    pub hint_color: Color32,
    pub val: &'a mut f64,
}

impl Widget for Knob<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_at_least(Vec2::splat(40.), Sense::drag());
        ui.painter().circle_filled(
            rect.center(),
            rect.width() / 2.,
            catppuccin_egui::MACCHIATO.surface1,
        );
        ui.painter().line_segment(
            [rect.center(), rect.center_top()],
            Stroke::new(4., self.hint_color),
        );

        if response.hovered {
            ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
        }

        response
    }
}
