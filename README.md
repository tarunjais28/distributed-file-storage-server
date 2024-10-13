
# File Upload/Download Service

This project provides a file upload and download service built with Rust using Actix Web. The service supports uploading large files in chunks, saving them to a PostgreSQL database, and then retrieving them by file ID. It also provides metadata about the uploaded files, such as the file name and chunk count.

## Features

- **File Upload**: Allows users to upload files in chunks.
- **File Download**: Retrieve files by file ID.
- **File Metadata**: Fetch metadata about uploaded files, such as file name and chunk count.
- **Database Integration**: Uses PostgreSQL for storing file chunks and metadata.
- **Concurrency**: The application processes file chunks in parallel using Rust's thread model.
- **Logging**: Provides detailed logs via the `env_logger` library.

## Prerequisites

Before running the application, ensure the following tools are installed:

- Docker and Docker Compose installed.
- A PostgreSQL database configured (or use the provided Docker setup for local development).

## Project Structure

- **main.rs**: The main entry point where the HTTP server and services are defined.
- **FileMetadata**: A struct that represents file metadata (ID, name, chunk count).
- **upload_file**: API to upload a file and save its chunks to the database.
- **get_files**: API to retrieve a list of file metadata.
- **download_file**: API to download a file by ID.
- **CustomError**: A custom error handling enum used throughout the service.
- **Dockerfiles**: Contains both a `Dockerfile` and `docker-compose.yaml` for easy deployment in containers.

## Setting up the Application with Docker

### 1. Clone the repository
Clone the repository to the local machine:
```bash
git clone https://github.com/tarunjais28/distributed-file-storage-server.git
cd distributed-file-storage-server
```

### 2. Build and Run with Docker Compose

Run the following command to build and start the application and PostgreSQL containers:

```bash
cd distributed-file-storage-server
docker compose build
docker compose up
```

This command will build the Docker images and start the services defined in the `docker-compose.yaml` file.

### 3. Access the Application

Once the containers are running, the application will be available at `http://localhost:8080`.

The following APIs can be used:

- **Upload File**: `POST /upload` to upload files.
```bash
curl --location 'localhost:8080/upload' \
--header 'Content-Type: application/octet-stream' \
--form 'file=@"<path-to-the-file>"'
```

- **Get Files**: `GET /files` to retrieve file metadata.
```bash
curl --location 'localhost:8080/files'
```

- **Download File**: `GET /download/{file_id}` to download a file by ID.
```bash
curl --location 'localhost:8080/download/<file-id>' \
--header 'Content-Type: application/json'
```

The file will be downloaded inside the downloads folder at path distributed-file-storage-server/downloads.

### 4. Stop the Application

To stop the application, use the following command:

```bash
docker-compose down
```

## Database Schema

The database consists of two main tables:

- **files**: Stores file metadata such as ID, name, and chunk count.
- **chunks**: Stores each file chunk, associated with a file ID and chunk number.

### Example File Metadata (JSON Response)

```json
{
  "id": "b9fa0c8b-25b6-4d38-b40f-47adac342db7",
  "name": "example.txt",
  "chunk_count": 5
}
```

## Error Handling

The application uses custom error types to handle different error scenarios, including I/O errors, database errors, and connection issues. These errors are logged for debugging purposes.
