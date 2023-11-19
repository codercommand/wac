//#![windows_subsystem = "windows"]

// Adding support for my exe. I need to add it to the windows registry.
// https://superuser.com/questions/130672/how-do-i-set-the-windows-default-browser-for-a-custom-application-like-foobar
// https://learn.microsoft.com/en-gb/windows/win32/shell/start-menu-reg?redirectedfrom=MSDN
// https://stackoverflow.com/questions/32671277/how-do-i-register-a-custom-application-as-a-web-browser-in-windows-8-1
//
// I made a start, directory here in the registery:
// Computer\HKEY_LOCAL_MACHINE\SOFTWARE\Clients\StartMenuInternet\Wac
//
// It's harder to get working. I got to dig through lots of registry settings. 
// https://stackoverflow.com/questions/32354861/how-to-find-the-default-browser-via-the-registry-on-windows-10
//
//
// Official Windows how-to for registry
// https://learn.microsoft.com/en-us/windows/win32/shell/default-programs
// https://learn.microsoft.com/en-us/windows/win32/shell/fa-progids
// Turns out it changed in Windows 10
// https://blogs.windows.com/windows-insider/2015/05/20/announcing-windows-10-insider-preview-build-10122-for-pcs/
// Sounds like the only change was that the program itself can't set defaults.

use eframe::egui::*;

mod browsericons;
use browsericons::*;

mod iconbuttons;
use iconbuttons::*;

fn main() -> () {
    let window_config: eframe::NativeOptions = eframe::NativeOptions {
        always_on_top: true,
        maximized: true,
        decorated: false,
        icon_data: None, //Change this later so I have a custom icon
        transparent: true,
        active: true,
        centered: false,
        ..Default::default()
    };

    // If we have at least 1 argument, assume it's a url.
    // arg 0 is the directory of the application.
    let url: Option<String> = std::env::args().nth(0);
    // If we have 0 arguments, then don't even bother launching the program.
    if let Some(url) = url {
        println!("El: {}", url);
        eframe::run_native(
            "WAC",
            window_config,
            Box::new(|cc| Box::new(Wac::new(cc, url))),
        )
        .unwrap();
    }
}

struct Wac {
    website_url: String,
}

impl Wac {
    fn new(_cc: &eframe::CreationContext<'_>, url: String) -> Self {
        Wac { website_url: url }
    }

    fn render_url_preview_bar(ui: &mut Ui, url: &str) -> () {
        let url: RichText = RichText::new(url)
            .color(Color32::WHITE)
            .background_color(Color32::from_rgba_unmultiplied(12, 12, 12, 160))
            .size(24.0);
        ui.label(url);
    }

    fn render_browser_icons(&self, ui: &mut Ui) -> () {
    
    }
}

impl eframe::App for Wac {
    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> [f32; 4] {
        //Color32::TRANSPARENT.to_normalized_gamma_f32()
        Color32::from_rgba_unmultiplied(12, 12, 12, 80).to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let screen = ctx.screen_rect();

        // Scales the spacing between icons
        let padding = screen.width() / 10.0;
        // Gets the enter of all the icons with their padding.
        let icon_center = ((padding * 2.0) + (icon_size(ctx).x * 3.0)) / 2.0;
        //Gets the center of the screen, then subtracts the center offset of the icons so they align with the enter of the screen.
        let offset = (screen.width() / 2.0) - icon_center;

        Area::new(Id::null())
            .constrain(true)
            .movable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(screen.height() / 40.0);
                    Wac::render_url_preview_bar(ui, &self.website_url);
                    ui.add_space((screen.height() / 2.0) - (screen.height() / 40.0) - icon_size(ctx).y);
                    ui.horizontal(|ui| {
                        ui.add_space(offset);
                        chrome_button(ctx, frame, ui);
                        ui.add_space(padding);
                        firefox_button(ctx, frame, ui);
                        ui.add_space(padding);
                        edge_button(ctx, frame, ui);
                    });
                });
            });
    }
}