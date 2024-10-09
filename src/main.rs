use serde::{Deserialize, Serialize};
use reqwest::{self, StatusCode};
use std::io::{self, Write};
use rpassword::read_password;

#[derive(Deserialize, Serialize, Debug)]
struct Blog {
    _id: Option<String>,
    title: String,
    author: String,
    content: String,
    date: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Comment {
    _id: Option<String>,
    post_id: String,
    user_id: String,
    user_picture: String,
    user_name: String,
    content: String,
    date: String,
}

type BlogsResponse = Vec<Blog>;
type CommentsResponse = Vec<Comment>;

fn get_blogs() -> Result<Vec<Blog>, Box<dyn std::error::Error>> {
    let url = "https://api.sundehakon.tech/Blogs";
    let response = reqwest::blocking::get(url)?.json::<BlogsResponse>()?;
    
    Ok(response)
}

fn delete_blog(blog_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.sundehakon.tech/Blogs/{}", blog_id);

    let mut username = String::new();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap(); 
    println!(); 

    let client = reqwest::blocking::Client::new();
    let response = client
        .delete(&url)
        .basic_auth(username, Some(password))
        .send()?;

    if response.status() == StatusCode::OK {
        println!("Blog deleted successfully!");
    } else {
        println!("Failed to delete blog. Status: {:?}", response.status());
        println!("Response Body: {:?}", response.text()?);
    }

    Ok(())
}

fn add_blog(blog: Blog) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.sundehakon.tech/Blogs";

    let mut username = String::new();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();
    
    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap(); 
    println!(); 

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .json(&blog)
        .basic_auth(username, Some(password))
        .send()?;

    if response.status() == StatusCode::CREATED {
        println!("Blog added successfully!");
    } else {
        println!("Failed to add blog. Status: {:?}", response.status());
        println!("Response Body: {:?}", response.text()?);
    }

    Ok(())
}

fn get_comments() -> Result<Vec<Comment>, Box<dyn std::error::Error>> {
    let url = "https://api.sundehakon.tech/Comments";
    let response = reqwest::blocking::get(url)?.json::<CommentsResponse>()?;
    
    Ok(response)
}

fn remove_comments(comment_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.sundehakon.tech/Comments/{}", comment_id);

    let mut username = String::new();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap(); 
    println!(); 

    let client = reqwest::blocking::Client::new();
    let response = client
        .delete(&url)
        .basic_auth(username, Some(password))
        .send()?;

    if response.status() == StatusCode::OK {
        println!("Comment deleted successfully!");
    } else {
        println!("Failed to delete comment. Status: {:?}", response.status());
        println!("Response Body: {:?}", response.text()?);
    }

    Ok(())
}

fn main() {
    println!("Welcome to blog-editor!");

    loop {
        println!("Choose an option:");
        println!("1. Create a new blog post");
        println!("2. View all blog posts");
        println!("3. Delete blog post by index");
        println!("4. View all comments");
        println!("5. Delete comment by index");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let mut title = String::new();
                let author = "Håkon Sunde"; 
                let mut content = String::new();
                let mut date = String::new();

                print!("Enter title: ");
                io::stdout().flush().unwrap(); 
                io::stdin().read_line(&mut title).unwrap();
                title = title.trim().to_string(); 

                print!("Enter content: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut content).unwrap();
                content = content.trim().to_string();

                print!("Enter blog date (YYYY-MM-DD): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut date).unwrap();
                date = date.trim().to_string();

                let new_blog = Blog {
                    _id: None,  
                    title,
                    author: author.to_string(),
                    content,
                    date,
                };

                if let Err(e) = add_blog(new_blog) {
                    eprintln!("Error adding blog: {}", e);
                }
            }
            "2" => {
                match get_blogs() {
                    Ok(blogs) => {
                        for blog in blogs {
                            println!("----------------------");
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
            "3" => {
                match get_blogs() {
                    Ok(blogs) => {
                        for (index, blog) in blogs.iter().enumerate() {
                            println!("{}. {} by {} on {}", index + 1, blog.title, blog.author, blog.date);
                        }
                        println!("Enter the number of the blog to delete:");
                        let mut delete_choice = String::new();
                        io::stdin().read_line(&mut delete_choice).unwrap();
                        let delete_choice: usize = delete_choice.trim().parse().unwrap_or(0);

                        if delete_choice > 0 && delete_choice <= blogs.len() {
                            let blog_id = &blogs[delete_choice - 1]._id.as_deref().unwrap();
                            if let Err(e) = delete_blog(blog_id) {
                                eprintln!("Error deleting blog: {}", e);
                            }
                        } else {
                            println!("Invalid selection.");
                        }
                    },
                    Err(e) => eprintln!("Error fetching blogs: {}", e),
                }
            }
            "4" => {
                match get_comments() {
                    Ok(comments) => {
                        for comment in comments {
                            println!("----------------------");
                            println!("Post ID: {}", comment.post_id);
                            println!("User ID: {}", comment.user_id);
                            println!("User Picture: {}", comment.user_picture);
                            println!("User Name: {}", comment.user_name);
                            println!("Content: {}", comment.content);
                            println!("Date: {}", comment.date);
                            println!("----------------------");
                        }
                    },
                    Err(e) => eprintln!("Error fetching comments: {}", e),
                }
            }
            "5" => {
                match get_comments() {
                    Ok(comments) => {
                        for (index, comment) in comments.iter().enumerate() {
                            println!("{}. {} by {} on {}", index + 1, comment.content, comment.user_name, comment.date);
                        }
                        println!("Enter the number of the comment to delete:");
                        let mut delete_choice = String::new();
                        io::stdin().read_line(&mut delete_choice).unwrap();
                        let delete_choice: usize = delete_choice.trim().parse().unwrap_or(0);

                        if delete_choice > 0 && delete_choice <= comments.len() {
                            let comment_id = &comments[delete_choice - 1]._id.as_deref().unwrap();
                            if let Err(e) = remove_comments(comment_id) {
                                eprintln!("Error deleting comment: {}", e);
                            }
                        } else {
                            println!("Invalid selection.");
                        }
                    },
                    Err(e) => eprintln!("Error fetching comments: {}", e),
                }
            }
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}
