use std::sync::Arc;
use yew_hooks::prelude::*;
use yew::prelude::*;

use crate::{
    components::{Layout, ArticlePreviewList},
    services::{user::get_profile, article::get_articles},
};
use types::{user::Profile as UserProfile, article::MultipleArticles};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub username: String,
}

#[function_component(Profile)]
pub fn profile(_props: &Props) -> Html {
    let username = Arc::new(_props.username.to_string());
    let username_clone = Arc::clone(&username);
    let user = use_async_with_options(async move { get_profile(&username_clone).await }, UseAsyncOptions::enable_auto());
    let articles = use_async_with_options(async move { get_articles(&username).await }, UseAsyncOptions::enable_auto());

    html! {
        <Layout>
            <>
                <div class="profile-page">
                <div class="user-info">
                    <div class="container">
                        <div class="row">
                        {
                            if user.loading {
                                html! { "Loading" }
                            } else {
                                html! {}
                            }                         
                        }
                        {
                            if let Some(UserProfile { profile }) = &user.data  {
                                html! {
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
                                } 
                            } else {
                                html! {}
                            }
                        }
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

                        {
                            if articles.loading {
                                html! { "Loading" }
                            } else {
                                html! {}
                            }       
                        }
                        {
                            if let Some(MultipleArticles { articles, articles_count: _ }) = &articles.data {
                                html! {
                                    <ArticlePreviewList articles={articles.clone()} />
                                }
                            } else {
                                html! {}
                            }
                        }

                        </div>
                    </div>
                </div>
                </div>
            </>
        </Layout>
    }
}
