use yew::prelude::*;
use js_sys::Math;

#[component]
pub fn BannerComponent() -> Html {
    let banner_url = vec![
        "public/img/kuruminha_banner1.png",
        "public/img/kuruminha_banner2.jpg",
        "public/img/kuruminha_banner3.jpg",
    ];
    let index = (Math::random() * banner_url.len() as f64) as usize;
    let random_banner = banner_url[index];

    html! {
        <section class="md:min-w-[500px] md:max-w-[700px] md:max-h-[400px] max-h-[200px] min-w-[200px] max-w-[400px]">
            <img class="w-full h-full" src={random_banner} alt="kuruminha_banner"/>
        </section>
    }
}
