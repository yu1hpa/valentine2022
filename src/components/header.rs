use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <header class="py-2">
            <div  class="py-4 grid place-items-center">
              <div class="text-center hover:opacity-50">
                <h1 class="text-6xl font-mono text-pink-800 font-festive">
                  <a href="https://github.com/yu1hpa/valentine2022">{ "Valentine's Day 2022" }</a>
                </h1>
              </div>
            </div>
          </header>
        }
    }
}
