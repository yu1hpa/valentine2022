use yew::{html, Component, Context, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct Description;

impl Component for Description {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
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
        }
    }
}
