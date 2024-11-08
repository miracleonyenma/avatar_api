
# Avatar Generator API

This is a Rust-based API built with Rocket that generates and serves random avatars by downloading them from Tapback. Each visit to <https://www.tapback.co/api/avatar.webp> generates a new random avatar image, which this API downloads and stores locally, providing URLs for easy access.

## Features

- Downloads a specified number of random avatars.
- Provides URLs to download or view each generated avatar.
- Serves avatars from a local avatars folder.

## Attribution

- Avatars are sourced from Tapback's API, a free service that provides random avatars.

## Prerequisites

- Rust (installed via rustup)
- Cargo (included with Rust)
- Rocket dependencies for the server
- Optional: Git, if you wish to clone this repository

## Getting Started

### Step 1: Clone the Repository

```bash
git clone https://github.com/your-username/avatar-generator-api.git
cd avatar-generator-api
```

### Step 2: Install Dependencies

Make sure you have all the required dependencies for Rocket. This includes setting up rustup and adding the required toolchains.

```bash
rustup update
rustup default stable
```

### Step 3: Run the Server

To start the Rocket server:

```bash
cargo run
```

The server will start on `http://localhost:8000` by default.

## Usage

### Generate Avatars

You can generate a specified number of avatars by making a GET request to the `/generate_avatars` endpoint. For example, to generate 5 avatars:

```bash
curl "http://localhost:8000/generate_avatars?count=5"
```

#### Response Example

```json
{
  "avatars": [
    "/avatars/avatar_1.webp",
    "/avatars/avatar_2.webp",
    "/avatars/avatar_3.webp"
  ]
}
```

Each URL in the response can be accessed to view or download the generated avatars.

### Accessing Avatars

To access a specific avatar, use the `/avatars/<filename>` endpoint. For example, to view or download `avatar_1.webp`, go to:

`http://localhost:8000/avatars/avatar_1.webp`

## Deploying on Render

To deploy this API on Render, follow these steps:

1. Push this project to GitHub or GitLab.
2. Create a new Web Service on Render and connect it to the repository.
3. Set the build and start commands in Render:
   - Build Command: `cargo build --release`
   - Start Command: `cargo run --release`
4. Follow Render's Rocket setup guide for additional configuration, if needed.

Once deployed, you can access your API via `https://<your-render-app>.onrender.com`.

### Example API Requests on Render

To generate avatars:

```bash
curl "https://<your-render-app>.onrender.com/generate_avatars?count=5"
```

To download an avatar:

```bash
curl -O "https://<your-render-app>.onrender.com/avatars/avatar_1.webp"
```

## Project Structure

- `src/main.rs`: Main application file, containing routes and logic for generating and serving avatars.
- `avatars/`: Folder where avatars are saved. Created automatically when the server runs.

## Error Handling

If an avatar fails to download, it will be logged to the server's output. The API will continue processing other requests and include any successfully downloaded avatars in the response.

