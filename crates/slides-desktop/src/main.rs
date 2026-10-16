mod app;
mod painting;

use breakfree_slides_core::{
    office::{self, Launch},
    storage,
};
fn main() {
    if let Err(e) = run() {
        eprintln!("BreakFree Slides: {e}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let launch = office::parse(std::env::args_os().skip(1))?;
    match &launch {
        Launch::Capabilities => {
            println!("{}", office::capabilities(env!("CARGO_PKG_VERSION")));
            return Ok(());
        }
        Launch::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        _ => {}
    }
    let (deck, path) = if let Launch::Open(path) = &launch {
        (storage::load(path)?, Some(path.clone()))
    } else {
        (Default::default(), None)
    };
    let capture = if let Launch::UiSmoke(dir) = launch {
        Some(dir)
    } else {
        None
    };
    let icon = image::load_from_memory(include_bytes!(
        "../../../docs/brand/breakfree-slides-transparent.webp"
    ))
    .map_err(|e| e.to_string())?
    .into_rgba8();
    let viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([1440.0, 900.0])
        .with_min_inner_size([800.0, 560.0])
        .with_icon(eframe::egui::IconData {
            width: icon.width(),
            height: icon.height(),
            rgba: icon.into_raw(),
        });
    eframe::run_native(
        "BreakFree Slides",
        eframe::NativeOptions {
            viewport,
            renderer: eframe::Renderer::Glow,
            ..Default::default()
        },
        Box::new(move |cc| Ok(Box::new(app::SlidesApp::new(cc, deck, path, capture)))),
    )
    .map_err(|e| e.to_string())
}
