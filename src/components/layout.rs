use yew::{html, Children, Component, Context, Html, Properties};
use crate::components::header::Header;

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
                  <div class="xl:w-1/2 sm:w-2/3">
                    <p class="bg-green-200 rounded-md p-4 m-6">
                      { "🍫（チョコレート）🍰（ケーキ）🍭（キャンディー１）🍦（ソフトクリーム）
                      🍧（かき氷）🍩（ドーナツ）🍪（クッキー）🧁（カップケーキ）🥧（パイ）
                      🍬（キャンディー２）🍮（プリン）🥶（寒い顔）の12個が含まれてます" }
                    </p>
                    <p class="p-4 m-6">
                      { "🍫が唯一の" }
                      <span class="underline decoration-red-600">
                        { "当たり" }
                      </span>
                        { "で、他は" }
                      <span class="underline decoration-green-600">
                        { "ハズレ" }
                      </span>
                        { "で、🥶は" }
                      <span class="underline decoration-blue-600">
                        { "大ハズレです" }
                      </span>
                    </p>
                  </div>
                  <div class="p-2 m-4 h-auto flex flex-row">
                    { for ctx.props().children.iter() }
                  </div>
                </div>
              </main>
            </div>
            <footer class="py-2 border-t border-gray-300 text-center text-gray-600">
              <span>{ "Copyright©2022: yu1hpa" }</span>
            </footer>
          </>
        }
    }
}
