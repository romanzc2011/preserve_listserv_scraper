use reqwest::blocking::Client;
use scraper::{Html, Selector};
use dotenv::dotenv;
use std::env;

pub struct User {
    email: String,
    password: String,
    login_url: String,
    protected_url: String,
    client: Option<Client>,
}

impl User {
    // Constructor for User struct
    pub fn new(email: &str, password: &str, login_url: &str, protected_url: &str) -> Self {
        User {
            email: email.to_string(),
            password: password.to_string(),
            login_url: login_url.to_string(),
            protected_url: protected_url.to_string(),
            client: None,
        }
    }

    pub fn login(&mut self) -> Result<(), String> {
        // Create a new HTTP client with cookie store
        let client = Client::builder()
            .cookie_store(true)
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| format!("Failed to build client: {}", e))?;

        // Get the login page
        let login_page = client.get(&self.protected_url).send().map_err(|e| format!("Failed to get login page: {}", e))?;
        let login_html = login_page.text().map_err(|e| format!("Failed to read login page: {}", e))?;
        let login_doc = Html::parse_document(&login_html);

        // Extract all input fields
        let input_sel = Selector::parse("form#login input").map_err(|e| format!("Failed to parse selector: {}", e))?;
        let mut params: Vec<(String, String)> = Vec::new();
        for inp in login_doc.select(&input_sel) {
            if let Some(name) = inp.value().attr("name") {
                let val = inp.value().attr("value").unwrap_or("").to_string();
                params.push((name.to_string(), val));
            }
        }        

        dotenv().ok();
        let passwd = env::var("LISTSERV_PASSWORD").map_err(|_| "LISTSERV_PASSWORD must be set".to_string())?;

        // Overwrite the params with the login credentials
        for(name, val) in &mut params {
            if name == "p" {
                *val = passwd.to_string();
            }
        }

        // POST the form to login
        let resp = client.post(&self.login_url)
            .form(&params)
            .send()
            .map_err(|e| format!("Failed to post login form: {}", e))?;

        // Check if login was successful
        if resp.status().is_success() {
            println!("\x1b[32mLogin successful\x1b[0m");
            println!("Login POST returned: {}", resp.status());
        } else {
            return Err("Login failed".to_string());
        }

        // Store the client for later use
        self.client = Some(client);
        Ok(())
    }

    // Get the client for use in other modules
    pub fn get_client(&self) -> Result<&Client, String> {
        self.client.as_ref().ok_or_else(|| "Client not initialized. Call login() first.".to_string())
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        if self.password.is_empty() {
            return Err("Password cannot be empty".to_string());
        }
        Ok(())
    }
}