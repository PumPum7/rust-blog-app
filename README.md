# Blogpost App

This is a blogpost application built with Rust using the Axum web framework.

## Technologies Used

- Rust 1.81
- Axum web framework
- SQLite for database
- Docker for containerization

## Features

- Create blog posts with text content
- Upload images for blog posts
- Set user avatars via URL
- View all posts in a feed

## How to Run

### Prerequisites

- Docker installed on your system

### Steps

1. Clone the repository

2. Build the Docker image:
   ```
   docker build -t blogpost-app .
   ```

3. Run the Docker container:
   ```
   docker run -p 3000:3000 blogpost-app
   ```

4. Access the application in your web browser at `http://localhost:3000/home`

## Project Structure

- `src/main.rs`: Main application code
- `src/handlers.rs`: Request handlers
- `src/state.rs`: Application state and database initialization
- `src/models.rs`: Data models
- `src/errors.rs`: Error handling
- `src/index.html`: HTML template for the home page
- `Dockerfile`: Docker configuration for building and running the app
- `images/`: Directory for storing uploaded images (created at runtime)

## API Endpoints

- GET `/`: Redirects to `/home`
- GET `/home`: Displays the home page
- POST `/submit`: Submits a new blog post
- GET `/posts`: Retrieves all blog posts

## Notes

- The application uses an SQLite database to store blog posts.
- Uploaded images are stored in the `images/` directory.
- The application runs on port 3000 by default.
