
use yew::prelude::*;
use yew_router::prelude::*;

use self::home_view::HomeView;

#[path ="./home_view.rs"]
mod home_view;


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    AboutMe,
    #[at("/projects")]
    Projects,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html!{<HomeView/>},
        Route::AboutMe => todo!(),
        Route::Projects => todo!(),
        Route::Contact => todo!(),
        Route::NotFound => todo!(),
    }
}

#[function_component(RouterCustom)]
pub fn Router() -> Html {
    html! {
        <BrowserRouter>
        <Switch <Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}
