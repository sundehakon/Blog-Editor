use serde::Deserialize;
use reqwest;

#[derive(Deserialize, Debug)]
struct Blog {
    _id: String,
    title: String,
    author: String,
    content: String,
    date: String,
}

type BlogsResponse = Vec<Blog>;

fn get_blogs() -> Result<Vec<Blog>, Box<dyn std::error::Error>> {
    let url = format!("https://portfolioapi-hysa.onrender.com/Blogs");

    let response = reqwest::blocking::get(&url)?.json::<BlogsResponse>()?;
    
    Ok(response)
}

fn main() {
    match get_blogs() {
        Ok(blogs) => {
            for blog in blogs {
                println!("Title: {}", blog.title);
                println!("Author: {}", blog.author);
                println!("Content: {}", blog.content);
                println!("Date: {}", blog.date);
                println!("----------------------");
            }
        },
        Err(e) => eprintln!("Error fetching blogs: {}", e),
    }
}
