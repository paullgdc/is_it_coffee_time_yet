pub mod components;
pub mod services;
pub mod utils;

use std::time::Duration;
use stdweb::web::Date;
use yew::{
    html,
    services::timeout::{TimeoutService, TimeoutTask},
    Component, ComponentLink, Html, ShouldRender,
};

const NUMBER_OF_IMAGES: usize = 3;
const COFFEE_TIME: i32 = 13;

struct App {
    timeout: TimeoutTask,
    current_hour: i32,
    link: ComponentLink<Self>,
}

enum Msg {
    RefreshTime,
}

impl App {
    fn refresh_timeout(&mut self) {
        self.timeout = TimeoutService::new().spawn(
            Duration::from_secs(60),
            self.link.callback(|_| Msg::RefreshTime),
        );
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let timeout = TimeoutService::new()
            .spawn(Duration::from_secs(60), link.callback(|_| Msg::RefreshTime));
        App {
            timeout,
            current_hour: Date::new().get_hours(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RefreshTime => {
                let old_hour = self.current_hour;
                self.current_hour = Date::new().get_hours();
                self.refresh_timeout();
                self.current_hour != old_hour
            }
        }
    }

    fn view(&self) -> Html {
        let image = (services::random() * NUMBER_OF_IMAGES as f64) as usize;
        html! {
            <>
                <components::Header::<&'static str>
                    text="Is it coffee time yet?"
                />
                <h2>{if self.current_hour == COFFEE_TIME { "Yes" } else { "No" }}</h2>
                <div class="centered-section">
                    <img class="image"
                        src={format!("public/coffee_cups/{}.jpg", image)}
                        alt="Coffee cup"
                    ></img>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
