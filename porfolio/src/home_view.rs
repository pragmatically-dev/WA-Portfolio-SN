
use yew::prelude::*;

pub struct HomeView;

pub enum Msg {}

impl Component for HomeView {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1>{"home"}</h1>
        }
    }
}
