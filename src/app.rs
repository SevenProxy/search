use yew::prelude::*;
use yew_router::prelude::*;
use crate::{
    routes::Route,
    page::{
        HomePage,
        SearchPage,
    },
};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Init => html! { <HomePage /> },
        Route::Search => html! { <SearchPage /> },
        Route::NotFound => html! { <HomePage /> },
    }
}

#[component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
