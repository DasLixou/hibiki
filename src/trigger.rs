use eframe::{
    egui::{CursorIcon, Response, Sense, Ui, Widget},
    epaint::Vec2,
};

pub struct Trigger {}

impl Widget for Trigger {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_at_least(Vec2::splat(50.), Sense::click_and_drag());
        ui.painter()
            .rect_filled(rect, 10., catppuccin_egui::MACCHIATO.surface1);

        if response.hovered {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        response
    }
}
