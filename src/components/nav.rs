use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Nav component
pub struct Nav {
    link: ComponentLink<Self>,
    state: State,
}

pub struct State {
    menu_open: bool,
}

pub enum Msg {
    ToggleMenu,
    CloseMenu,
}

impl Component for Nav {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Nav {
            link,
            state: State { menu_open: false },
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::CloseMenu => {
                self.state.menu_open = false;
                true
            }
            Msg::ToggleMenu => {
                self.state.menu_open = !self.state.menu_open;
                true
            }
        }
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        let close_menu_callback = self.link.callback(|_| Msg::CloseMenu);
        let toggle_menu_callback = self.link.callback(|_| Msg::ToggleMenu);
        html! {
          <nav class="bg-gray-800">
            <div class="max-w-7xl mx-auto px-2 sm:px-4 lg:px-8">
              <div class="relative flex items-center justify-between h-16">
                <div class="flex items-center px-2 lg:px-0">
                  <div class="flex-shrink-0">
                    <img class="block lg:hidden h-8 w-auto" src="/img/logos/workflow-mark-on-dark.svg" alt="" />
                    <img class="hidden lg:block h-8 w-auto" src="/img/logos/workflow-logo-on-dark.svg" alt="" />
                  </div>
                  <div class="hidden lg:block lg:ml-6">
                    <div class="flex">
                      <RouterAnchor<AppRoute>
                        route=AppRoute::Home
                        classes="px-3 py-2 rounded-md text-sm leading-5 font-medium text-white bg-gray-900 focus:outline-none focus:text-white focus:bg-gray-700 transition duration-150 ease-in-out"
                      >
                        { "Home" }
                      </RouterAnchor<AppRoute>>
                      <RouterAnchor<AppRoute>
                        route=AppRoute::About
                        classes="ml-4 px-3 py-2 rounded-md text-sm leading-5 font-medium text-gray-300 hover:text-white hover:bg-gray-700 focus:outline-none focus:text-white focus:bg-gray-700 transition duration-150 ease-in-out"
                      >
                        { "About" }
                      </RouterAnchor<AppRoute>>
                    </div>
                  </div>
                </div>
                <div class="flex-1 flex justify-center px-2 lg:ml-6 lg:justify-end">
                  <div class="max-w-lg w-full lg:max-w-xs">
                    <label for="search" class="sr-only">{ "Search" }</label>
                    <div class="relative">
                      <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <svg class="h-5 w-5 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                          <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
                        </svg>
                      </div>
                      <input
                        id="search"
                        class="block w-full pl-10 pr-3 py-2 border border-transparent rounded-md leading-5 bg-gray-700 placeholder-gray-400 focus:outline-none focus:bg-white sm:text-sm transition duration-150 ease-in-out"
                        placeholder="Search"
                        autofocus=true
                      />
                    </div>
                  </div>
                </div>
                <div class="flex lg:hidden">
                  <button
                    onclick=&toggle_menu_callback
                    class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none focus:bg-gray-700 focus:text-white transition duration-150 ease-in-out"
                  >
                    <svg class="h-6 w-6" stroke="currentColor" fill="none" viewBox="0 0 24 24">
                      <path
                        class=if self.state.menu_open { "hidden" } else { "inline-flex" }
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M4 6h16M4 12h16M4 18h16" />
                      <path
                        class=if self.state.menu_open { "inline-flex" } else { "hidden" }
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
                <div class="hidden lg:block lg:ml-4">
                  <div class="flex items-center">
                    <div
                      onclick=&close_menu_callback
                      class="ml-4 relative flex-shrink-0"
                    >
                      <div
                        x-transition_enter="transition ease-out duration-100"
                        x-transition_enter-start="transform opacity-0 scale-95"
                        x-transition_enter-end="transform opacity-100 scale-100"
                        x-transition_leave="transition ease-in duration-75"
                        x-transition_leave-start="transform opacity-100 scale-100"
                        x-transition_leave-end="transform opacity-0 scale-95"
                        class=if self.state.menu_open { "origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg"} else { "hidden" }
                      >
                        <div class="py-1 rounded-md bg-white shadow-xs">
                          <a href="#" class="block px-4 py-2 text-sm leading-5 text-gray-700 hover:bg-gray-100 focus:outline-none focus:bg-gray-100 transition duration-150 ease-in-out">{ "Your Profile" }</a>
                          <a href="#" class="block px-4 py-2 text-sm leading-5 text-gray-700 hover:bg-gray-100 focus:outline-none focus:bg-gray-100 transition duration-150 ease-in-out">{ "Settings" }</a>
                          <a href="#" class="block px-4 py-2 text-sm leading-5 text-gray-700 hover:bg-gray-100 focus:outline-none focus:bg-gray-100 transition duration-150 ease-in-out">{ "Sign out" }</a>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div
              class=if self.state.menu_open { "lg:hidden block" } else { "hidden lg:hidden" }
            >
              <div class="px-2 pt-2 pb-3">
                <RouterAnchor<AppRoute>
                  route=AppRoute::Home
                  classes="block px-3 py-2 rounded-md text-base font-medium text-white bg-gray-900 focus:outline-none focus:text-white focus:bg-gray-700 transition duration-150 ease-in-out"
                >
                  { "Home" }
                </RouterAnchor<AppRoute>>
                <RouterAnchor<AppRoute>
                  route=AppRoute::About
                  classes="mt-1 block px-3 py-2 rounded-md text-base font-medium text-gray-300 hover:text-white hover:bg-gray-700 focus:outline-none focus:text-white focus:bg-gray-700 transition duration-150 ease-in-out"
                >
                  { "About" }
                </RouterAnchor<AppRoute>>
              </div>
            </div>
          </nav>
        }
    }
}
