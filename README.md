# Clearch 🚀 

Clearch is a command-line search engine that leverages AI models such as Gemini, Claude, and GPT for its operations. It is developed in Rust, ensuring both performance and safety. 🦀 

## Table of Contents 📚
- -> [Project Structure 📂](#project-structure-)
- -> [Description 📝](#description-)
- -> [Command-Line Interface 💻](#command-line-interface-)
- -> [Relevant Files and Symbols 📂](#relevant-files-and-symbols-)

## Project Structure 📂 

```
. 
├── .gitignore 
├── .vscode/ 
│   └── settings.json 
├── build.sh 
├── Cargo.lock 
├── Cargo.toml 
├── packager.py 
├── README.md 
├── src/ 
│   ├── llms/ 
│   │   ├── claude.rs 
│   │   ├── gemini.rs 
│   │   ├── gpt.rs 
│   │   └── mod.rs 
│   ├── main.rs 
│   ├── prompt 
│   └── traits.rs 
└── test.py 
``` 

## Description 📝 

Clearch is a command-line search engine utilizing various AI models. The main logic of the project resides in the `src` directory, while specific implementations of the AI models are found in the `src/llms` directory. Each file in `src/llms` corresponds to a specific AI model: 

- **gemini.rs**: Implements the Gemini model. 
- **claude.rs**: Implements the Claude model. 
- **gpt.rs**: Implements the GPT model. 

These models are structured as Rust modules and are imported in `src/llms/mod.rs`. Each AI model implements the `LLMRequest` trait, defined in `src/traits.rs`, providing a common interface ensuring interchangeability among models in the main program. 🔄 

### Command-Line Interface 💻 

The `clap` library is used in `src/main.rs` for handling command-line inputs. This popular Rust library simplifies command-line argument parsing. The `#[derive(Parser)]` attribute automatically generates the necessary code to parse command-line arguments into a `Gemini` struct. This struct represents the command-line interface, wherein each field corresponds to a specific command-line argument or option. 

#### Relevant Files and Symbols 📂 

- `src/llms/mod.rs` 
- `src/llms/gemini.rs` 
- `src/llms/claude.rs` 
- `src/traits.rs` 
- `LLMRequest` 
- `src/main.rs` 
- `Gemini` 

