use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};

use crate::utils::NeqAssign;


#[derive(PartialEq, Clone, Properties)]
pub struct HeaderProps<T: 'static + ToString + Clone + Default + PartialEq> {
    #[props(required)]
    pub text: T,
}

pub struct Header<T: 'static + ToString+ Clone + Default+ PartialEq> {
    props: HeaderProps<T>,
}

impl<T: 'static + ToString+ Clone + Default + PartialEq> Component for Header<T> {
    type Message = ();
    type Properties = HeaderProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <div class="header"><h1>{ self.props.text.to_string() }</h1></div>
        }
    }

}
