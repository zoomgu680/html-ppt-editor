// HTMLEditor 桌面端 — 文件系统命令层
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Clone)]
struct DirEntry {
    name: String,
    rel_path: String,
    abs_path: String,
    is_dir: bool,
    size: u64,
    mime: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct FolderFile {
    rel_path: String,
    abs_path: String,
    name: String,
    mime: String,
    data_b64: String,
}

fn mime_of(p: &Path) -> String {
    match p.extension().and_then(|e| e.to_str()).map(|s| s.to_ascii_lowercase()).as_deref() {
        Some("html") | Some("htm") => "text/html".into(),
        Some("css") => "text/css".into(),
        Some("js") | Some("mjs") => "application/javascript".into(),
        Some("json") => "application/json".into(),
        Some("svg") => "image/svg+xml".into(),
        Some("png") => "image/png".into(),
        Some("jpg") | Some("jpeg") => "image/jpeg".into(),
        Some("gif") => "image/gif".into(),
        Some("webp") => "image/webp".into(),
        Some("avif") => "image/avif".into(),
        Some("ico") => "image/x-icon".into(),
        Some("woff") => "font/woff".into(),
        Some("woff2") => "font/woff2".into(),
        Some("ttf") => "font/ttf".into(),
        Some("otf") => "font/otf".into(),
        Some("txt") | Some("md") => "text/plain".into(),
        _ => "application/octet-stream".into(),
    }
}

fn is_text_ext(p: &Path) -> bool {
    matches!(
        p.extension().and_then(|e| e.to_str()).map(|s| s.to_ascii_lowercase()).as_deref(),
        Some("html") | Some("htm") | Some("css") | Some("js") | Some("mjs") | Some("json")
            | Some("svg") | Some("txt") | Some("md")
    )
}

fn is_bin_ext(p: &Path) -> bool {
    matches!(
        p.extension().and_then(|e| e.to_str()).map(|s| s.to_ascii_lowercase()).as_deref(),
        Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("webp") | Some("avif")
            | Some("ico") | Some("woff") | Some("woff2") | Some("ttf") | Some("otf")
    )
}

/// 递归列出目录（仅元数据，不含内容）
#[tauri::command]
fn list_dir(path: String) -> Result<Vec<DirEntry>, String> {
    let root = PathBuf::from(&path);
    if !root.is_dir() {
        return Err(format!("不是目录: {}", path));
    }
    let mut out = Vec::new();
    for entry in WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        let name = match p.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if name.starts_with('.') {
            continue;
        }
        let is_dir = entry.file_type().is_dir();
        if !is_dir && !is_text_ext(p) && !is_bin_ext(p) {
            continue;
        }
        let rel = p.strip_prefix(&root).unwrap_or(p);
        let rel_path = rel.to_string_lossy().replace('\\', "/");
        let abs_path = p.to_string_lossy().replace('\\', "/");
        let size = if is_dir { 0 } else { entry.metadata().map(|m| m.len()).unwrap_or(0) };
        out.push(DirEntry {
            name,
            rel_path,
            abs_path,
            is_dir,
            size,
            mime: if is_dir { String::new() } else { mime_of(p) },
        });
    }
    Ok(out)
}

/// 一次性读取目录下所有文件内容（base64），减少 IPC 往返
#[tauri::command]
fn read_folder_files(path: String) -> Result<Vec<FolderFile>, String> {
    let root = PathBuf::from(&path);
    if !root.is_dir() {
        return Err(format!("不是目录: {}", path));
    }
    let mut out = Vec::new();
    for entry in WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if entry.file_type().is_dir() {
            continue;
        }
        let name = match p.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if name.starts_with('.') {
            continue;
        }
        if !is_text_ext(p) && !is_bin_ext(p) {
            continue;
        }
        let data = match fs::read(p) {
            Ok(d) => d,
            Err(_) => continue,
        };
        let rel = p.strip_prefix(&root).unwrap_or(p);
        out.push(FolderFile {
            rel_path: rel.to_string_lossy().replace('\\', "/"),
            abs_path: p.to_string_lossy().replace('\\', "/"),
            name,
            mime: mime_of(p),
            data_b64: B64.encode(&data),
        });
    }
    Ok(out)
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("读取失败 {}: {}", path, e))
}

#[tauri::command]
fn read_binary(path: String) -> Result<String, String> {
    let data = fs::read(&path).map_err(|e| format!("读取失败 {}: {}", path, e))?;
    Ok(B64.encode(&data))
}

