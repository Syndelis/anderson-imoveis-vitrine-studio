#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::fs::read;
use std::sync::Mutex;

use tauri::{State, Manager};
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::http::{ResponseBuilder, Uri};

#[derive(Default)]
struct ImageCache(Mutex<HashMap<String, Vec<u8>>>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn image_dialog() -> Option<String> {

    let future = tauri::async_runtime::spawn_blocking(move || {
        FileDialogBuilder::new()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file().map(|path| path.to_string_lossy().to_string())
    });

    future.await.unwrap().map(|path| {
        let mut url = String::from("reqimg://localhost/?n=");
        url_escape::encode_query_to_string(&path, &mut url);
        println!("{}", &url);
        url
    })

}

fn main() {
    tauri::Builder::default()
        .manage(ImageCache::default())
        .invoke_handler(tauri::generate_handler![image_dialog])
        .register_uri_scheme_protocol("reqimg", move |app, request| {
            let res_not_img = ResponseBuilder::new().status(404).body(Vec::new());
            if request.method() != "GET" { return res_not_img; }
            
            let uri = request.uri().parse::<Uri>().unwrap();

            let query_params = uri.query().unwrap();
            let file_path = &url_escape::decode(query_params)[2..];

            let response = tauri::http::ResponseBuilder::new().mimetype("image/*");

            let image_cache: State<ImageCache> = app.state();

            {
                let mut image_cache = image_cache.0.lock().unwrap();
                if let Some(data) = image_cache.get(file_path) {
                    response.body(data.clone())
                }

                else if let Ok(data) = read(file_path) {
                    image_cache.insert(file_path.into(), data.clone());
                    response.body(data)
                }
                
                else {
                    res_not_img
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
