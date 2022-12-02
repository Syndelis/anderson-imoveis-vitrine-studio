use web_sys::window;
use yew::prelude::*;

use crate::{components::ButtonLink, Route};

#[derive(Properties, Debug, PartialEq)]
pub struct BackAndPrintProps {
    #[prop_or_default]
    pub to: Route,
}

#[function_component]
pub fn BackAndPrint(props: &BackAndPrintProps) -> Html {
    
    let to = props.to.clone();
    let onclick = |_| { window().unwrap().print().unwrap(); };
    
    html! {
        <div class="row back-and-print">
            <ButtonLink<Route> {to}>
                { "Voltar" }
            </ButtonLink<Route>>

            <button type="button" {onclick}>
                { "Imprimir" }
            </button>
        </div>
    }
}