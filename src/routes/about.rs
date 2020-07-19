use yew::prelude::*;

/// About page
pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        About {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
        <div class="content-grid">
          <div
            class="content w-full text-xl leading-normal text-white"
            style="font-family: Georgia, serif;"
          >
            <div style="height: 80vh;">
              <div class="font-sans">
                <h1
                  class="font-sans break-normal text-white pt-6 pb-2 text-3xl
                  md:text-4xl"
                >
                  { "About me" }
                </h1>
              </div>
              <p class="py-6">
                { "I'm a developper who thinks of programming as a form of Art!" }
                <br />
                <br />
                { "I'm quite interested in the biotech space. contact me if there is
                something you want to make." }
                <br />
                <br />
                { "raphael at megzari dot com" }
                <br />
                <br />
                { "Here is my portfolio" }
              </p>
            </div>
            // {#each projects as project}
            //   <Project {project} />
            // {/each}
          </div>
        </div>
                }
    }
}
