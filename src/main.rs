#[macro_use] extern crate rocket;

use rocket::fs::{NamedFile};
use rocket::serde::json::Json;
use reqwest::Client;
use serde::Serialize;
use std::fs::{create_dir_all};
use std::path::{Path, PathBuf};
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;

#[derive(Serialize)]
struct AvatarResponse {
    avatars: Vec<String>,
}

#[get("/generate_avatars?<count>")]
async fn generate_avatars(count: Option<usize>) -> Json<AvatarResponse> {
    // Set default count to 1 if not specified
    let count = count.unwrap_or(1);
    
    // Ensure the "avatars" folder exists
    let folder = PathBuf::from("avatars");
    create_dir_all(&folder).unwrap();

    // HTTP client to download avatars
    let client = Client::new();
    let mut avatars_urls = Vec::new();

    // Download specified number of avatars asynchronously
    for i in 1..=count {
        match download_avatar(i, &folder, &client).await {
            Ok(path) => {
                // Convert local path to URL format for download
                let url = format!("/avatars/{}", path.file_name().unwrap().to_string_lossy());
                avatars_urls.push(url);
            },
            Err(e) => eprintln!("Failed to download avatar {}: {}", i, e),
        }
    }

    // Return the URLs of the downloaded avatars
    Json(AvatarResponse { avatars: avatars_urls })
}

async fn download_avatar(id: usize, folder: &PathBuf, client: &Client) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // URL for random avatar generation
    let url = "https://www.tapback.co/api/avatar.webp";
    
    // Send a GET request to download the avatar
    let response = client.get(url).send().await?.bytes().await?;

    // Create the file path in the specified folder with unique filenames
    let file_path = folder.join(format!("avatar_{}.webp", id));
    let mut file = TokioFile::create(&file_path).await?;

    // Write the downloaded content into the file asynchronously
    file.write_all(&response).await?;

    // Return the file path
    Ok(file_path)
}

// Route to serve the avatar images
#[get("/avatars/<filename>")]
async fn get_avatar(filename: &str) -> Option<NamedFile> {
    let path = Path::new("avatars").join(filename);
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![generate_avatars, get_avatar])
}