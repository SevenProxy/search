use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::Card;

#[component]
pub fn SearchPage() -> Html {
    let location = use_location();

    let query_param = location.and_then(|loc| {
        loc.query::<std::collections::HashMap<String, String>>().ok()
    });

    let search_term = query_param
        .and_then(|map| map.get("q").cloned())
        .unwrap_or_else(|| "Search not found".to_string());

    html! {
        <div class="bg-black">
            <Main />
        </div>
    }
}


#[component]
pub fn Main() -> Html {
    html! {
        <div class="text-white min-h-screen bg-black text-white font-sans">
            <div class="border-b border-solid border-zinc-900">
                <Header />
                <SearchNav />
            </div>
            <Content />
            <footer class="w-full mt-8 p-10 border-t border-solid border-zinc-900">
                <div>
                    <div class="flex items-center justify-between p-4 border border-solid borde-[#1a1a1a] rounded-md">
                        <div class="flex items-start gap-2">
                            <img class="w-10 h-10 rounded-full" src="public/img/insonia-icon.png" />
                            <div class="flex flex-col items-start gap-1">
                                <p class="text-xl font-bold">{"INSONIA"}</p>
                                <p class="text-zinc-400">{"Entre no nosso servidor no Discord"}</p>
                            </div>
                        </div>
                        <div>
                            <a class="py-2 px-4 rounded-md bg-purple-600 text-white text-base">{"Join"}</a>
                        </div>
                    </div>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn Header() -> Html {
    html! {
        <div class="flex items-center justify-between ml-[5rem] mr-8 pt-10">
            <div class="flex items-center justify-between gap-10">
                <div class="flex items-center gap-3">
                    <img src="https://duckduckgo.com/assets/logo_homepage.normal.v108.svg" class="w-8 h-8" />
                    <span class="text-lg font-semibold">{"BRASILISMO"}</span>
                </div>
                <SearchBar />
            </div>
            <div class="text-sm text-gray-400">{"≡"}</div>
        </div>
    }
}

#[component]
fn SearchBar() -> Html {
    html! {
        <div class="flex justify-center">
            <div class="w-[600px] bg-[#1a1a1a] rounded-md py-2 px-4 flex items-center gap-3">
                <input
                    type="text"
                    placeholder="Search..."
                    class="bg-transparent w-full outline-none text-white"
                />
                <span class="text-gray-400">{"🔍"}</span>
            </div>
        </div>
    }
}

#[component]
fn SearchNav() -> Html {
    html! {
        <nav class="ml-[17.5rem]">
            <ul class="flex items-center gap-3">
                <li class="">{"Todos"}</li>
                <li>{"Imagens"}</li>
            </ul>
        </nav>
    }
}

#[component]
fn Content() -> Html {
    html! {
        <div class="grid grid-cols-3 gap-2 ml-[17.5rem] mr-8 mt-8">
            <Results />
            <Card />
        </div>
    }
}

#[component]
fn Results() -> Html {
    html! {
        <div class="col-span-2 space-y-7 h-screen">
            <ResultItem
                title="Quando usar a, à, há e ah?"
                link="dicio.com.br"
                description="Saiba as diferenças entre essas formas e como usar corretamente..."
            />

            <ResultItem
                title="Colégio Classe A"
                link="colegioclassea.com.br"
                description="Informações sobre a instituição e contatos..."
            />

            <ResultItem
                title="A - Wikipédia"
                link="wikipedia.org"
                description="A letra A é a primeira do alfabeto latino..."
            />

            <div class="w-full my-4">
                <button class="w-full p-2 rounded-md bg-zinc-600 font-bold text-xl">{"Buscar mais"}</button>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ResultProps {
    title: &'static str,
    link: &'static str,
    description: &'static str,
}

#[component]
fn ResultItem(props: &ResultProps) -> Html {
    html! {
        <div class="space-y-1">
            <div class="text-sm text-gray-400">{ props.link }</div>
            <div class="text-blue-400 text-lg cursor-pointer hover:underline">
                { props.title }
            </div>
            <div class="text-gray-300 text-sm">{ props.description }</div>
        </div>
    }
}

