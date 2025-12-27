use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use webbrowser;

pub fn search(query: &[String], open_first: bool) -> Result<()> {
    let query_str = query.join(" ");
    let url = format!("https://html.duckduckgo.com/html?q={}", query_str);

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.magenta} {msg}")?);
    spinner.set_message(format!("Searching for '{}'...", query_str));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; NeoCLI/1.0)")
        .build()?;

    let resp = client.get(&url).send()?;
    let body = resp.text()?;

    spinner.finish_and_clear();

    if open_first {
        println!("{}", "Opening top result in browser...".blue());
        // We need to parse at least the first link to open it, or just open the search page
        // Opening the search page is safer if we just want "search results"
        // But the feature said "Open results", let's try to extract the first one.
        
        let document = Html::parse_document(&body);
        let link_selector = Selector::parse(".result__a").unwrap();
        
        if let Some(element) = document.select(&link_selector).next() {
            if let Some(href) = element.value().attr("href") {
                webbrowser::open(href)?;
                return Ok(());
            }
        }
        
        // Fallback
        webbrowser::open(&url)?;
    } else {
        println!("{}", format!("Search Results for: {}", query_str).bold().underline());
        println!();

        let document = Html::parse_document(&body);
        let result_selector = Selector::parse(".result").unwrap();
        let title_selector = Selector::parse(".result__a").unwrap();
        let snippet_selector = Selector::parse(".result__snippet").unwrap();

        let mut count = 0;
        for element in document.select(&result_selector) {
            if count >= 5 { break; } // Limit to 5 results

            let title_el = element.select(&title_selector).next();
            let snippet_el = element.select(&snippet_selector).next();

            if let (Some(title), Some(snippet)) = (title_el, snippet_el) {
                let title_text = title.text().collect::<Vec<_>>().join("");
                let link = title.value().attr("href").unwrap_or_default();
                let snippet_text = snippet.text().collect::<Vec<_>>().join("");

                println!("{}", title_text.cyan().bold());
                println!("{}", link.dimmed());
                println!("{}", snippet_text);
                println!();
                count += 1;
            }
        }

        if count == 0 {
            println!("{}", "No results found or could not parse results.".yellow());
            println!("Try opening in browser: {}", url);
        } else {
             println!("{}", "Run with --open to open the first result automatically.".dimmed().italic());
        }
    }

    Ok(())
}
