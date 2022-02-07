use yew::prelude::*;

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
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        if self.is_clicked {
            html! {
              <div class="flex justify-center">
                <button onclick={link.callback(|_| Msg::Clicked)}>{ "Click" }</button>
              </div>
            }
        } else {
            html! {
              <div class="flex justify-center">
                <h1 class="font-bold text-6xl text-red-600">
                    {"Clicked"}
                </h1>
              </div>
            }
        }
    }
}


fn main() {
    yew::start_app::<Model>();
}
