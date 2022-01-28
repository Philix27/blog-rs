use std::collections::HashMap;

use blog_common::dto::post::{PostData, PostDetail};
use blog_common::dto::Response;
use gloo_file::callbacks::FileReader;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use weblog::*;
use yew::events::InputEvent;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::{self, AnyRoute};

use crate::component::Unauthorized;

#[wasm_bindgen(module = "/asset/editor.js")]
extern "C" {
    #[wasm_bindgen(js_name = getContent)]
    fn get_content() -> String;
    #[wasm_bindgen(js_name = setContent)]
    fn set_content(c: JsValue);
    #[wasm_bindgen(js_name = inputTag)]
    fn input_tag(event: web_sys::KeyboardEvent);
    #[wasm_bindgen(js_name = showOriginTags)]
    fn show_origin_tags(tags: Vec<JsValue>);
    #[wasm_bindgen(js_name = getAddedTags)]
    fn get_added_tags() -> Vec<JsValue>;
    #[wasm_bindgen(js_name = randomTitleImage)]
    fn random_title_image(event: MouseEvent, id: u64, payload_callback: JsValue);
    #[wasm_bindgen(js_name = uploadTitleImage)]
    fn upload_title_image(event: Event, post_id: u64, files: Vec<web_sys::File>, payload_callback: JsValue);
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct UpdatePostProps {
    onsubmit: Callback<FocusEvent>,
    onchange: Callback<Event>,
    download_image: Callback<MouseEvent>,
    oninput: Callback<InputEvent>,
    onupdate: Callback<MouseEvent>,
    go_back: Callback<MouseEvent>,
    post_id: u64,
    title_onchange: Callback<String>,
    title_image_onchange: Callback<String>,
    set_content: Callback<String>,
    set_tags: Callback<Option<Vec<String>>>,
}

#[function_component(UpdatePost)]
fn update_post(
    UpdatePostProps {
        onsubmit,
        onchange,
        download_image,
        oninput,
        onupdate,
        go_back,
        post_id,
        title_onchange,
        title_image_onchange,
        set_content,
        set_tags,
    }: &UpdatePostProps,
) -> Html {
    let detail_url = format!("/post/show/{}?edit=true", post_id);
    let post_detail = use_state(|| None::<PostDetail>);
    {
        let post_detail = post_detail.clone();
        use_effect_with_deps(
            move |_| {
                let post_detail = post_detail.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response: Response<PostDetail> = reqwasm::http::Request::get(&detail_url)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    if response.status == 0 {
                        post_detail.set(Some(response.data.unwrap()));
                    } else {
                        post_detail.set(Some(PostDetail::default()));
                    }
                });
                || ()
            },
            (),
        );
    }
    if post_detail.is_none() {
        return html! {};
    }
    let mut post_detail = (*post_detail).clone().unwrap();
    if post_detail.id < 1 {
        return html! {
          <Unauthorized />
        };
    }
    title_onchange.emit(post_detail.title.clone());
    if post_detail.title_image.len() > 0 {
        title_image_onchange.emit(post_detail.title_image.clone());
    }
    let mut content = String::new();
    std::mem::swap(&mut post_data.content, &mut content);
    set_content.emit(content);
    if post_detail.tags.is_some() {
        set_tags.emit(Some(post_detail.tags.unwrap()));
    } else {
        set_tags.emit(None);
    }
    html! {
      <>
        <div class="container">
          <h1 class="title is-1">{"编辑博客/Editing post"}</h1>
        </div>
        <p>{" "}</p>
        <form class="row g-3" onsubmit={onsubmit}>
          <div class="container">
            <div class="field">
              <label class="label">{"题图/Image"}</label>
            </div>
            <nav class="level">
            <p class="level-item has-text-centered">
              {""}
            </p>
            <p class="level-item has-text-centered">
              <div class="file is-normal">
                <label class="file-label">
                  <input class="file-input" multiple=false accept="image/*" type="file" name="title-image" onchange={onchange}/>
                  <span class="file-cta">
                    <span class="file-icon"><i class="fas fa-upload"></i></span>
                    <span class="file-label">{"上传图片/Upload"}</span>
                  </span>
                </label>
              </div>
            </p>
            <p class="level-item has-text-centered">{"或/Or"}</p>
            <p class="level-item has-text-centered">
              <button class="button">// onclick={download_image}
                <span class="icon"><i class="fas fa-download"></i></span>
                <span>{"下载一张/Download"}</span>
              </button>
            </p>
            <p class="level-item has-text-centered">{""}</p>
          </nav>
        </div>
        <section class="hero is-large is-light has-background">
          <img id="title-image" src={post_detail.title_image.clone()} class="hero-background is-transparent"/>
          <div class="hero-body"></div>
        </section>
        <div class="container">
          <div class="field">
            <label class="label">{"标题/Title"}</label>
            <div class="control">
              <input class="input" type="text" value={post_detail.title.clone()}
                  oninput={oninput}/>
            </div>
          </div>
          <div class="field">
            <label class="label">{"内容/Content"}</label>
            <div id="post-content">{&post_detail.content}</div>
            <iframe id="editor" width="100%" height="500" src="/asset/editor.html"></iframe>
          </div>
          <div class="field">
            <label class="label">{"标签/Labels"}</label>
              <div class="control">
                <input maxlength="10" id="tagInput" class="input" type="text" placeholder="回车添加/Press 'Enter' to add" onkeyup={input_tag}/>
              </div>
              <br/>
              <div id="tags" class="tags"></div>
          </div>
          <div class="field is-grouped">
            <div class="control">
              <button class="button is-link" onclick={onupdate}>{ "更新/Update" }</button>
            </div>
            <div class="control">
              <button class="button is-link is-light" onclick={go_back}>{ "返回/Cancel" }</button>
            </div>
            </div>
          </div>
        </form>
      </>
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub post_id: u64,
}

