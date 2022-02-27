use std::collections::HashMap;

pub fn run(){
    let mut reviews: HashMap<String, String> = HashMap::new();
    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate"));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes"));
    reviews.insert(String::from("Programming in Rust"), String::from("Great Examples"));

    // Look for a specific review
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}': {:?}", book, reviews.get(book));

    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' Removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\' {:?}", obsolete, reviews.get(obsolete));

}
