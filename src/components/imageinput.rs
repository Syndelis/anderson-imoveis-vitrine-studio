use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[function_component]
pub fn ImageInput() -> Html {

    let current_image: UseStateHandle<Option<String>> = use_state(|| None);

    let onclick = {
        let current_image = current_image.clone();
        Callback::from(move |_| {
            let current_image = current_image.clone();
            spawn_local(async move {
                let new_image = invoke("image_dialog", JsValue::NULL).await;
                let image_path = new_image.as_string();
                log(&image_path.clone().unwrap_or("None".into()));
                current_image.set(image_path);
            });
        })
    };

    let image_html = if let Some(path) = &*current_image {
        let host = window().unwrap().location().host().unwrap();
        let url = path.replacen("localhost", &host, 1);
        
        html!{
            <img src={url}/>
        }
    }
    else {
        html!()
    };

    html! {
        <>
        <div>{image_html}</div>
        <button type="button" {onclick}>{"Choose image"}</button>
        </>
    }
}