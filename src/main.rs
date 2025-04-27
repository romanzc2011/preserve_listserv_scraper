use dotenv::dotenv;
use std::env;
use std::error::Error;
use rust_loguru::{Logger, LogLevel, Record};
use rust_loguru::handler::console::ConsoleHandler;
use rust_loguru::{info, debug, error};
use std::sync::Arc;
use parking_lot::RwLock;

mod auth;
mod search;
        
use auth::user::User;
use search::search_thread::SearchThread;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger();
    dotenv().ok();
    let email = env::var("LISTSERV_EMAIL").expect("LISTSERV_EMAIL must be set");
    let password = env::var("LISTSERV_PASSWORD").expect("LISTSERV_PASSWORD must be set");

    // URLs
    let protected_url = "https://arcus.nyed.circ2.dcn/cgi-bin/wa?A0=JUDSYS&X=O4F720C6CA80E094778&Y=roman_campbell%40lawb.uscourts.gov";
    let login_url  = "https://arcus.nyed.circ2.dcn/cgi-bin/wa";

    let mut user = User::new(&email, &password, &login_url, &protected_url);
    
    user.validate()?;
    user.login()?;

    info!("✅ Done!");
    
    // Create a search thread with the client
    let mut search = SearchThread::new(user.get_client()?.clone(), &protected_url);
    //search.search()?;
    search.search_toc()?;

    Ok(())
}

fn setup_logger() {
   // Create a logger with a handler
    let handler = Arc::new(RwLock::new(ConsoleHandler::new()));
    let mut logger = Logger::new(LogLevel::Debug);
    logger.add_handler(handler); 

    rust_loguru::init(logger);
}

