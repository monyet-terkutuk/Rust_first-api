use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use serde::de::Expected;
use std::fmt::format;
use std::net::{TcpListener,TcpStream};
use std::io::{BufRead, Read, Write};
use std::env;

// #[macro_use]
// extern crate serde_derive;

// // Model: User struct with id, name, email
// #[derive(Serialize, Deserialize)]
// struct User{
//     id: Option<i32>,
//     name: String,
//     email: String
// }

// // Database URL
// const DB_URL: &str = !env("DATABASE_URL");

// // constants response
// const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
// const NOT_FOUND : &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
// const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

// // main function
// fn main(){
//     // set db
//     if let Err(e) = set_database(){
//         println!("Error: {}", e);
//         return;
//     }

//     // start server and print port
//     let listener = TcpListener::bind(format(0.0.0.0:8080)).unwrap();
//     println!("Server running in port 8080");

//     // handle the client
//     for stream in listener.incoming(){
//         match stream {
//             OK(stream) =>{
//                 handle_client(stream);
//             }
//             Err(e) => {
//                 println!("Error: {}", e);
//             }
//         }
//     }
// }

// // set_database function
// fn set_database() -> Result<(), PostgresError> {
//     let mut client = Client::connect(DB_URL, NoTls)?;

//     // Create Table
//     client.execute(
//         "CREATE TABLE IF NOT EXISTS users(
//             id SERIAL PRIMARY KEY,
//             name VARCHAR NOT NULL,
//             email VARCHAR NOT NULL
//         )", &[])?;
// }

#[macro_use]
extern crate serde_derive;

// Model: User struct with id, name, email
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String
}

// Database URL
const DB_URL: &str = env!("DATABASE_URL");

// Constants response
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

// Main function
fn main() {
    // Set up database
    if let Err(e) = set_database() {
        println!("Error: {}", e);
        return;
    }

    // Start server and print port
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server running on port 8080");

    // Handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

// Handle client function
fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if request_with("POST /users") => handle_post_request(r),
                r if request_with("GET /users") => handle_get_request(r),
                r if request_with("PUT /users") => han
            };
        }
        Err(e) =>{
            println!("Error:")
        }
    }
}

// Set up database function
fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;

    // Create Table
    client.execute(
        "CREATE TABLE IF NOT EXISTS users(
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )", &[])?;

    Ok(())
}

// Handle client function
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response = OK_RESPONSE;
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Get id function
fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

// Deserialize user from request body with the id
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error>{
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}