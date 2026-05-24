use std::process::Command;
use libadwaita::prelude::*;
use libadwaita::{AlertDialog, ApplicationWindow};

pub fn launch_game(
    use_dgpu: bool,
    use_wayland: bool,
    hide_while_running: bool,
    window: &ApplicationWindow,
    launch_btn: &gtk4::Button,
) {
    let mut cmd = Command::new("/app/share/tetrio/TETR.IO");
    cmd.arg("--no-sandbox");

    if use_wayland {
        cmd.arg("--ozone-platform-hint=auto");
        cmd.arg("--enable-features=WaylandWindowDecorations");
    } else {
        cmd.arg("--ozone-platform=x11");
    }

    let flatpak_id = std::env::var("FLATPAK_ID")
        .unwrap_or_else(|_| "io.github.relativemodder.tetrio-flatpak".to_string());
    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        let tmpdir = format!("{}/app/{}", runtime_dir, flatpak_id);
        cmd.env("TMPDIR", tmpdir);
    }

    if use_dgpu {
        cmd.env("__NV_PRIME_RENDER_OFFLOAD", "1");
        cmd.env("__GLX_VENDOR_LIBRARY_NAME", "nvidia");
        cmd.env("DRI_PRIME", "1");
    }

    match cmd.spawn() {
        Ok(mut child) => {
            launch_btn.set_sensitive(false);
            if hide_while_running {
                window.hide();
            } else {
                launch_btn.set_label(&*rust_i18n::t!("running"));
            }

            let (sender, receiver) = std::sync::mpsc::channel::<Result<(), String>>();
            
            let window_weak = window.downgrade();
            let launch_btn_clone = launch_btn.clone();
            
            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                if let Ok(res) = receiver.try_recv() {
                    if let Some(win) = window_weak.upgrade() {
                        if hide_while_running {
                            win.present();
                        }
                        launch_btn_clone.set_sensitive(true);
                        launch_btn_clone.set_label(&*rust_i18n::t!("play"));

                        if let Err(err_msg) = res {
                            let alert = AlertDialog::builder()
                                .heading(&*rust_i18n::t!("execution_failed"))
                                .body(&err_msg)
                                .build();
                            alert.add_response("ok", "OK");
                            alert.present(Some(&win));
                        }
                    }
                    gtk4::glib::ControlFlow::Break
                } else {
                    gtk4::glib::ControlFlow::Continue
                }
            });

            std::thread::spawn(move || {
                match child.wait() {
                    Ok(status) => {
                        if status.success() {
                            let _ = sender.send(Ok(()));
                        } else {
                            let code_str = status.code()
                                .map(|c| format!("{}", c))
                                .unwrap_or_else(|| "?".to_string());
                            let err_translated = rust_i18n::t!("crashed_with_error", code_str = code_str).to_string();
                            let _ = sender.send(Err(err_translated));
                        }
                    }
                    Err(err) => {
                        let _ = sender.send(Err(err.to_string()));
                    }
                }
            });
        }
        Err(err) => {
            eprintln!("Error spawning TETR.IO: {}", err);
            let alert = AlertDialog::builder()
                .heading(&*rust_i18n::t!("failed_to_launch"))
                .body(&*rust_i18n::t!("could_not_spawn", err = err.to_string()))
                .build();
            alert.add_response("ok", "OK");
            alert.present(Some(window));
        }
    }
}
