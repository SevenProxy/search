use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Init,
    #[at("/search")]
    Search,

    #[not_found]
    #[at("/404")]
    NotFound,
}
