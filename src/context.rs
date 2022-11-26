mod template_selector;
mod template_editor;

use std::{rc::Rc, ops::Deref};

use yew::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    TemplateSelector,
    TemplateEditor {
        pane_count: u32,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub current_page: Page,
}

impl Reducible for Message {
    type Action = Page;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Message { current_page: action }.into()
    }
}

pub type MessageContext = UseReducerHandle<Message>;

#[function_component]
pub fn MessageProvider() -> Html {
    let msg = use_reducer(|| Message {
        current_page: Page::TemplateSelector,
    });

    let inner_component: Html = match &msg.current_page {
        Page::TemplateSelector => html! { <template_selector::TemplateSelector/> },
        Page::TemplateEditor { pane_count } => html! { <template_editor::TemplateEditor pane_count={pane_count}/> },
    };

    html! {
        <ContextProvider<MessageContext> context={msg}>
            {inner_component}
        </ContextProvider<MessageContext>>
    }
}

fn set_page<EVENT, CTX: Deref<Target = MessageContext> + 'static>(page: Page, ctx: CTX) -> Callback<EVENT> {
    Callback::from(move |_| {
        ctx.dispatch(page);
    })
}