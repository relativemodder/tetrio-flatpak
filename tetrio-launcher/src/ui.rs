use gtk4::prelude::*;
use libadwaita::prelude::*;
use libadwaita::{AboutDialog, Application, ApplicationWindow, HeaderBar, PreferencesGroup, SwitchRow};
use crate::config::Settings;
use crate::launcher::launch_game;

pub fn build_ui(app: &Application) {
    let settings = Settings::load();
    let version = get_version_from_metainfo();

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&*rust_i18n::t!("title"))
        .default_width(450)
        .default_height(580)
        .resizable(false)
        .build();

    let main_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let header_bar = HeaderBar::builder()
        .show_end_title_buttons(true)
        .show_start_title_buttons(true)
        .build();

    let about_button = gtk4::Button::builder()
        .icon_name("help-about-symbolic")
        .tooltip_text(&*rust_i18n::t!("about"))
        .build();

    let window_weak = window.downgrade();
    let version_clone = version.clone();
    about_button.connect_clicked(move |_| {
        if let Some(win) = window_weak.upgrade() {
            let about = AboutDialog::builder()
                .application_name(&*rust_i18n::t!("title"))
                .developer_name("Relative")
                .version(&version_clone)
                .website("https://github.com/relativemodder/tetrio-flatpak")
                .comments(&*rust_i18n::t!("about_comment"))
                .build();
            about.present(Some(&win));
        }
    });

    header_bar.pack_start(&about_button);
    main_box.append(&header_bar);

    let content_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(20)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    let brand_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .build();

    let logo_image = gtk4::Image::builder()
        .icon_name("io.github.relativemodder.tetrio-flatpak")
        .pixel_size(96)
        .halign(gtk4::Align::Center)
        .margin_bottom(12)
        .build();
    logo_image.set_icon_name(Some("input-gaming-symbolic"));

    let title_label = gtk4::Label::builder()
        .label("TETR.IO")
        .halign(gtk4::Align::Center)
        .build();
    title_label.add_css_class("title-1");

    let subtitle_label = gtk4::Label::builder()
        .label(&*rust_i18n::t!("subtitle", version = version))
        .halign(gtk4::Align::Center)
        .build();
    subtitle_label.add_css_class("dim-label");

    brand_box.append(&logo_image);
    brand_box.append(&title_label);
    brand_box.append(&subtitle_label);
    content_box.append(&brand_box);

    let pref_group = PreferencesGroup::builder()
        .title(&*rust_i18n::t!("pref_title"))
        .build();

    let dgpu_row = SwitchRow::builder()
        .title(&*rust_i18n::t!("dgpu_title"))
        .subtitle(&*rust_i18n::t!("dgpu_subtitle"))
        .active(settings.dgpu)
        .build();

    let wayland_row = SwitchRow::builder()
        .title(&*rust_i18n::t!("wayland_title"))
        .subtitle(&*rust_i18n::t!("wayland_subtitle"))
        .active(settings.wayland)
        .build();

    let hide_while_running_row = SwitchRow::builder()
        .title(&*rust_i18n::t!("hide_title"))
        .subtitle(&*rust_i18n::t!("hide_subtitle"))
        .active(settings.hide_while_running)
        .build();

    pref_group.add(&dgpu_row);
    pref_group.add(&wayland_row);
    pref_group.add(&hide_while_running_row);
    content_box.append(&pref_group);

    let save_settings = {
        let dgpu_row = dgpu_row.clone();
        let wayland_row = wayland_row.clone();
        let hide_row = hide_while_running_row.clone();
        move || {
            let settings = Settings {
                dgpu: dgpu_row.is_active(),
                wayland: wayland_row.is_active(),
                hide_while_running: hide_row.is_active(),
            };
            settings.save();
        }
    };

    let save_1 = save_settings.clone();
    dgpu_row.connect_active_notify(move |_| save_1());
    let save_2 = save_settings.clone();
    wayland_row.connect_active_notify(move |_| save_2());
    let save_3 = save_settings.clone();
    hide_while_running_row.connect_active_notify(move |_| save_3());

    let launch_button = gtk4::Button::builder()
        .label(&*rust_i18n::t!("play"))
        .halign(gtk4::Align::Fill)
        .build();
    launch_button.add_css_class("suggested-action");
    launch_button.add_css_class("pill");

    let close_button = gtk4::Button::builder()
        .label(&*rust_i18n::t!("close"))
        .halign(gtk4::Align::Fill)
        .build();
    close_button.add_css_class("pill");

    let dgpu_row_clone = dgpu_row.clone();
    let wayland_row_clone = wayland_row.clone();
    let hide_row_clone = hide_while_running_row.clone();
    let launch_btn_clone = launch_button.clone();
    let window_clone = window.clone();

    launch_button.connect_clicked(move |_| {
        launch_game(
            dgpu_row_clone.is_active(),
            wayland_row_clone.is_active(),
            hide_row_clone.is_active(),
            &window_clone,
            &launch_btn_clone,
        );
    });

    let window_weak = window.downgrade();
    close_button.connect_clicked(move |_| {
        if let Some(win) = window_weak.upgrade() {
            win.close();
        }
    });

    let button_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(8)
        .build();
    button_box.append(&launch_button);
    button_box.append(&close_button);

    content_box.append(&button_box);
    main_box.append(&content_box);

    window.set_content(Some(&main_box));
    window.present();
    launch_button.grab_focus();
}

fn get_version_from_metainfo() -> String {
    let metainfo_path = "/app/share/metainfo/io.github.relativemodder.tetrio-flatpak.metainfo.xml";
    if let Ok(content) = std::fs::read_to_string(metainfo_path) {
        if let Some(pos) = content.find("<release version=\"") {
            let start = pos + "<release version=\"".len();
            if let Some(end) = content[start..].find("\"") {
                return content[start..start + end].to_string();
            }
        }
    }
    "10".to_string()
}
