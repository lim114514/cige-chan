use tauri_plugin_dialog::DialogExt;

#[derive(serde::Serialize)]
pub struct Opened {
    pub name: String,
    pub contents: String,
}

/// 弹系统保存对话框，把文本写到用户选的位置。
/// 返回 None 表示用户取消了。
#[tauri::command]
async fn save_text(
    app: tauri::AppHandle,
    default_name: String,
    filter_name: String,
    exts: Vec<String>,
    contents: String,
) -> Result<Option<String>, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();

    app.dialog()
        .file()
        .set_file_name(default_name)
        .add_filter(filter_name, &ext_refs)
        .save_file(move |p| {
            let _ = tx.send(p);
        });

    let picked = rx.recv().map_err(|e| e.to_string())?;
    let Some(fp) = picked else { return Ok(None) };
    let path = fp.into_path().map_err(|e| e.to_string())?;
    std::fs::write(&path, contents).map_err(|e| e.to_string())?;
    Ok(Some(path.to_string_lossy().to_string()))
}

/// 弹系统打开对话框，读回文本内容。
#[tauri::command]
async fn open_text(
    app: tauri::AppHandle,
    filter_name: String,
    exts: Vec<String>,
) -> Result<Option<Opened>, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();

    app.dialog()
        .file()
        .add_filter(filter_name, &ext_refs)
        .pick_file(move |p| {
            let _ = tx.send(p);
        });

    let picked = rx.recv().map_err(|e| e.to_string())?;
    let Some(fp) = picked else { return Ok(None) };
    let path = fp.into_path().map_err(|e| e.to_string())?;
    let contents = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    Ok(Some(Opened { name, contents }))
}

/// 弹系统打开对话框，原样读回二进制内容（给 MIDI 导入用，文本那套 open_text
/// 用 read_to_string 会把非 UTF-8 的字节弄坏）。
#[tauri::command]
async fn open_binary(
    app: tauri::AppHandle,
    filter_name: String,
    exts: Vec<String>,
) -> Result<Option<Vec<u8>>, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();

    app.dialog()
        .file()
        .add_filter(filter_name, &ext_refs)
        .pick_file(move |p| {
            let _ = tx.send(p);
        });

    let picked = rx.recv().map_err(|e| e.to_string())?;
    let Some(fp) = picked else { return Ok(None) };
    let path = fp.into_path().map_err(|e| e.to_string())?;
    let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
    Ok(Some(bytes))
}

/// 弹系统保存对话框，把二进制内容原样写到用户选的位置（给 MIDI 导出用）。
#[tauri::command]
async fn save_binary(
    app: tauri::AppHandle,
    default_name: String,
    filter_name: String,
    exts: Vec<String>,
    contents: Vec<u8>,
) -> Result<Option<String>, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();

    app.dialog()
        .file()
        .set_file_name(default_name)
        .add_filter(filter_name, &ext_refs)
        .save_file(move |p| {
            let _ = tx.send(p);
        });

    let picked = rx.recv().map_err(|e| e.to_string())?;
    let Some(fp) = picked else { return Ok(None) };
    let path = fp.into_path().map_err(|e| e.to_string())?;
    std::fs::write(&path, contents).map_err(|e| e.to_string())?;
    Ok(Some(path.to_string_lossy().to_string()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            save_text, open_text, open_binary, save_binary
        ])
        .run(tauri::generate_context!())
        .expect("词格酱启动失败");
}
