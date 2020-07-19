use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

/// Nav component
pub struct Nav {
    link: ComponentLink<Self>,
    menu_open: bool,
    route: Route,
    _route_agent_bridge: RouteAgentBridge,
}

pub enum Msg {
    ToggleMenu,
    NewRoute(Route),
}

impl Nav {
    pub fn home_classes(&self) -> String {
        if let Some(AppRoute::Home) = AppRoute::switch(self.route.clone()) {
            "nav-active"
        } else {
            "nav-inactive"
        }
        .to_owned()
            + " nav-default"
    }

    pub fn about_classes(&self) -> String {
        if let Some(AppRoute::About) = AppRoute::switch(self.route.clone()) {
            "nav-active"
        } else {
            "nav-inactive"
        }
        .to_owned()
            + " nav-default"
    }

    pub fn blog_classes(&self) -> String {
        if let Some(AppRoute::Blog) = AppRoute::switch(self.route.clone()) {
            "nav-active"
        } else {
            "nav-inactive"
        }
        .to_owned()
            + " nav-default"
    }
}

impl Component for Nav {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::NewRoute);
        let route_agent_bridge = RouteAgentBridge::new(callback);
        let route_service: RouteService<()> = RouteService::new();
        let route = route_service.get_route();

        Nav {
            _route_agent_bridge: route_agent_bridge,
            link,
            route,
            menu_open: false,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::ToggleMenu => {
                self.menu_open = !self.menu_open;
            }
            Msg::NewRoute(route) => {
                self.route = route;
            }
        }
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        let toggle_menu_callback = self.link.callback(|_| Msg::ToggleMenu);
        html! {
        <nav class="nav">
        <div class="mx-auto px-2 sm:px-4 lg:px-8">
          <div class="relative flex items-center justify-between h-16">
            <div class="flex items-center px-2 lg:px-0">
              <div class="hidden lg:block lg:ml-6">
                <div class="flex space-x-4">
                  <RouterAnchor<AppRoute>
                    route=AppRoute::Home
                    classes=&self.home_classes()
                  >
                    { "Home"}
                  </RouterAnchor<AppRoute>>
                  <RouterAnchor<AppRoute>
                    route=AppRoute::About
                    classes=&self.about_classes()
                  >
                    { "About" }
                  </RouterAnchor<AppRoute>>
                  <RouterAnchor<AppRoute>
                    route=AppRoute::Blog
                    classes=&self.blog_classes()
                  >
                    { "Blog" }
                  </RouterAnchor<AppRoute>>
                </div>
              </div>
            </div>
            <div class="flex-1 flex justify-center px-2 lg:ml-6 lg:justify-end">
              // <div class="max-w-lg w-full">
              //   <label for="search" class="sr-only">Search</label>
              //   <div class="relative">
              //     <div
              //       class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none"
              //     >
              //       <svg
              //         class="h-5 w-5 text-gray-400"
              //         fill="currentColor"
              //         viewBox="0 0 20 20"
              //       >
              //         <path
              //           fill-rule="evenodd"
              //           d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
              //           clip-rule="evenodd"
              //         />
              //       </svg>
              //     </div>
              //     <input
              //       id="search"
              //       class="block w-full pl-10 pr-3 py-2 border border-transparent rounded-md leading-5 bg-gray-700 placeholder-gray-400 focus:outline-none focus:bg-white sm:text-sm transition duration-150 ease-in-out"
              //       placeholder="Search"
              //     />
              //   </div>
              // </div>
            </div>
            <div class="flex lg:hidden">
              <button
                onclick=&toggle_menu_callback
                class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none focus:bg-gray-700 focus:text-white transition duration-150 ease-in-out"
              >
                <svg class="h-6 w-6" stroke="currentColor" fill="none" viewBox="0 0 24 24">
                  <path
                    class=if self.menu_open { "hidden" } else { "inline-flex" }
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 6h16M4 12h16M4 18h16"
                  />
                  <path
                    class=if self.menu_open { "inline-flex" } else { "hidden" }
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            </div>
          </div>
        </div>
        <div class=if self.menu_open { "block" } else { "hidden" }.to_owned() + " lg:hidden">
          <div class="px-2 pt-2 pb-3 flex flex-col space-y-1">
            <RouterAnchor<AppRoute>
              route=AppRoute::Home
              classes=&self.home_classes()
            >
              { "Home"}
            </RouterAnchor<AppRoute>>
            <RouterAnchor<AppRoute>
              route=AppRoute::About
              classes=self.about_classes()
            >
              { "About" }
            </RouterAnchor<AppRoute>>
            <RouterAnchor<AppRoute>
              route=AppRoute::Blog
              classes=self.blog_classes()
            >
              { "Blog" }
            </RouterAnchor<AppRoute>>
          </div>
        </div>
        </nav>
                }
    }
}
