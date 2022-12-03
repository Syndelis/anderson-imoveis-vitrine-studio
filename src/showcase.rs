use yew::prelude::*;
use crate::Route;
use crate::components::{BackAndPrint, ImageInput};

#[function_component]
pub fn Showcase() -> Html {
    html! {
        <div class="with-bg">
            <main>
                <h1 class="area-of-text showcase" contenteditable="true" data-placeholder="Título"/>
                <h2 class="area-of-text showcase" contenteditable="true" data-placeholder="Subtítulo"/>
                <ImageInput classes="showcase"/>

                <img src="/public/watermark.png?v=0" class="watermark"/>

                <BackAndPrint to={Route::TemplateSelector}/>
            </main>
        </div>
    }
}