use yew::{Children, Component, Context, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props{
    #[prop_or_default]
    pub children: Children,
}

pub struct Emoji;

impl Component for Emoji {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <>
            <h1 class="text-bold text-6xl">
              { for ctx.props().children.iter() }
            </h1>
          </>
        }
    }
}
