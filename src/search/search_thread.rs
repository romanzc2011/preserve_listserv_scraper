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

    fn resolve_href(&self, href_id: &str) -> Result<String, String> {
        // For now, just return the href_id as is
        // In a real implementation, you would look up the element with this ID
        Ok(href_id.to_string())
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