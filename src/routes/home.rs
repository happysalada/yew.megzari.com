use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Home page
pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
        <div class="h-full flex flex-col justify-center text-center my-auto mx-auto">
          <p class="text-base text-gray-400 md:text-xl">{ "Hi! I'm Raphael" }</p>
          <h2
            class="headline tracking-tight leading-10 font-extrabold text-white mt-3
            md:mt-10"
          >
            { "I" }
            <a
              href="https://github.com/happysalada"
              class="leading-6 text-blue-500 hover:text-blue-300"
            >
              { " code" }
            </a>
            <br class="xl:hidden" />
            { " ideas to life" }
          </h2>
          <p class="mt-3 md:mt-10 text-base text-gray-400 md:text-xl">
            { "welcome to my personal website." }
          </p>
          <div class="mt-5 max-w-md mx-auto sm:flex sm:justify-center md:mt-8">
            <div class="rounded-md shadow">
              <RouterAnchor<AppRoute>
                route=AppRoute::Blog
                classes="w-full flex items-center justify-center px-8 py-3 border-4
        border-blue-500 text-base text-black leading-6 font-medium rounded-md
        bg-blue-500 hover:bg-black hover:text-blue-500 focus:outline-none
        focus:shadow-outline-blue transition duration-150 ease-in-out md:py-4
        md:text-lg md:px-10"
                >
                { "Blog"}
              </RouterAnchor<AppRoute>>
            </div>
            <div class="rounded-md shadow mt-3 md:ml-3 md:mt-0">
              <RouterAnchor<AppRoute>
                route=AppRoute::About
                classes="w-full flex items-center justify-center px-8 py-3 border-4
        border-blue-500 text-base text-black leading-6 font-medium rounded-md
        bg-blue-500 hover:bg-black hover:text-blue-500 focus:outline-none
        focus:shadow-outline-blue transition duration-150 ease-in-out md:py-4
        md:text-lg md:px-10"
                >
                { "About"}
              </RouterAnchor<AppRoute>>
            </div>
          </div>
        </div>
                }
    }
}
