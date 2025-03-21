mod llms;
mod traits;
use crate::llms::gemini::GeminiModel;
use crate::traits::LLMRequest;
use clap::CommandFactory; // Add this import for Gemini::command()
use clap::Parser;
use dotenv::dotenv;
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

    println!(
        "OS: {}  OS REL: {} ",
        os_type().unwrap(),
        os_release().unwrap(),
    );

    let search = Clearch::parse();

    let apikey = match std::env::var("AI_API") {
        Ok(apikey) => apikey,
        Err(e) => panic!("API not found: {}", e),
    };

    let gemini_model = GeminiModel::new(apikey);

    // If --std flag is provided, read from stdin (ignoring other args)
    if search.use_stdin {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read from stdin");

        if !buffer.trim().is_empty() {
            println!("Searching with input from stdin");
            let prompt = match std::fs::read_to_string("prompt") {
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
            gemini_model.req(&buffer, prompt.as_str()).await.unwrap();
        } else {
            println!("Empty input from stdin");
            std::process::exit(1);
        }
    }
    // If search_query is provided with -q/--specify flag, use that directly
    else if let Some(query) = search.search_query.as_deref() {
        let prompt = match std::fs::read_to_string("prompt") {
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
        println!("Searching for: {}", query);
        gemini_model.req(query, prompt.as_str()).await.unwrap();
    }
    // If direct args were provided without flags, use them as query
    else if !search.args.is_empty() {
        let query = search.args.join(" ");
        let prompt = match std::fs::read_to_string("prompt") {
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
        println!("Searching for: {}", query);
        gemini_model.req(&query, prompt.as_str()).await.unwrap();
    }
    // If nothing provided, show help
    else {
        Clearch::command().print_help().unwrap();
        std::process::exit(1);
    }
}
