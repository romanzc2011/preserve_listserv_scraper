use reqwest::blocking::Client;
use scraper::{Html, Selector};

pub struct SearchThread {
    client: Client,
    url: String,
    page: u32,
    doc: Html,
    results: Vec<String>,
}

impl SearchThread {
    pub fn new(client: Client, url: &str) -> Self {
        SearchThread {
            client,
            url: url.to_string(),
            page: 0,
            doc: Html::parse_document(""),
            results: Vec::new(),
        }
    }

    pub fn search_toc(&mut self) -> Result<(), String> {
        self.build_search_url()?;

        let link_sel = Selector::parse("a[href^='#']").unwrap();
        for a in self.doc.select(&link_sel) {
            let href = a.value().attr("href").unwrap();
            let text = a.text().collect::<String>();
            println!("{}: {}", href, text);
        }
        
        Ok(())
    }

    // Build the search URL
    fn build_search_url(&mut self) -> Result<String, String> {
        let protected = self.client.get(&self.url).send().map_err(|e| format!("Failed to get page: {}", e))?;
        println!("Protected GET returned: {}", protected.status());
        let protected_html = protected.text().map_err(|e| format!("Failed to get text: {}", e))?;
        self.doc = Html::parse_document(&protected_html);

        Ok(self.url.clone())
    }
        
}