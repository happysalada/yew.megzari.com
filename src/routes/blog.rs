use yew::prelude::*;

/// Blog page
pub struct Blog;

impl Component for Blog {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Blog {}
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
                <div class="font-sans">
                  <h1 class="font-sans break-normal text-white pt-6 pb-2 text-3xl md:text-4xl">
                    { "Recent posts" }
                  </h1>

                  <ul>
                    // {#each posts as {title, tags, outline, slug}}
                    // <li>
                    //   <a class="text-blue-500 space-y-3" rel="prefetch" href="blog/{slug}">
                    //     <h2>
                    //       {title}
                    //     </h2>

                    //     <p class="text-white text-base">
                    //       {outline}
                    //     </p>

                    //     <p class="text-sm font-normal text-gray-500">
                    //       Tags: {#each tags as tag}
                    //         <Tag { tag }/>
                    //       {/each}
                    //     </p>
                    //   </a>
                    // </li>
                    // {/each}
                  </ul>
                </div>
          </div>
        </div>
        }
    }
}
