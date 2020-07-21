use crate::components::tag::Tag;
use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use yew::format::{Nothing, Yaml};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

/// About page
pub struct About {
    projects: Option<Vec<Project>>,
    _fetch_task: FetchTask,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Client {
    name: String,
    url: String,
    caption: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    title: String,
    description: String,
    client: Client,
    tags: Vec<Tag>,
}

pub enum Msg {
    FetchReady(Result<Vec<Project>, Error>),
}

impl Component for About {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(
            move |response: Response<Yaml<Result<Vec<Project>, Error>>>| {
                let (meta, Yaml(data)) = response.into_parts();
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::FetchReady(Err(anyhow!("Fetch failed META {:?}, {:?}", meta, data)))
                }
            },
        );
        let request = Request::get("/projects.yaml").body(Nothing).unwrap();
        let fetch_task = FetchService::fetch_binary(request, callback).unwrap();
        About {
            projects: None,
            _fetch_task: fetch_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(fectched_data) => fectched_data.map_or_else(
                |err| log::error!("{}", err),
                |projects| self.projects = Some(projects),
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
            { self.render_projects() }
          </div>
        </div>
                }
    }
}

impl About {
    fn render_projects(&self) -> Html {
        self.projects.as_ref().map_or_else(
            || html! { "No projects"},
            |projects| projects.iter().map(render_project).collect(),
        )
    }
}

fn render_project(project: &Project) -> Html {
    let Project {
        client,
        description,
        title,
        tags,
    } = project;
    html! {
    <div
      class="flex flex-col justify-center text-center my-auto mx-auto"
      style="height: 80vh;"
    >
      <h2
        class="text-4xl tracking-tight leading-10 font-extrabold text-white mt-3 md:mt-10"
      >
        { title }
      </h2>
      <p class="mt-3 md:mt-10 text-base md:text-xl">
        { "For: " }
        <a
          href=client.url.as_str()
          class="leading-6 text-blue-500 hover:text-blue-300"
        >
          { client.name.as_str() }
        </a>
        { " - " } { client.caption.as_str() }
      </p>

      <p class="mt-3 md:mt-10 text-base md:text-xl">
        { description }
      </p>
      <p class="mt-3 md:mt-10 text-sm font-normal text-gray-500">
        { "Tags: " }
        { for tags.iter().map(|tag| {
          html!{  <Tag tag=tag /> }
        } )}
      </p>
    </div>
            }
}
