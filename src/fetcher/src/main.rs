use std::{error::Error, path::Path, time::Duration};

use clap::{command, Parser};
use thirtyfour::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    url: String,

    #[arg(long, default_value_t = 0)]
    scroll: u32,

    #[arg(long, default_value_t = 0)]
    sleep: u64,

    #[arg(long, default_value_t = false)]
    image: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.set_window_rect(0, 0, 1024, 1024).await?;

    driver.goto(&args.url).await?;
    driver
        .execute(format!("window.scrollBy(0,{})", args.scroll), Vec::new())
        .await?;

    tokio::time::sleep(Duration::from_secs(args.sleep)).await;

    if args.image {
        driver.screenshot(Path::new("temp.png")).await?;
    } else {
        let data = driver.screenshot_as_png_base64().await?;
        println!("{}", data);
    }

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
