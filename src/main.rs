use notify_rust::{Notification, Timeout};
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await.unwrap();
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    // Navigate to cash or trade
    driver
        .goto("https://cashortrade.org/tickets")
        .await
        .unwrap();
    let posts: Vec<WebElement> = driver.find_all(By::ClassName("postItem")).await.unwrap();
    for post in &posts {
        let title = post.find(By::ClassName("postItem-title")).await.unwrap();
        let post_name = title.inner_html().await.unwrap();
        if post_name.to_lowercase().contains("renewal") {
            println!("New post for renewal tickets!");
            // need to give beep permissions?
            Notification::new()
                .summary("Cashortrade")
                .body("Billy ticket post!")
                .icon("firefox")
                .timeout(Timeout::Milliseconds(6000)) //milliseconds
                .show()
                .unwrap();
        }
    }
}
}
