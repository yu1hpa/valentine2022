use yew::prelude::*;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use rand::seq::SliceRandom;

mod components;
use components::layout::Layout;
use components::emoji::Emoji;

enum Msg {
    Clicked,
}

struct Model {
    is_clicked: bool,
    emoji: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_clicked: true,
            emoji: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                // if button clicked, change to "false"
                self.is_clicked = false;

                // Choose randomly
                let emojies = vec!["🍫", "🍰", "🍭", "🍦",
                                   "🍧", "🍩", "🍪", "🧁",
                                   "🥧", "🍬", "🍮", "🥶"];

                let one_emoji: Vec<_> = emojies
                                        .choose_multiple(&mut rand::thread_rng(), 1)
                                        .collect();
                self.emoji = one_emoji[0].to_string();

                // Set emoji to LocalStorage
                LocalStorage::set("emoji", self.emoji.clone()).ok();

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let emoji: String = LocalStorage::get("emoji").unwrap_or_default();
        let content = if emoji == "" {
            // true -> false
            if self.is_clicked {
                html! {
                  <div>
                    <button
                      class="bg-transparent hover:bg-green-500 text-green-700
                            font-semibold hover:text-white py-2 px-4
                            border border-green-500 hover:border-transparent rounded"
                      onclick={ctx.link().callback(|_| Msg::Clicked)}
                    >
                        { "引く" }
                    </button>
                  </div>
                }
            } else {
                html! {
                  // not going through here
                  // because emoji is stored, if button clicked
                }
            }
        } else {
            html! {
              <>
                <Emoji>{ emoji }</Emoji>
              </>
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
