/*!
 * Global broswer icon resource module.
 *
 * This module handles instancing of images in memory. To get the broswer icons, just call the function with
 * the same name as the broswer icon you want.
 */

use eframe::egui::Context;
use eframe::emath::{Rect, Vec2};
use eframe::epaint::TextureId;

use egui_extras::RetainedImage;

/**
 * This holds the global instance of the broswer icons.
 *
 * This variable is used by the other public functions to retrieve the associated icon.
*/
static mut ICONS: Option<BrowserIconsResource> = None;

struct BrowserIconsResource {
    edge: RetainedImage,
    chrome: RetainedImage,
    firefox: RetainedImage,
}

unsafe impl Sync for BrowserIconsResource {}

impl BrowserIconsResource {
    fn new() -> BrowserIconsResource {
        BrowserIconsResource {
            edge: RetainedImage::from_image_bytes("edge.png", include_bytes!("../icons/edge.png"))
                .unwrap(),
            chrome: RetainedImage::from_image_bytes(
                "chrome.png",
                include_bytes!("../icons/chrome.png"),
            )
            .unwrap(),
            firefox: RetainedImage::from_image_bytes(
                "firefox.png",
                include_bytes!("../icons/firefox.png"),
            )
            .unwrap(),
        }
    }
}

fn instance_icons_resource() -> () {
    unsafe { ICONS = Some(BrowserIconsResource::new()) };
}

pub fn icon_size(ctx: &Context) -> Vec2 {
    let screen: Rect = ctx.screen_rect();
    // screen.width() / 10.0 translates to one tenth of the screen space.
    // Meaning that the size of the icon will be 10% of the screen width.
    let icon_size: Vec2 = Vec2 {
        x: screen.width() / 10.0,
        y: screen.width() / 10.0,
    };
    return icon_size;
}

#[inline(always)]
fn force_get_icons() -> &'static BrowserIconsResource {
    unsafe { ICONS.as_ref().unwrap() }
}

pub fn edge(ctx: &Context) -> TextureId {
    match unsafe { &ICONS } {
        Some(image) => image.edge.texture_id(ctx),
        None => {
            instance_icons_resource();
            force_get_icons().edge.texture_id(ctx)
        }
    }
}

pub fn chrome(ctx: &Context) -> TextureId {
    match unsafe { &ICONS } {
        Some(image) => image.chrome.texture_id(ctx),
        None => {
            instance_icons_resource();
            force_get_icons().chrome.texture_id(ctx)
        }
    }
}

pub fn firefox(ctx: &Context) -> TextureId {
    match unsafe { &ICONS } {
        Some(image) => image.firefox.texture_id(ctx),
        None => {
            instance_icons_resource();
            force_get_icons().firefox.texture_id(ctx)
        }
    }
}
