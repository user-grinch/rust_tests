/*
    Mixmods Scrapper
    Scraps mixmods for articles
*/

use std::{io::Write};
use std::fs;
use std::fs::OpenOptions;
use scraper::{Html, Selector};

const URL : &str = "https://www.mixmods.com.br/page/";
const NUMBER_OF_PAGES : u32 = 2;
const OUT_FILE : &str = "out.txt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _ = fs::remove_file(OUT_FILE);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(OUT_FILE)
        .expect("Failed to create output file");

    for page in 1..NUMBER_OF_PAGES {
        let responce = reqwest::get(format!("{}{}", URL , page)).await?;

        if responce.status().is_success() {
            let body = responce.text().await?;
            let document = Html::parse_document(&body);
            
            let article_selector = Selector::parse("article div.inside-article")
                .expect("Failed to parse document");

            for article in document.select(&article_selector) {
                
                // header
                let header_selector = Selector::parse("header h2 a")
                    .expect("Failed to parse document");
                let header = article.select(&header_selector)
                    .nth(0)
                    .unwrap();
                let link = header.value().attr("href")
                    .expect("Failed to get link");
                let title = header.text()
                    .nth(0)
                    .unwrap();
                
                // date
                let date_selector = Selector::parse("header div span time")
                    .expect("Failed to parse document");
                let footer = article.select(&date_selector)
                    .nth(0)
                    .unwrap();
                let date = footer.text()
                    .nth(0)
                    .unwrap();

                // footer
                let footer_selector = Selector::parse("footer span a")
                    .expect("Failed to parse document");
                let footer = article.select(&footer_selector)
                    .nth(0)
                    .unwrap();
                let mut comments = footer.text()
                    .nth(0)
                    .unwrap()
                    .split(" ")
                    .nth(0)
                    .unwrap();

                if comments == "Deixe" {
                    comments = "0";
                }

                writeln!(file, "Article: {}\nLink: {}\nUpdated: {}\nComments: {}\n", title, link, date, comments)?;
            }         
        }else {
            println!("Failed to get responce");
        }
    }

    Ok(())
}