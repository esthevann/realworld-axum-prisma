use std::sync::Arc;

use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::{components::{Layout, Article as ArticleComponent}, services::{article::get_article, ApiError}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub slug: String
}

#[function_component(Article)]
pub fn article(_props: &Props) -> Html {
    let slug = Arc::new(_props.slug.to_string());
    let article = use_async_with_options(async move { get_article(&slug).await }, UseAsyncOptions::enable_auto());

    if article.loading {
        html!{ "Loading" }
    } else if let Some(error) =  &article.error {
        match error {
            ApiError::ServerError => html!{"Internal Server Error"},
            ApiError::NotFound => html!{"Not Found"},
        }
    } else {
        html! {
            <Layout> 
                <>
                <div class="article-page">
                    {
                    if let Some(article) = &article.data {
                        html!{
                            <ArticleComponent article={article.article.clone()} />
                        }
                    } else {
                        html! {}
                    }
                    }
        
                    <div class="row">
        
                        <div class="col-xs-12 col-md-8 offset-md-2">
        
                            <form class="card comment-form">
                                <div class="card-block">
                                    <textarea class="form-control" placeholder="Write a comment..." rows="3"></textarea>
                                </div>
                                <div class="card-footer">
                                    <img alt="profile pic" src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img"/>
                                    <button class="btn btn-sm btn-primary">
                                        {"Post Comment"}
                                    </button>
                                </div>
                            </form>
        
                            <div class="card">
                                <div class="card-block">
                                    <p class="card-text">{"With supporting text below as a natural lead-in to additional content."}</p>
                                </div>
                                <div class="card-footer">
                                    <a href="" class="comment-author">
                                        <img alt="profile pic" src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img"/>
                                    </a>
                                    {&'\u{00a0}'}
                                    <a href="" class="comment-author">{"Jacob Schmidt"}</a>
                                    <span class="date-posted">{"Dec 29th"}</span>
                                </div>
                            </div>
        
                            <div class="card">
                                <div class="card-block">
                                    <p class="card-text">{"With supporting text below as a natural lead-in to additional content."}</p>
                                </div>
                                <div class="card-footer">
                                    <a href="" class="comment-author">
                                        <img alt="profile pic" src="http://i.imgur.com/Qr71crq.jpg" class="comment-author-img"/>
                                    </a>
                                    {&'\u{00a0}'}
                                    <a href="" class="comment-author">{"Jacob Schmidt"}</a>
                                    <span class="date-posted">{"Dec 29th"}</span>
                                    <span class="mod-options">
                        <i class="ion-edit"></i>
                        <i class="ion-trash-a"></i>
                        </span>
                                </div>
                            </div>
        
                        </div>
        
                    </div>
        
                </div>
                </>
            </Layout>
            }
    }
    
}