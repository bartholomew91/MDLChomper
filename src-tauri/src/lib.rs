use std::{fs, io::Read, path::Path};
use tauri::menu::{Menu, SubmenuBuilder};

#[tauri::command]
fn load_hex(path: &str) -> Vec<String> {
    let file_path = Path::new(&path);
    let mut fh = fs::File::open(file_path).unwrap();
    let buf = &mut vec![];
    let _ = fh.read_to_end(buf);
    let formatted_chunks = &mut vec![];

    for chunk in buf {
        formatted_chunks.push(format!("{:02X?}", chunk));
    }

    formatted_chunks.to_owned()
}

#[tauri::command]
fn hex_to_f32(hex: &str) -> f32 {
    let bits = u32::from_str_radix(hex, 16).unwrap();
    f32::from_bits(bits)
}

#[tauri::command]
fn is_valid_f32(hex: &str) -> bool {
    false
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let handle = app.handle();
            let menu = Menu::new(handle)?;
            let submenu = SubmenuBuilder::new(handle, "File")
                .text("open-file", "Open File")
                .build()?;
            menu.append(&submenu)?;
            app.set_menu(menu);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_hex, hex_to_f32, is_valid_f32])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
