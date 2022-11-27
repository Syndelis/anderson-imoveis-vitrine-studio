use yew::prelude::*;
use yew_router::prelude::Link;

pub trait Routable = yew_router::Routable + Default + 'static;

#[function_component]
pub fn ButtonLink<R: Routable>(props: &ButtonLinkProps<R>) -> Html {
    let classes = props.classes.clone();
    let to = props.to.clone();
    let children = props.children.clone();

    html! {
        <Link<R> {classes} {to}>
            <button type="button">
                {children}
            </button>
        </Link<R>>
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct ButtonLinkProps<R: Routable> {
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub to: R,
    #[prop_or_default]
    pub children: Children,
}