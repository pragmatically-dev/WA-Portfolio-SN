use yew::prelude::{function_component, html};
#[path ="./custom_router.rs"]
mod custom_router;
use custom_router::RouterCustom;
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <RouterCustom/>
    }
}
