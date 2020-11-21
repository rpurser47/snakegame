fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn website_title() {
        use selenium_rs::webdriver::{Browser,WebDriver};

        let mut driver= WebDriver::new(Browser::Chrome);
        let _error1 = driver.start_session();
        
        let _error2 = driver.navigate("https://www.rust-lang.org"); 
        assert_eq!(driver.get_current_url().unwrap(), String::from("https://www.rust-lang.org/"));
       }
}