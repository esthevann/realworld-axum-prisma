use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    User(User),
    Article(Article)
}


#[derive(Debug, Args, Clone)]
pub struct User {
    #[command(subcommand)]
    pub command: UserCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum UserCommands {
    CreateUser,
    GetProfile,
    LoginUser,
    FollowUser,
    UnfollowUser,
}

#[derive(Debug, Args, Clone)]
pub struct Article {
    #[command(subcommand)]
    pub command: ArticleCommands
}

#[derive(Subcommand, Clone, Debug)]
pub enum ArticleCommands {
    CreateArticle,
    GetArticle
}