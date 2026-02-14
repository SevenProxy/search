use yew::prelude::*;
use crate::components::BannerComponent;

#[component]
pub fn App() -> Html {
    html! {
        <main class="h-screen">
            <div class="7proxy w-full h-full bg-black">
                <div class="w-full h-full flex flex-col items-center justify-center my-auto">
                    <BannerComponent />
                    
                    <section class="my-4 w-full">
                        <div class="w-full flex flex-col text-center items-center justify-center">
                            <div class="flex items-center gap-2">
                                <span class="text-yellow-600 font-bold">{"Brasilismo"}</span>
                                <p>{"Melhor search engine para "}<strong>{"patriotas."}</strong></p>
                            </div>
                            <div class="mt-6 md:max-w-[700px] max-w-[400px] flex items-center justify-center text-center w-full">
                                <input class="bg-zinc-700 w-full py-2 px-2 rounded-md border-0 outline-0 focus-0" placeholder=""/>
                            </div>
                            <div class="mt-4">
                                <button class="hover:bg-yellow-600 hover:border-white py-1 px-4 rounded-md border border-solid border-yellow-600 bg-black text-white">{"Search"}</button>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        </main>
    }
}
