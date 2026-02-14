use yew::prelude::*;

#[component]
pub fn App() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.svg" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle text-red-600">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
