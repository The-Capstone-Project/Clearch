use std::process::{Command, Stdio};

pub fn run_os_scraper() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("./os-scraper")
        .arg("-all") // Add a single argument
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    // Or, to add multiple arguments at once:
    // let output = Command::new("./os-scraper")
    //     .args(["-all", "-verbose", "-debug"])
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(format!(
            "os-scraper failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}
