use yew::prelude::*;

#[component]
pub fn BannerComponent() -> Html {
    let banner_url = "public/img/kuruminha_banner1.png";


    html! {
        <section class="md:min-w-[500px] md:max-w-[700px] min-w-[200px] max-w-[400px]">
            <img class="w-full h-full" src={banner_url} alt="kuruminha_banner"/>
        </section>
    }
}
