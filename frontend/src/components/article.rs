use yew::prelude::*;
use types::article::ArticleBody;

#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    pub article: ArticleBody
}

#[function_component(Article)]
pub fn article(_props: &ArticleProps) -> Html {
    html! {
        <>
        <div class="banner">
            <div class="container">

                <h1>{&_props.article.title}</h1>

                <div class="article-meta">
                    <a href=""><img alt="profile pic" src={_props.article.author.profile.image.as_ref().unwrap().to_owned()}/></a>
                    <div class="info">
                        <a href="" class="author">{&_props.article.author.profile.username}</a>
                        <span class="date">{&_props.article.created_at.to_string()}</span>
                    </div>
                    <button class="btn btn-sm btn-outline-secondary">
                        <i class="ion-plus-round"></i>
                        {&'\u{00a0}'}
                        {format!("Follow {}", &_props.article.author.profile.username)} <span class="counter">{"2"}</span>
                    </button>
                    {&'\u{00a0}'}
                    {&'\u{00a0}'}
                    <button class="btn btn-sm btn-outline-primary">
                        <i class="ion-heart"></i>
                        {&'\u{00a0}'}
                        {"Favorite Post "}<span class="counter">{&_props.article.favorites_count}</span>
                    </button>
                </div>

            </div>
        </div>

        <div class="container page">

            <div class="row article-content">
                <div class="col-md-12">
                    <p>
                        {&_props.article.description}
                    </p>

                    <p>{&_props.article.body}</p>
                </div>
            </div>

            <hr/>

            <div class="article-actions">
            <div class="article-meta">
                <a href=""><img alt="profile pic" src={_props.article.author.profile.image.as_ref().unwrap().to_owned()}/></a>
                <div class="info">
                    <a href="" class="author">{&_props.article.author.profile.username}</a>
                    <span class="date">{&_props.article.created_at.to_string()}</span>
                </div>
                <button class="btn btn-sm btn-outline-secondary">
                    <i class="ion-plus-round"></i>
                    {&'\u{00a0}'}
                    {format!("Follow {}", &_props.article.author.profile.username)} <span class="counter">{"2"}</span>
                </button>
                {&'\u{00a0}'}
                {&'\u{00a0}'}
                <button class="btn btn-sm btn-outline-primary">
                    <i class="ion-heart"></i>
                    {&'\u{00a0}'}
                    {"Favorite Post "}<span class="counter">{&_props.article.favorites_count}</span>
                </button>
            </div>
            </div>
        </div>
        </>
    }
}
