use crate::browsericons::*;

use eframe::{
    egui::{Context, ImageButton, TextureId, Ui},
    epaint::{Color32, Vec2},
};

fn render_button(
    ctx: &Context,
    frame: &mut eframe::Frame,
    ui: &mut Ui,
    button_texture: fn(ctx: &Context) -> TextureId,
    size: Vec2,
    button_hover_state: &'static mut bool,
) -> () {
    let mut button_widget: ImageButton = ImageButton::new(button_texture(ctx), size).frame(false);

    if *button_hover_state {
        button_widget = button_widget.tint(Color32::GRAY);
    }
    let button = ui.add(button_widget);

    if button.hovered() {
        *button_hover_state = true;
    } else {
        *button_hover_state = false;
    }

    if button.clicked() {
        std::process::Command::new("C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe")
            .args(["https://www.youtube.com/watch?v=l8KpjxesjsU"])
            .spawn()
            .expect("Error");
        frame.close();
    }
}

static mut EDGE_HOVER_STATE: bool = false;
pub fn edge_button(ctx: &Context, frame: &mut eframe::Frame, ui: &mut Ui) -> () {
    render_button(ctx, frame, ui, edge, icon_size(ctx), unsafe {
        &mut EDGE_HOVER_STATE
    });
}

static mut CHROME_HOVER_STATE: bool = false;
pub fn chrome_button(ctx: &Context, frame: &mut eframe::Frame, ui: &mut Ui) -> () {
    render_button(ctx, frame, ui, chrome, icon_size(ctx), unsafe {
        &mut CHROME_HOVER_STATE
    });
}

static mut FIREFOX_HOVER_STATE: bool = false;
pub fn firefox_button(ctx: &Context, frame: &mut eframe::Frame, ui: &mut Ui) -> () {
    render_button(ctx, frame, ui, firefox, icon_size(ctx), unsafe {
        &mut FIREFOX_HOVER_STATE
    });
}
