mod apartment;

use web_sys::window;
use yew::prelude::*;

use crate::Route;
use crate::components::{ButtonLink, ImageInput};
use crate::editor::apartment::ApartmentTemplate;

#[derive(Properties, Debug, PartialEq)]
pub struct TemplateEditorProps {
    #[prop_or_default]
    pub pane_count: u32,
}

#[function_component]
pub fn TemplateEditor(props: &TemplateEditorProps) -> Html {
    
    let pane_count = props.pane_count;
    let onclick = |_| { window().unwrap().print().unwrap(); };
    
    html! {
        <div class="centralizer">
            <div>
                {for (0..pane_count).map(|_| html_nested!(<ApartmentTemplate {pane_count}/>))}
            </div>

            <div class="row">
                <ButtonLink<Route> to={Route::TemplateSelector}>
                    { "Voltar" }
                </ButtonLink<Route>>

                <button type="button" {onclick}>
                    { "Imprimir" }
                </button>
            </div>
        </div>
    }
}
