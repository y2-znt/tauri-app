use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub fn setup(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyS);

    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app, triggered_shortcut, event| {
                if triggered_shortcut != &shortcut {
                    return;
                }

                if event.state() != ShortcutState::Pressed {
                    return;
                }

                match app.clipboard().read_image() {
                    Ok(image) => {
                        println!(
                            "clipboard image: {}x{} — {} bytes",
                            image.width(),
                            image.height(),
                            image.rgba().len()
                        );
                    }
                    Err(error) => {
                        println!("clipboard doesn't contain an image: {error}");
                    }
                }

                let Some(window) = app.get_webview_window("main") else {
                    return;
                };

                if window.is_visible().unwrap_or(false) {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            })
            .build(),
    )?;

    app.global_shortcut().register(shortcut)?;

    Ok(())
}
