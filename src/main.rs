// Nessuna console: su Windows TapDeck è sempre un'app con finestra (anche in debug),
// così all'avvio non lampeggia mai il terminale.
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod actions;
mod brand;
mod config;
mod detect;
mod i18n;
mod icons;
mod server;
mod ui;

/// Permette una sola istanza di TapDeck: se è già in esecuzione,
/// porta in primo piano la finestra esistente ed esce in silenzio.
#[cfg(target_os = "windows")]
mod single_instance {
    use std::ffi::c_void;
    use std::sync::OnceLock;

    const ERROR_ALREADY_EXISTS: u32 = 183;

    unsafe extern "system" {
        fn CreateMutexW(
            lpMutexAttributes: *const c_void,
            bInitialOwner: i32,
            lpName: *const u16,
        ) -> *mut c_void;
        fn GetLastError() -> u32;
        fn CloseHandle(hObject: *mut c_void) -> i32;
        fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> *mut c_void;
        fn SetForegroundWindow(hWnd: *mut c_void) -> i32;
    }

    /// Ritorna `true` se questa è l'unica istanza e va avanti,
    /// `false` se un'altra copia è già attiva (e la riporta in primo piano).
    pub fn try_acquire() -> bool {
        // Il nome è per-sessione: due utenti diversi possono usare TapDeck insieme.
        let name = "Local\\TapDeck.SingleInstance";
        let wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        unsafe {
            let h = CreateMutexW(std::ptr::null(), 0, wide.as_ptr());
            if h.is_null() {
                return true; // impossibile creare il mutex: non blocchiamo l'avvio
            }
            if GetLastError() == ERROR_ALREADY_EXISTS {
                focus_existing_window();
                let _ = CloseHandle(h);
                return false;
            }
            // Il mutex resta vivo per tutta la vita del processo (mai chiuso, apposta).
            static HANDLE: OnceLock<usize> = OnceLock::new();
            let _ = HANDLE.set(h as usize);
            true
        }
    }

    fn focus_existing_window() {
        unsafe {
            let title: Vec<u16> = "TapDeck — la tua Stream Deck"
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let hwnd = FindWindowW(std::ptr::null(), title.as_ptr());
            if !hwnd.is_null() {
                let _ = SetForegroundWindow(hwnd);
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod single_instance {
    pub fn try_acquire() -> bool {
        true
    }
}

use std::sync::{Arc, Mutex, RwLock};

use config::{LogSink, push_log};

/// Icona della finestra/taskbar: logo TapDeck generato all'avvio.
fn load_window_icon(icons_dir: &std::path::Path) -> Option<eframe::egui::viewport::IconData> {
    use eframe::egui::viewport::IconData;
    let bytes = std::fs::read(icons_dir.join("icon-192.png")).ok()?;
    let img = image::load_from_memory(&bytes).ok()?.to_rgba8();
    let (w, h) = (img.width(), img.height());
    Some(IconData {
        rgba: img.into_raw(),
        width: w,
        height: h,
    })
}

fn main() {
    // Prima di tutto: se TapDeck è già aperto, riportalo in primo piano ed esci
    // in silenzio (niente seconda finestra, niente terminale che lampeggia).
    if !single_instance::try_acquire() {
        return;
    }

    if let Err(e) = config::ensure_dirs() {
        eprintln!("Impossibile creare le cartelle di TapDeck: {e}");
    }
    let log: LogSink = Arc::new(Mutex::new(Vec::new()));

    let mut cfg = config::Config::load();
    // Riconoscimento automatico dei programmi: riempie i percorsi .exe vuoti
    // dei tasti "PC → apre un programma" (es. Discord, Spotify, OBS).
    let filled = detect::autofill_config(&mut cfg);
    if !filled.is_empty() {
        for msg in &filled {
            push_log(&log, msg.clone());
        }
        let _ = cfg.save();
    }
    let icons_dir = config::icons_dir();

    if let Err(e) = icons::generate_brand_icons(&icons_dir) {
        config::push_log(&log, format!("Icone PWA: {e}"));
    }
    // Scarica in background i loghi già usati dalla config.
    let used = icons::used_catalog_slugs(
        cfg.pages
            .iter()
            .flat_map(|p| p.buttons.iter())
            .filter_map(|b| b.icon.clone()),
    );
    icons::warm_catalog(icons_dir.clone(), used, log.clone());

    let store = Arc::new(RwLock::new(cfg.clone()));

    if std::env::args().any(|a| a == "--serve-only") {
        match server::start("0.0.0.0", cfg.port, icons_dir, store, log) {
            Ok(h) => {
                println!(
                    "TapDeck: http://127.0.0.1:{}  (Ctrl+C per fermare)",
                    h.local_addr.port()
                );
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(3600));
                }
            }
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([1160.0, 730.0])
        .with_min_inner_size([960.0, 620.0])
        .with_title("TapDeck — la tua Stream Deck");
    if let Some(icon) = load_window_icon(&icons_dir) {
        viewport = viewport.with_icon(icon);
    }
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    let result = eframe::run_native(
        "TapDeck",
        options,
        Box::new(move |_cc| Ok(Box::new(ui::TapDeckApp::new(store, icons_dir, log)))),
    );
    if let Err(e) = result {
        eprintln!("Errore nell'interfaccia: {e}");
        std::process::exit(1);
    }
}
