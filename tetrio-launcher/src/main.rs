mod config;
mod launcher;
mod ui;

use libadwaita::prelude::*;
use libadwaita::Application;

rust_i18n::i18n!("locales", fallback = "en");

fn main() -> gtk4::glib::ExitCode {
    if let Ok(lang) = std::env::var("LANG") {
        let locale = lang.split('.').next().unwrap_or("en");
        let locale = locale.split('_').next().unwrap_or("en");
        rust_i18n::set_locale(locale);
    }

    let app = Application::builder()
        .application_id("io.github.relativemodder.tetrio-flatpak")
        .build();

    app.connect_activate(ui::build_ui);

    app.run()
}
