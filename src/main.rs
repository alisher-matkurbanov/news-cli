use std::{error::Error, env};
use serde::Deserialize;
use colour::{dark_green, yellow};

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: String) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(&url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for article in &articles.articles {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("NEWS_API_KEY")?;
    let url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", api_key);
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
