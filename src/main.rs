mod theme;

use std::error::Error;
use dotenv::dotenv;
use newsapi::{Articles, get_articles};

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top headline\n\n");
    for a in  &articles.articles {
        theme.print_text(&format!("`{}`",a.title));
        theme.print_text(&format!("> *{}*",a.url));
        theme.print_text("---");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let key = std::env::var("API_KEY")?;
    let url: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";

    let url = format!("{}{}",url, key);

    let articles  = get_articles(&url)?;
    render_articles(&articles);

    Ok(())

}
