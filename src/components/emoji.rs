use yew::{Component, Context, html, Html, Properties};
use rand::seq::SliceRandom;

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Emoji;

impl Component for Emoji {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Emoji
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let emojies = vec!["🍫", "🍰", "🍭", "🍦",
                           "🍧", "🍩", "🍪", "🧁",
                           "🥧", "🍬", "🍮", "🥶"];
        let emoji: Vec<_> = emojies.choose_multiple(&mut rand::thread_rng(), 1).collect();
        html! {
          <h1 class="text-bold text-6xl">
            { emoji }
          </h1>
        }
    }
}
