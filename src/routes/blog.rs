use crate::components::tag::Tag;
use crate::routes::AppRoute;
use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use yew::format::{Nothing, Yaml};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew_router::prelude::*;

/// Blog page
pub struct Blog {
    posts: Option<Vec<Post>>,
    _fetch_task: FetchTask,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
    title: String,
    tags: Vec<Tag>,
    outline: String,
    slug: String,
}

pub enum Msg {
    FetchReady(Result<Vec<Post>, Error>),
}

impl Component for Blog {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(move |response: Response<Yaml<Result<Vec<Post>, Error>>>| {
            let (meta, Yaml(data)) = response.into_parts();
            if meta.status.is_success() {
                Msg::FetchReady(data)
            } else {
                Msg::FetchReady(Err(anyhow!("Fetch failed META {:?}, {:?}", meta, data)))
            }
        });
        let request = Request::get("/posts.yaml").body(Nothing).unwrap();
        let fetch_task = FetchService::fetch_binary(request, callback).unwrap();
        Blog {
            posts: None,
            _fetch_task: fetch_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(fectched_data) => fectched_data.map_or_else(
                |err| log::error!("{}", err),
                |posts| self.posts = Some(posts),
            ),
        }
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
                  { self.render_posts() }
                  </ul>
                </div>
          </div>
        </div>
        }
    }
}

impl Blog {
    fn render_posts(&self) -> Html {
        self.posts.as_ref().map_or_else(
            || html! { "No posts"},
            |posts| posts.iter().map(render_post).collect(),
        )
    }
}

fn render_post(post: &Post) -> Html {
    let Post {
        title,
        outline,
        tags,
        slug,
    } = post;

    html! {
        <li>
            <RouterAnchor<AppRoute>
                route=AppRoute::Post(slug.into())
                classes="text-blue-500 space-y-3"
                >
                <h2>
                    {title}
                </h2>

                <p class="text-white text-base">
                    {outline}
                </p>

                <p class="text-sm font-normal text-gray-500">
                    { "Tags: " }
                    { for tags.iter().map(|tag| {
                    html!{  <Tag tag=tag /> }
                    } )}
                </p>
            </RouterAnchor<AppRoute>>
        </li>
    }
}
