use yew::{html, Children, Component, Context, Html, Properties};
use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::description::Description;

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
            <div class="h-screen bg-red-100">
              <Header />
              <main class="container mx-auto">
                <div class="flex flex-col justify-center items-center">
                  <Description />
                  <div class="p-2 m-4 h-auto flex flex-row">
                    { for ctx.props().children.iter() }
                  </div>
                </div>
              </main>
            </div>
            <Footer />
          </>
        }
    }
}
