use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{components::Layout, services::{user::get_profile, ApiError}};
use types::user::Profile as UserProfile;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub username: String,
}

#[function_component(Profile)]
pub fn profile(_props: &Props) -> Html {
    let user = use_state(|| None);
    let username = _props.username.to_owned();
    {
        let user = user.clone();
        use_effect_with_deps(
            move |_| {
                if user.is_none() {
                    spawn_local(async move {
                        let resp = get_profile(&username).await;
                        user.set(Some(resp))
                    });
                }
                || {}
            },
            (),
        );
    }
    
    match user.as_ref() {
        None => html! { <h1>{"Loading"}</h1> },
        Some(Ok(UserProfile { profile })) => {
            html! {
                <Layout>
                    <>
                        <div class="profile-page">
                        <div class="user-info">
                            <div class="container">
                                <div class="row">
                                    <div class="col-xs-12 col-md-10 offset-md-1">
                                        <img alt="profile pic" src={profile.image.clone()} class="user-img"/>
                                        <h4>{&profile.username}</h4>
                                        <p>
                                        {&profile.bio}
                                        </p>
                                        <button class="btn btn-sm btn-outline-secondary action-btn">
                                            <i class="ion-plus-round"></i>
                                            {&'\u{00a0}'}
                                            {format!("Follow {}", &profile.username)}
                                        </button>
                                    </div>
        
                                </div>
                            </div>
                        </div>
        
                        <div class="container">
                            <div class="row">
                                <div class="col-xs-12 col-md-10 offset-md-1">
                                    <div class="articles-toggle">
                                        <ul class="nav nav-pills outline-active">
                                            <li class="nav-item">
                                                <a class="nav-link active" href="">{"My Articles"}</a>
                                            </li>
                                            <li class="nav-item">
                                                <a class="nav-link" href="">{"Favorited Articles"}</a>
                                            </li>
                                        </ul>
                                    </div>
        
                                    <div class="article-preview">
                                        <div class="article-meta">
                                            <a href=""><img alt="profile pic" src="http://i.imgur.com/Qr71crq.jpg"/></a>
                                            <div class="info">
                                                <a href="" class="author">{"Eric Simons"}</a>
                                                <span class="date">{"January 20th"}</span>
                                            </div>
                                            <button class="btn btn-outline-primary btn-sm pull-xs-right">
                                                <i class="ion-heart"></i> {"29"}
                                            </button>
                                        </div>
                                        <a href="" class="preview-link">
                                            <h1>{"How to build webapps that scale"}</h1>
                                            <p>{"This is the description for the post."}</p>
                                            <span>{"Read more..."}</span>
                                        </a>
                                    </div>
        
                                    <div class="article-preview">
                                        <div class="article-meta">
                                            <a href=""><img alt="profile pic" src="http://i.imgur.com/N4VcUeJ.jpg"/></a>
                                            <div class="info">
                                                <a href="" class="author">{"Albert Pai"}</a>
                                                <span class="date">{"January 20th"}</span>
                                            </div>
                                            <button class="btn btn-outline-primary btn-sm pull-xs-right">
                                                <i class="ion-heart"></i> {"32"}
                                            </button>
                                        </div>
                                        <a href="" class="preview-link">
                                            <h1>{"The song you won't ever stop singing. No matter how hard you try."}</h1>
                                            <p>{"This is the description for the post."}</p>
                                            <span>{"Read more..."}</span>
                                            <ul class="tag-list">
                                                <li class="tag-default tag-pill tag-outline">{"Music"}</li>
                                                <li class="tag-default tag-pill tag-outline">{"Song"}</li>
                                            </ul>
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                        </div>
                    </>
                </Layout>
            }
        },
        Some(Err(ApiError::NotFound)) => html! { <h1>{"Not Found"}</h1> },
        Some(_) => html! { <h1>{"Internal Server Error"}</h1> },
    }

    
}
