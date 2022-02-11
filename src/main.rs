use yew::prelude::*;

mod components;
use components::layout::Layout;

enum Msg {
    Clicked,
}

struct Model {
    is_clicked: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_clicked: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                self.is_clicked = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let content = if self.is_clicked {
            html! {
              <div>
                <button onclick={ctx.link().callback(|_| Msg::Clicked)}>{ "Click" }</button>
              </div>
            }
        } else {
            html! {
              <h1 class="font-bold text-6xl text-red-600">
                {"Clicked"}
              </h1>
            }
        };
        html! {
            <Layout>
              { content }
            </Layout>
        }
    }
}


fn main() {
    yew::start_app::<Model>();
}
