use crate::components::tag::Tag;
use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use web_sys::Node;
use yew::format::{Nothing, Yaml};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::virtual_dom::VNode;

pub struct PostPage {
    post: Option<Post>,
    slug: String,
    _fetch_task: FetchTask,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
    title: String,
    tags: Vec<Tag>,
    html: String,
    outline: String,
    published_at: String,
    slug: String,
}

pub enum Msg {
    FetchReady(Result<Vec<Post>, Error>),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub slug: String,
}

impl Component for PostPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
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
        PostPage {
            post: None,
            slug: props.slug,
            _fetch_task: fetch_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(fectched_data) => fectched_data.map_or_else(
                |err| log::error!("{}", err),
                |posts| {
                    self.post = posts
                        .iter()
                        .find(|Post { slug, .. }| *slug == self.slug)
                        .cloned()
                },
            ),
        }
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
          { self.post.as_ref().map_or_else(
            || html! { "No Post not found"},
            |post| {
            let Post {
                title,
                tags,
                published_at,
                html,
                ..
            } = post;
            // trick to render raw html
            // see examples/inner_html for more details
            let js_html = {
                let div = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("div")
                    .unwrap();
                div.set_inner_html(html);
                div
            };
            let node = Node::from(js_html);
            let vnode = VNode::VRef(node);
            html! {
              <div class="content-grid">
                <div
                  class="content w-full text-xl leading-normal text-white"
                  style="font-family: Georgia, serif;"
                >
                  <div class="font-sans">
                    <h1
                      class="font-sans break-normal text-white pt-6 pb-2 text-3xl md:text-4xl"
                    >
                      {title}
                    </h1>
                    <p class="text-sm md:text-base font-normal text-gray-500">
                      { "Published " }{ published_at }
                    </p>
                    <p class="text-sm font-normal text-gray-500">
                      { "Tags: " }
                      { for tags.iter().map(|tag| {
                      html!{  <Tag tag=tag /> }
                      } )}
                    </p>
                  </div>
                  {
                    vnode
                  }
                </div>
              </div>
            }})
          }
        }
    }
}
