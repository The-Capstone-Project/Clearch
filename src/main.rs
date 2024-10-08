mod llms;
mod traits;
use crate::llms::gemini::GeminiModel;
use crate::traits::LLMRequest;
use clap::Parser;
use clap_stdin::FileOrStdin;
use dotenv::dotenv;
use sys_info::{os_release, os_type};


#[derive(Parser)]
#[command(name = "Clearch")]
#[command(author = "Advaith Narayanan <advaith@glitchy.systems>")]
#[command(about = "Search using the command line")]
struct Gemini {
    
    #[arg(short='q',long="specify")]
    search_query: Option<String>,
    
    // #[arg(long="query",help = "Pass `-h` and you'll see me!")]
    query: FileOrStdin<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!(
        "OS: {}  OS REL: {} ",
        os_type().unwrap(),
        os_release().unwrap(),
    );

    let search = Gemini::parse();

    let apikey = match std::env::var("GEMIAI_API") {
        Ok(apikey) => apikey,
        Err(e) => panic!("API not found: {}", e),
    };
    
    let gemini_model = GeminiModel::new(apikey);

    if let Some(query) = search.search_query.as_deref() {
        println!("Searching for: {}", query);
        gemini_model.req(query, "").await.unwrap();
    } else {
        if let Ok(buffer) = search.query.contents() {
            println!("{buffer}");
            let prompt: Vec<String> = include_str!("prompt").split("\n").map(|s| s.to_string()).collect();
            println!("lenth of array {}",prompt.len());
            for i in prompt{
                println!("{}", i);
            }
            gemini_model.req(
                &buffer,
                format!(
                    "OS: {}  kernal version: {} use the information and the query provided in the promt to answer as correctly as possible if unsure do not answer but reply the model is unsure about the answer",
                    os_type().unwrap(),
                    os_release().unwrap(),
                
                )
                .as_str(),
            )
            .await
            .unwrap();
        }
    }
}


