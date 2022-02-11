use yew::{html, Children, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
}

pub struct Layout;

impl Component for Layout {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <>
            <header class="py-2">
              <div  class="py-4 grid place-items-center">
                <div class="text-center hover:opacity-50">
                  <h1 class="text-6xl font-mono text-pink-800 font-festive">
                    <a href="https://github.com/yu1hpa/valentine2022">{ "Valentine's Day 2022" }</a>
                  </h1>
                </div>
              </div>
            </header>
            <main>
              <div class="container mx-auto flex flex-col justify-center items-center">
                { for ctx.props().children.iter() }
              </div>
            </main>
            <footer class="py-2 border-t border-gray-300 text-center text-gray-600">
              <span>{ "Copyright©2022: yu1hpa" }</span>
            </footer>
          </>
        }
    }
}