#[tauri::command]
fn write_file(path: String, text: String) -> Result<(), String> {
    // 确保父目录存在
    if let Some(parent) = Path::new(&path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(&path, text).map_err(|e| format!("写入失败 {}: {}", path, e))
}

#[tauri::command]
fn copy_file(src: String, dst: String) -> Result<(), String> {
    if let Some(parent) = Path::new(&dst).parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::copy(&src, &dst).map_err(|e| format!("复制失败 {} -> {}: {}", src, dst, e))?;
    Ok(())
}

#[tauri::command]
fn path_exists(path: String) -> bool {
    Path::new(&path).exists()
}

/// 检测系统是否已安装 WebView2 Runtime（Evergreen）
/// 与安装包的检测逻辑一致：查注册表 EdgeUpdate 客户端条目的 pv 值
#[cfg(target_os = "windows")]
fn webview2_installed() -> bool {
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;
    use windows_sys::Win32::System::Registry::*;

    const GUID: &str = "{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}";
    let locations = [
        (HKEY_LOCAL_MACHINE, format!("SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients\\{GUID}\0")),
        (HKEY_CURRENT_USER, format!("SOFTWARE\\Microsoft\\EdgeUpdate\\Clients\\{GUID}\0")),
    ];
    for (root, subkey) in &locations {
        unsafe {
            let mut hkey: HKEY = std::ptr::null_mut();
            if RegOpenKeyExA(*root, subkey.as_ptr(), 0, KEY_READ, &mut hkey) == ERROR_SUCCESS {
                let mut buf = [0u8; 64];
                let mut len = buf.len() as u32;
                let mut vtype = 0u32;
                let rc = RegQueryValueExA(
                    hkey,
                    b"pv\0".as_ptr(),
                    std::ptr::null(),
                    &mut vtype,
                    buf.as_mut_ptr(),
                    &mut len,
                );
                RegCloseKey(hkey);
                if rc == ERROR_SUCCESS && len > 1 {
                    let pv = String::from_utf8_lossy(&buf[..len as usize - 1]);
                    if !pv.is_empty() && pv != "0.0.0.0" {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// 缺少 WebView2 时弹窗提示并打开官方下载页，然后退出
#[cfg(target_os = "windows")]
fn prompt_webview2_missing() {
    const DOWNLOAD_URL: &str = "https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/";
    let choice = rfd::MessageDialog::new()
        .set_title("缺少 WebView2 运行时")
        .set_description(
            "本应用需要 Microsoft Edge WebView2 运行时才能运行，当前系统未安装。\n\n\
             点击“确定”打开官方下载页面，下载并安装 Evergreen Standalone Installer 后重新启动本应用。\n\n\
             下载地址：\nhttps://developer.microsoft.com/zh-cn/microsoft-edge/webview2/",
        )
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::OkCancel)
        .show();
    if choice == rfd::MessageDialogResult::Ok {
        let _ = open::that(DOWNLOAD_URL);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri::menu::{MenuBuilder, MenuItem, PredefinedMenuItem};
    use tauri::Emitter;

    // 启动前检测系统 WebView2：有则直接用系统的；没有则提示下载安装并退出
    #[cfg(target_os = "windows")]
    if !webview2_installed() {
        prompt_webview2_missing();
        return;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            list_dir,
            read_folder_files,
            read_file,
            read_binary,
            write_file,
            copy_file,
            path_exists,
        ])
        .setup(|app| {
            // 原生菜单
            let file_menu = tauri::menu::SubmenuBuilder::new(app, "文件")
                .item(&MenuItem::with_id(app, "open_dir", "打开文件夹", true, None::<&str>)?)
                .item(&MenuItem::with_id(app, "open_file", "打开 HTML", true, None::<&str>)?)
                .item(&PredefinedMenuItem::separator(app)?)
                .item(&MenuItem::with_id(app, "save", "保存", true, Some("CmdOrCtrl+S"))?)
                .item(&MenuItem::with_id(app, "save_as", "另存为", true, Some("CmdOrCtrl+Shift+S"))?)
                .item(&PredefinedMenuItem::separator(app)?)
                .item(&PredefinedMenuItem::quit(app, Some("退出"))?)
                .build()?;

            let edit_menu = tauri::menu::SubmenuBuilder::new(app, "编辑")
                .item(&MenuItem::with_id(app, "undo", "撤销", true, Some("CmdOrCtrl+Z"))?)
                .item(&MenuItem::with_id(app, "redo", "重做", true, Some("CmdOrCtrl+Shift+Z"))?)
                .item(&PredefinedMenuItem::separator(app)?)
                .item(&PredefinedMenuItem::select_all(app, Some("全选"))?)
                .build()?;

            let menu = MenuBuilder::new(app)
                .item(&file_menu)
                .item(&edit_menu)
                .build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| {
                let id = event.id().as_ref();
                let _ = app.emit("menu-action", id);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
