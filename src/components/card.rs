use yew::prelude::*;

#[component]
pub fn Card() -> Html {
    html! {
        <div class="space-y-3">
            <div class="bg-[#1a1a1a] p-4 rounded-lg">
                <div class="text-lg font-semibold mb-2">{"A"}</div>
                <p class="text-sm text-gray-300">
                    {"Primeira letra do alfabeto latino..."}
                </p>
            </div>

            <div class="bg-[#1a1a1a] p-4 rounded-lg">
                <div class="text-sm text-gray-400 mb-2">{"Pesquisas relacionadas"}</div>
                <ul class="space-y-2">
                    <li class="text-sm text-gray-300">{"a à ou há"}</li>
                    <li class="text-sm text-gray-300">{"diferença entre há e a"}</li>
                    <li class="text-sm text-gray-300">{"significado de a"}</li>
                </ul>
            </div>
        </div>
    }
}
