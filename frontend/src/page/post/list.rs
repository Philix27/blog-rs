use blog_common::dto::post::PostDetail;
use blog_common::dto::Response;
use weblog::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

pub enum Msg {
    Compose,
}

pub struct PostList;

impl Component for PostList {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Compose => {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = reqwasm::http::Request::get("/post/new").send().await.unwrap();
                    let json: Response<u64> = response.json().await.unwrap();
                    if json.status == 0 {
                        ctx.link()
                            .history()
                            .unwrap()
                            .push(crate::router::Route::ComposePost { id: json.data.unwrap() });
                        // yew_router::push_route(crate::router::Route::ComposePost { id: json.data.unwrap() });
                    } else {
                        // ctx.link().location().unwrap().route().set_href("/management");
                        if let Some(loc) = web_sys::window().map(|window| window.location()) {
                            let _ = loc.set_href("/management");
                        } else {
                            console_log!("get location failed");
                        }
                    }
                });
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let page = self.current_page();
        let posts = use_state(|| vec![]);
        {
            let posts = posts.clone();
            use_effect_with_deps(
                move |_| {
                    let posts = posts.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let response: Response<Vec<PostDetail>> = reqwasm::http::Request::get("/post/list/1")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                        posts.set(response.data.unwrap());
                    });
                    || ()
                },
                (),
            );
        }
        html! {
            <>
                <div class="columns">
                    <div class="column is-right">
                    {"My Blog"}
                    </div>
                    <div class="column">
                    {""}
                    </div>
                    <div class="column">
                    {""}
                    </div>
                    <div class="column">
                        <button class="button" onclick={ctx.link().callback(|_| Msg::Compose)}>
                            <span class="icon">
                                <i class="fab fa-github"></i>
                            </span>
                            <span>{"写博客/Compose"}</span>
                        </button>
                    </div>
                </div>
                <h1 class="title is-1">{ "博客/Posts" }</h1>
                <h2 class="subtitle">{ "All of your quality writing in one place" }</h2>
                <div class="columns">
                    <div class="column">
                        <ul class="list">
                            { for posts.by_ref().take(posts.len() / 2 + posts.len() % 2) }
                        </ul>
                    </div>
                    <div class="column">
                        <ul class="list">
                            { for posts }
                        </ul>
                    </div>
                </div>
                <div class="container">
                    <nav class="pagination is-right" role="navigation" aria-label="pagination">
                        <a class="pagination-previous">
                            {"上一页/Previous"}
                        </a>
                        <a class="pagination-next">
                            {"下一页/Next page"}
                        </a>
                    </nav>
                </div>
            </>
        }
    }
}

impl PostList {
    fn render_post(&self, post_detail: &PostDetail) -> Html {
        html! {
            <>
                <div class="card">
                    <div class="card-image">
                        <figure class="image is-2by1">
                            <img src={post_detail.title_image.clone()} loading="lazy" />
                        </figure>
                    </div>
                    <div class="card-content">
                        <Link<Route> classes={classes!("title", "is-block")} to={Route::Post { id: post.seed }}>
                            { &post.title }
                        </Link<Route>>
                        <Link<Route> classes={classes!("subtitle", "is-block")} to={Route::Author { id: post.author.seed }}>
                            { &post.author.name }
                        </Link<Route>>
                    </div>
                </div>
                <div class="tile is-6 is-parent">
                    <div class="tile is-child">
                        <div class="card">
                            <div class="card-image">
                                <figure class="image is-2by1">
                                    <img src=""/>
                                </figure>
                            </div>
                            <div class="card-content">
                                <div class="content">
                                    <h1 class="title">{ &post_detail.title }</h1>
                                    { &post_detail.content }
                                    <br/>
                                    {"#css #responsive"}
                                    <br/>
                                    <time datetime="2016-1-1">{"11:09 PM - 1 Jan 2016"}</time>
                                    <br/>
                                    <a>{"查看/Detail"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
