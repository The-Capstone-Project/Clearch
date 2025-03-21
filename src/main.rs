mod llms;
mod os_scraper;
mod traits; // Import the new module

use crate::llms::gemini::GeminiModel;
use crate::traits::LLMRequest;
use clap::CommandFactory;
use clap::Parser;
use dotenv::dotenv;
use std::io::BufRead;
use sys_info::{os_release, os_type};

#[derive(Parser, Debug)]
#[command(name = "Clearch")]
#[command(author = "Advaith Narayanan <advaith@glitchy.systems>")]
#[command(about = "Search using the command line")]
struct Clearch {
    #[arg(
        short = 'q',
        long = "specify",
        help = "Specify search query as a string"
    )]
    search_query: Option<String>,

    #[arg(long = "std", help = "Read query from stdin (ignores any args)")]
    use_stdin: bool,

    #[arg(trailing_var_arg = true, help = "Direct input without flags")]
    args: Vec<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let os_info = format!(
        "OS: {}  OS REL: {} ",
        os_type().unwrap_or_else(|_| "Unknown".to_string()),
        os_release().unwrap_or_else(|_| "Unknown".to_string()),
    );

    println!("{}", os_info);

    let search = Clearch::parse();

    let apikey = match std::env::var("AI_API") {
        Ok(apikey) => apikey,
        Err(e) => panic!("API not found: {}", e),
    };

    let gemini_model = GeminiModel::new(apikey);

    // Run os-scraper and get its output
    let os_scraper_output = match os_scraper::run_os_scraper() {
        // Call the function from the module
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error running os-scraper: {}", e);
            String::new() // Use an empty string if os-scraper fails
        }
    };

    // Function to get the prompt with OS info and os-scraper output
    fn get_prompt(os_info: &str, os_scraper_output: &str) -> String {
        let prompt_content = match std::fs::read_to_string("prompt") {
            Ok(content) => content,
            Err(_) => {
                // Try alternative path - based on executable location
                let exe_path = std::env::current_exe().unwrap_or_default();
                let exe_dir = exe_path
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new(""));
                let src_dir = exe_dir.join("src");

                std::fs::read_to_string(src_dir.join("prompt")).unwrap_or_else(|_| {
                    // For development environment
                    std::fs::read_to_string("src/prompt").unwrap_or_default()
                })
            }
        };
        format!("{}\n{}\n{}", prompt_content, os_info, os_scraper_output)
    }

    // If --std flag is provided, read from stdin (ignoring other args)
    if search.use_stdin {
        let mut buffer = String::new();
        for line in std::io::stdin().lock().lines() {
            buffer.push_str(&line.expect("Failed to read line from stdin"));
            buffer.push('\n');
        }

        if !buffer.trim().is_empty() {
            println!("Searching with input from stdin");
            let prompt = get_prompt(&os_info, &os_scraper_output);
            gemini_model.req(&buffer, &prompt).await.unwrap();
        } else {
            println!("Empty input from stdin");
            std::process::exit(1);
        }
    }
    // If search_query is provided with -q/--specify flag, use that directly
    else if let Some(query) = search.search_query.as_deref() {
        let prompt = get_prompt(&os_info, &os_scraper_output);
        println!("Searching for: {}", query);
        gemini_model.req(query, &prompt).await.unwrap();
    }
    // If direct args were provided without flags, use them as query
    else if !search.args.is_empty() {
        let query = search.args.join(" ");
        let prompt = get_prompt(&os_info, &os_scraper_output);
        println!("Searching for: {}", query);
        gemini_model.req(&query, &prompt).await.unwrap();
    }
    // If nothing provided, show help
    else {
        Clearch::command().print_help().unwrap();
        std::process::exit(1);
    }
}
