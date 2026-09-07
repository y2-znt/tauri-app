use clipboard_rs::{
    ClipboardHandler,
    ClipboardWatcher,
    ClipboardWatcherContext,
    Result,
};

struct ClipboardManager;

impl ClipboardHandler for ClipboardManager {
    fn on_clipboard_change(&mut self) {
        println!("clipboard changed");
    }
}

pub fn setup() -> Result<()> {
    let manager = ClipboardManager;

    let mut watcher = ClipboardWatcherContext::new()?;

    watcher.add_handler(manager);

    std::thread::spawn(move || {
        watcher.start_watch();
    });

    Ok(())
}