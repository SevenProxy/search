use yew::prelude::*;
use web_sys::{
    window,
    HtmlInputElement,
};
use crate::components::BannerComponent;

#[component]
pub fn App() -> Html {
    let search_input = use_state(|| String::new());

    let oninput = {
        let current = search_input.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current.set(input.value());
        })
    };

    let onsubmit = {
        let current = search_input.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if current.is_empty() {
                return;
            }

            if let Some(window) = window() {
                let location = window.location();

                location.set_href(&format!("https://www.google.com/search?q={}", (*current).clone())).unwrap();
            }
        })
    };

    html! {
        <main class="h-screen">
            <div class="7proxy w-full h-full bg-black">
                <div class="w-full h-full flex flex-col items-center justify-center my-auto">
                    <BannerComponent />
                    
                    <section class="my-4 w-full">
                        <form {onsubmit} class="w-full flex flex-col text-center items-center justify-center">
                            <div class="flex items-center gap-2">
                                <span class="text-yellow-600 font-bold">{"Brasilismo"}</span>
                                <p>{"Melhor search engine para "}<strong>{"patriotas."}</strong></p>
                            </div>
                            <div class="mt-6 md:max-w-[700px] max-w-[400px] flex items-center justify-center text-center w-full">
                                <input type="text" value={(*search_input).clone()} {oninput} class="bg-zinc-700 w-full py-2 px-2 rounded-md border-0 outline-none focus-0" placeholder=""/>
                            </div>
                            <div class="mt-4">
                                <div class="flex items-center gap-4">
                                    <button type="submit" class="hover:bg-yellow-500 hover:border-white py-1 px-4 rounded-md border border-solid border-yellow-500 bg-black text-white translate-full delay-150">{"Search"}</button>
                                    <button class="text-white bg-purple-800 border border-solid border-purple-800 flex items-center gap-2 py-1 px-4 rounded-md">
                                        <svg fill="currentColor" class="bi bi-discord w-6 h-6 text-white" viewBox="0 0 16 16">
                                            <path d="M13.545 2.907a13.2 13.2 0 0 0-3.257-1.011.05.05 0 0 0-.052.025c-.141.25-.297.577-.406.833a12.2 12.2 0 0 0-3.658 0 8 8 0 0 0-.412-.833.05.05 0 0 0-.052-.025c-1.125.194-2.22.534-3.257 1.011a.04.04 0 0 0-.021.018C.356 6.024-.213 9.047.066 12.032q.003.022.021.037a13.3 13.3 0 0 0 3.995 2.02.05.05 0 0 0 .056-.019q.463-.63.818-1.329a.05.05 0 0 0-.01-.059l-.018-.011a9 9 0 0 1-1.248-.595.05.05 0 0 1-.02-.066l.015-.019q.127-.095.248-.195a.05.05 0 0 1 .051-.007c2.619 1.196 5.454 1.196 8.041 0a.05.05 0 0 1 .053.007q.121.1.248.195a.05.05 0 0 1-.004.085 8 8 0 0 1-1.249.594.05.05 0 0 0-.03.03.05.05 0 0 0 .003.041c.24.465.515.909.817 1.329a.05.05 0 0 0 .056.019 13.2 13.2 0 0 0 4.001-2.02.05.05 0 0 0 .021-.037c.334-3.451-.559-6.449-2.366-9.106a.03.03 0 0 0-.02-.019m-8.198 7.307c-.789 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.45.73 1.438 1.613 0 .888-.637 1.612-1.438 1.612m5.316 0c-.788 0-1.438-.724-1.438-1.612s.637-1.613 1.438-1.613c.807 0 1.451.73 1.438 1.613 0 .888-.631 1.612-1.438 1.612"/>
                                        </svg>
                                        <a>{"Discord"}</a>
                                    </button>
                                </div>
                                <p class="mt-2 text-sm font-bold text-gray-700">{"Version: Beta 0.1"}</p>
                            </div>
                        </form>
                    </section>

                    <section class="mt-6">
                        <div class="w-full text-sm text-center">
                            <a class="text-blue-500" href="">{"Terms of Service"}</a>
                            <p class="text-gray-700">{"O Brasilismo é o caminho para uma nação prosperar."}</p>
                            <a class="text-blue-500" href="">{"Servidor no discord"}</a>
                        </div>
                    </section>
                </div>
            </div>
        </main>
    }
}
