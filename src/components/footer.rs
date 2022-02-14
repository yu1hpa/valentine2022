use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <footer class="py-2 border-t border-gray-300 text-center text-gray-600">
            <span>{ "Copyright©2022: yu1hpa" }</span>
          </footer>
        }
    }
}
