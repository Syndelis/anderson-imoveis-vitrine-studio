use yew::prelude::*;
use crate::Route;
use crate::components::{BackAndPrint, ImageInput};

#[function_component]
pub fn Showcase() -> Html {
    html! {
        <div class="with-bg">
            <main>
                <h1 class="area-of-text showcase" contenteditable="true" data-placeholder="Título">
                    { "Ed. De Paulo da Silva Sauro" }
                </h1>

                <h2 class="area-of-text showcase" contenteditable="true" data-placeholder="Subtítulo">
                    { "Frente Para o Mar" }
                </h2>

                <ImageInput classes="showcase"/>

                <img src="/public/watermark.png?v=0" class="watermark"/>

                <BackAndPrint to={Route::TemplateSelector}/>
            </main>
        </div>
    }
}