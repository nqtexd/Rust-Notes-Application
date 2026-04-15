use serde::{Deserialize, Serialize};
use std::thread;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Note{
    id: usize,
    content: String
}
fn main(){

    let stream = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in stream.incoming(){
        let stream = stream.unwrap();
        thread::spawn(move || {
            build_server(stream);
        });
    }
}

fn build_server(mut stream: TcpStream){

    let mut buffer = [0; 4096];
    let byte_read = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..byte_read]);

    let mut sec = request.split("\r\n\r\n");

    let header = sec.next().unwrap().lines().next().unwrap();

    let method = header.split(" ").next().unwrap();
    let path = header.split(" ").nth(1).unwrap_or("/");

    if method == "GET" && path == "/api/list" {
        let file = fs::read("data.json").unwrap();
        let json = String::from_utf8(file).unwrap();
        let res = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}", json.len(), json);
        stream.write_all(res.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    if method == "POST" && path == "/api/add" {
        let body = sec.next().unwrap_or("");
        
        let data: Result<Note, _> = serde_json::from_str(body);

        let content = match data {
            Ok(note) => note.content,
            Err(e) => {
                println!("error caught {:?}", e);
                "".to_string()
            }
        };

        let json_data = fs::read_to_string("data.json").unwrap(); //reading the data file
        let mut notes_data: Vec<Note> = serde_json::from_str(&json_data).unwrap_or(vec![]);
        let id = notes_data.iter().map(|n| n.id).max().unwrap_or(0) + 1;
        let new_note = Note { id, content };
        notes_data.push(new_note);
        let u_json = serde_json::to_string_pretty(&notes_data).unwrap();
        fs::write("data.json", u_json).unwrap();

        let bod = serde_json::to_string(&notes_data).unwrap();
        let res = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}", bod.len(), bod);
        stream.write_all(res.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }

    if method == "GET" && path == "/" {

        let file = fs::read("index.html").unwrap();
        let f = String::from_utf8(file).unwrap();
        let res = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}", f.len(), f);
        stream.write_all(res.as_bytes()).unwrap();
        return;
    }
}