use article::create_article;
use clap::Parser;

use crate::{args::Cli, user::create_user};

mod args;
mod article;
mod user;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Cli::parse();

    match args.command {
        args::Commands::User(c) => match c.command {
            args::UserCommands::CreateUser => {
                create_user().await?;
            }
            args::UserCommands::GetProfile => todo!(),
            args::UserCommands::LoginUser => todo!(),
            args::UserCommands::FollowUser => todo!(),
            args::UserCommands::UnfollowUser => todo!(),
        },
        args::Commands::Article(c) => match c.command {
            args::ArticleCommands::CreateArticle => {
                create_article().await?;
            }
            args::ArticleCommands::GetArticle => todo!(),
        },
    };

    Ok(())
}
