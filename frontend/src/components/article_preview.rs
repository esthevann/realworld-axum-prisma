use types::article::{ArticleBody};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ArticlePreviewProps {
    pub author: String,
    pub image: String,
    pub date: String,
    pub favorites: i32,
    pub description: String,
    pub title: String,
    pub tag_list: Option<Vec<String>>
}



#[function_component(ArticlePreview)]
pub fn article_preview(props: &ArticlePreviewProps) -> Html {
    html! {
        <div class="article-preview">
            <div class="article-meta">
                <a href=""><img alt="profile pic" src={props.image.to_owned()}/></a>
                <div class="info">
                    <a href="" class="author">{&props.author}</a>
                    <span class="date">{&props.date}</span>
                </div>
                <button class="btn btn-outline-primary btn-sm pull-xs-right">
                    <i class="ion-heart"></i> {&props.favorites}
                </button>
            </div>
            <a href="" class="preview-link">
                <h1>{&props.title}</h1>
                <p>{&props.description}</p>
                <span>{"Read more..."}</span>
            </a>
        </div>

    }
}

#[derive(Properties, PartialEq)]
pub struct ArticlePreviewListProps {
    pub articles: Vec<ArticleBody>
}



#[function_component(ArticlePreviewList)]
pub fn article_preview_list(props: &ArticlePreviewListProps) -> Html {
    props.articles.iter().map(|article| {
        let article = article.clone();
        html! {
        <ArticlePreview author={article.author.profile.username} image={article.author.profile.image.unwrap_or_else(|| String::from(""))}
             date={article.created_at.to_string()} favorites={article.favorites_count} 
            description={article.description} title={article.title} tag_list={Some(article.tag_list)} />
    }}).collect()
}
