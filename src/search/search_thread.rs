use reqwest::blocking::Client;
use scraper::{Html, Selector};
use url::Url;
use std::collections::HashMap;
use crate::search::page_threads::{parse_threads, Thread};

#[derive(Debug)]
pub struct SearchThread {
    client: Client,
    url: String,
    page: u32,
    pub doc: Html,
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

    // #########################################################################
    // SEARCH AND TRAVERSE THE TOC
    // #########################################################################
    pub fn search_toc(&mut self) -> Result<(), String> {
        self.build_search_url()?;

        // First collect all links
        let mut links = Vec::new();
        let link_sel = Selector::parse("a[href^='#']").unwrap();
        for a in self.doc.select(&link_sel) {
            let href_attr = a.value().attr("href").unwrap();
            let text = a.text().collect::<String>();

            if let Some(href_id) = href_attr.strip_prefix("#") {
                println!("Found link: {} -> {}", href_id, text);
                
                if href_id.eq_ignore_ascii_case("skipnavigation") {
                    continue;
                }
                
                links.push(href_id.to_string());
            }
        }
        
        // Then process each link
        for link_id in links {
            self.follow_link(&link_id)?;
        }
        
        return Ok(())
    }

    // #########################################################################
    // SEARCH AND FOLLOW
    // #########################################################################
    pub fn search_and_follow(&mut self) -> Result<Vec<HashMap<String, Thread>>, String> {
        // Fetch and parse TOC
        self.build_search_url()?;

        // Fetch all More relative hrefs 
        let rel_links = self.collect_judsys_links()?;

        let mut all_threads = Vec::new();

        // Loop through more link and make absolute links
        for rel in rel_links {
            let base = Url::parse(&self.url).map_err(|e| format!("Failed to parse URL: {}", e))?;
            let full = base.join(&rel).map_err(|e| format!("Failed to join URL: {}", e).to_string())?;

            // Follow link and fetch content
            let resp = self.client.get(full.as_str()).send().map_err(|e| format!("Failed to send request: {}", e))?;
            let body = resp.text().map_err(|e| format!("Failed to get text: {}", e))?;

            // Parse and extract replies and hand over to page threads
            let doc2 = Html::parse_document(&body);
            let threads_map = parse_threads(&doc2)?;
            all_threads.push(threads_map);
        }
        
        return Ok(all_threads)
    }



    // #########################################################################
    // RESOLVE ID IN THE HREF - specific to the search page JUDSYS, searching for the text between <a> tags
    // #########################################################################
    pub fn collect_judsys_links(&self) -> Result<Vec<String>, String> {
        // Build a selector matching any <a> whose href starts with the JUDSYS prefix
        let prefix = "/cgi-bin/wa?A2=JUDSYS;";
        let selector_str = format!("a[href^=\"{}\"]", prefix);
        let sel = Selector::parse(&selector_str)
            .map_err(|e| format!("Failed to parse selector: `{}`: {}", selector_str,  e))?;

        // Collect all links that match the selector
        let links: Vec<String> = self.doc
            .select(&sel)
            .filter(|el| {
                el.text().map(str::trim).any(|t| t.starts_with("[More ..."))
            })
            .filter_map(|el| el.value().attr("href").map(str::to_string))
            .collect();

        return Ok(links)
    }

    // Build the search URL
    fn build_search_url(&mut self) -> Result<String, String> {
        let protected = self.client.get(&self.url).send().map_err(|e| format!("Failed to get page: {}", e))?;
        println!("Protected GET returned: {}", protected.status());
        let protected_html = protected.text().map_err(|e| format!("Failed to get text: {}", e))?;
        self.doc = Html::parse_document(&protected_html);

        Ok(self.url.clone())
    }

    // Follow a link by its ID and extract content
    fn follow_link(&mut self, link_id: &str) -> Result<(), String> {
        println!("Following link: {}", link_id);

        // Find the element with this ID
        let id_selector = Selector::parse(&format!("[id='{}']", link_id)).map_err(|e| format!("Failed to parse selector: {}", e))?;
        
        if let Some(element) = self.doc.select(&id_selector).next() {
            // Extract the content
            let content = element.text().collect::<String>();
            println!("Content: {}", content);
            
            // Store the result
            self.results.push(content);
        } else {
            println!("Element with ID '{}' not found", link_id);
        }
        
        Ok(())
    }
}