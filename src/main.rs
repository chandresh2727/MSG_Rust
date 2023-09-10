use reqwest::multipart;
use reqwest::Client;
use std::error::Error;
use std::io::{self, Write};
use std::fs::File;
use std::path::PathBuf;
use reqwest::multipart::Form;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read user input for the message
    let mut message = String::new();
    println!("Enter your message:");
    io::stdin().read_line(&mut message)?;

    // Remove leading/trailing whitespace and newlines
    message = message.trim().to_string();

    // Read user input for the file path
    let mut file_path = String::new();
    println!("Enter the file path:");
    io::stdin().read_line(&mut file_path)?;

    file_path = file_path.trim().to_string();

    let client = Client::new();


    let mut form = multipart::Form::new();
    let file = File::open(&file_path)?;
    let filename = PathBuf::from(&file_path).file_name().unwrap().to_string_lossy().to_string();
    form = form.file("file", &file_path)?;

    // Send the API request to upload the file with a message
    let response = client
    // here is u r Server Uploaded File Format 
        .post("https://example.com/upload")
        .multipart(form)
        .text("message", message)
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        println!("File uploaded successfully!");
    } else {
        println!("File upload failed. Status code: {:?}", response.status());
    }

    Ok(())
}


// fn main() {
//     println!("Hello, world!");
// }
