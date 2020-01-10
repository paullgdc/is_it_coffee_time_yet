pub mod components;
pub mod utils;
pub mod services;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use stdweb::web::Date;

const NUMBER_OF_IMAGES: usize = 3;

struct App {
}


impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let hour = Date::new().get_hours();
        let image = (services::random() * NUMBER_OF_IMAGES as f64) as usize;
        html! {
            <>
                <components::Header::<&'static str>
                    text="Is it coffee time yet?"
                />
                <h2>{if hour == 13 { "Yes" } else { "No" }}</h2>
                <div class="centered-section">
                    <img class="image" src={format!("public/coffee_cups/{}.jpg", image)} alt="Coffee cup"></img>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
