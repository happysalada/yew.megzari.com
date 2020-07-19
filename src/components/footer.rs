use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
        <footer class="footer">
          <div class="h-full my-auto mx-auto overflow-hidden">
            <p class="text-center text-base leading-6 text-gray-400">
              { "©2077 Raphael Megzari." }
              <br class="xl:hidden" />
              { "NO rights reserved." }
            </p>
          </div>
        </footer>
        }
    }
}
