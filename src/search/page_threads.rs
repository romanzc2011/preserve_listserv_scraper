use std::collections::HashMap;
use scraper::{Html, Selector};

// Data structure to hold the post/reply data
#[derive(Debug)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub body: String,
}

// Data structure to hold the thread data
pub struct Thread {
    pub post: Post,
    pub replies: Vec<Post>,
}

// Parse the HTML content and extract the thread data
pub fn parse_threads(doc: &Html) -> Result<HashMap<String, Thread>, String> {
    let row_sel = Selector::parse("tr[id]").map_err(|e| format!("Failed to parse selector: {}", e))?;

    let mut threads = HashMap::new();
    for row in doc.select(&row_sel) {
        let post_id: String = row.value().attr("id").unwrap_or_default().to_string();
        let post_title = row.select(&Selector::parse("td.title").unwrap()).next()
            .map(|e| e.text().collect::<String>()).unwrap_or_default();
        let post_author: String = row.select(&Selector::parse("td.author").unwrap()).next()
            .map(|e| e.text().collect::<String>()).unwrap_or_default();
        let post_date: String = row.select(&Selector::parse("td.date").unwrap()).next()
            .map(|e| e.text().collect::<String>()).unwrap_or_default();
        let post_body: String = row.select(&Selector::parse("td.body").unwrap()).next()
            .map(|e| e.text().collect::<String>()).unwrap_or_default();

        // Create the main post
        let main_post = Post {
            id: post_id.clone(),
            title: post_title,
            author: post_author,
            date: post_date,
            body: post_body,
        };

        // Create the thread
        let thread = Thread {
            post: main_post,
            replies: Vec::new(),
        };

        threads.insert(post_id, thread);
    }

    Ok(threads)
}
