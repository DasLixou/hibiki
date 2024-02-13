use eframe::{
    egui::{CursorIcon, Response, Sense, Ui, Widget},
    epaint::{Color32, Vec2},
};

pub struct Trigger {
    pub color: Color32,
}

impl Widget for Trigger {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_at_least(Vec2::splat(50.), Sense::click());
        ui.painter().rect_filled(rect, 10., self.color);

        if response.hovered {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        response
    }
}