pub struct PostCompose {
    post_id: u64,
    title: String,
    title_image: String,
    content: String,
    tags: Option<Vec<String>>,
    readers: HashMap<String, FileReader>,
}

pub enum Msg {
    // RequestPostData(u64),
    Ignore,
    UpdateTitle(String),
    UpdatePost,
    LoadedBytes(String, Vec<u8>),
    Files(Event, Vec<web_sys::File>),
    RetrieveRandomTitleImage(MouseEvent),
    GoBack,
    GoSignIn,
    SetContent(String),
    SetTags(Option<Vec<String>>),
    PayloadCallback(String),
}

impl Component for PostCompose {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            post_id: ctx.props().post_id,
            title: String::new(),
            title_image: String::new(),
            content: String::new(),
            tags: None,
            readers: HashMap::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PayloadCallback(s) => {
                self.title_image = s;
            },
            Msg::Ignore => {},
            Msg::UpdateTitle(s) => self.title = s,
            Msg::UpdatePost => {
                let selected_tags = get_added_tags();
                let tags = if selected_tags.is_empty() {
                    None
                } else {
                    Some(selected_tags.iter().map(|v| v.as_string().unwrap()).collect())
                };
                let post_data = PostData {
                    id: self.post_id as i64,
                    title: self.title.clone(),
                    title_image: self.title_image.clone(),
                    content: get_content(),
                    tags,
                };
                console_log!(&post_data.content);
                let navigator = ctx.link().navigator().unwrap();
                let payload = serde_json::to_string(&post_data).unwrap();
                let post_id = self.post_id;
                wasm_bindgen_futures::spawn_local(async move {
                    let _response: Response<PostDetail> = reqwasm::http::Request::post("/post/save")
                        .header("Content-Type", "application/json")
                        .body(payload)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    navigator.push(crate::router::Route::ShowPost { id: post_id });
                });

                // self.blog_params.tags = Some(get_selected_tags().iter().map(|e| e.as_string().unwrap()).collect());
                // let fetch_task = request::post::<NewPost, PostDetail>(
                //     val::BLOG_SAVE_URI,
                //     self.blog_params.clone(),
                //     self.response.clone(),
                // );
                // self.fetch_task = Some(fetch_task);
            },
            // Msg::Response(Ok::<PostDetail, _>(blog)) => {
            //     self.fetch_task = None;
            //     self.router_agent.send(ChangeRoute(AppRoute::PostShow(blog.id).into()));
            // },
            // Msg::Response(Err::<_, Error>(err)) => {
            //     self.error = Some(err);
            //     self.fetch_task = None;
            //     return true;
            // },
            // Msg::TagsResponse(Ok::<Vec<String>, _>(tags)) => {
            //     ConsoleService::log(tags.len().to_string().as_str());
            //     self.fetch_task = None;
            //     self.all_tags = tags;
            //     return true;
            // },
            // Msg::TagsResponse(Err::<_, Error>(err)) => {
            //     ConsoleService::log("error");
            //     self.error = Some(err);
            //     self.fetch_task = None;
            //     return true;
            // },
            Msg::LoadedBytes(file_name, data) => {
                let info = format!("file_name: {}, data: {:?}", file_name, data);
                console_log!(&info);
                // wasm_bindgen_futures::spawn_local(async move {
                //     let response = reqwasm::http::Request::post("").body(&data.as_slice()).send().await.unwrap();
                // });
                self.readers.remove(&file_name);
            },
            Msg::Files(event, files) => {
                // for file in files.into_iter() {
                // let file_name = file.name();
                // let task = {
                //     let file_name = file_name.clone();
                //     let link = ctx.link().clone();
                //     gloo_file::callbacks::read_as_bytes(&file, move |res| {
                //         link.send_message(Msg::LoadedBytes(
                //             file_name,
                //             res.expect("failed to read file"),
                //         ))
                //     })
                // };
                // self.readers.insert(file_name, task);
                // }
                let callback = ctx.link().callback(Msg::PayloadCallback);
                let js_callback = Closure::once_into_js(move |payload: String| callback.emit(payload));
                upload_title_image(event, self.post_id, files, js_callback);
            },
            Msg::RetrieveRandomTitleImage(event) => {
                let callback = ctx.link().callback(Msg::PayloadCallback);
                let js_callback = Closure::once_into_js(move |payload: String| callback.emit(payload));
                random_title_image(event, self.post_id, js_callback);
            },
            Msg::SetContent(c) => {
                self.content = c;
            }
            Msg::SetTags(tags) => {
                self.tags = tags;
            }
            Msg::GoBack => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.push(crate::router::Route::ShowPost {id: self.post_id});
            },
            Msg::GoSignIn => {
                let any_route = AnyRoute::new(String::from("/401"));
                let continue_url = crate::router::Route::ComposePost { id: self.post_id }.to_path();
                let query = HashMap::from([(".continue", continue_url.as_str())]);
                let navigator = ctx.link().navigator().unwrap();
                navigator.push_with_query(any_route, query);
            },
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let changed = self.post_id != ctx.props().post_id;
        if changed {
            self.post_id = ctx.props().post_id;
        }
        changed
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let Self { post_id, title, title_image, readers } = self;
        let post_id = self.post_id;

        let title_onchange = ctx.link().callback(move |title: String| Msg::UpdateTitle(title));
        let title_image_onchange = ctx.link().callback(move |s: String| Msg::PayloadCallback(s));

        let onsubmit = ctx.link().callback(|ev: FocusEvent| {
            ev.prevent_default();
            Msg::Ignore
        });
        let onchange = ctx.link().callback(move |e: Event| {
            let mut result = Vec::new();
            let input: HtmlInputElement = e.target_unchecked_into();

            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
              .unwrap()
              .unwrap()
              .map(|v| web_sys::File::from(v.unwrap()))
              // .map(File::from)
              ;
                result.extend(files);
            }
            Msg::Files(e, result)
        });
        let download_image = ctx.link().callback(|e: MouseEvent| Msg::RetrieveRandomTitleImage(e));
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            Msg::UpdateTitle(input.value())
        });
        let onupdate = ctx.link().callback(|_: MouseEvent| Msg::UpdatePost);
        let go_back = ctx.link().callback(|_: MouseEvent| Msg::GoBack);
        let set_content = Callback::from(|c: String| Msg::SetContent(c));
        let set_tags = Callback::from(|tags: Option<Vec<String>>| Msg::SetTags(tags));

        html! {
          <>
            <UpdatePost onsubmit={onsubmit} onchange={onchange} {download_image} oninput={oninput} onupdate={onupdate}
            {go_back} post_id={post_id as u64} title_onchange={title_onchange.clone()}
            title_image_onchange={title_image_onchange.clone()} {set_content} {set_tags} />
          </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if self.tags.is_some() {
            let origin_tags = self.tags.unwrap().iter().map(|t| JsValue::from_str(t)).collect();
            show_origin_tags(origin_tags);
        }
        set_content(JsValue::from_str(&self.content));
    }

}
