
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
    Joker(char),
}

#[derive(Debug)]
struct User {
    user_id: i32,
    user_name: String,
}

#[derive(Debug)]
struct Table {
    table_id: i32,
    users: (User, User, User),
}

struct GameServer {
    GamePool: Vec<Table>,
}

impl GameServer {
    pub fn deal_input(&self, data: &str) -> &str {
        println!("{:?}", data);
        match data {
            "help" =>{
                "list, create, join:{id}"
            }
            "list" => {
                for table in &self.GamePool {
                    println!("{:?}", table);
                }
                ""
            },
            "create"=>{
                ""
            }
            x if x.starts_with("join:")=>{
                ""
            }
            x if x.starts_with("create:") => "",
            x if x.starts_with("ready:") => {
                return "success ready";
            }
            _ => {
                println!("not cover {:?}", data);
                return "";
            }
        }
    }

    pub fn new() -> Self {
        // self.GamePool = Vec::new();
        Self {
            GamePool: Vec::new(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:12345").await?;
    let gameserver = GameServer::new();
    let game = Arc::new(gameserver);
    loop {
        let (mut socket, socket_addr) = listener.accept().await?;
        println!("{:?}", socket);
        println!("{:?}", socket_addr);
        let game = Arc::clone(&game);
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                eprintln!("get data {:?}", &buf[0..n]);
                let str_slice = std::str::from_utf8(&buf[0..n]).unwrap().trim();
                game.deal_input(&str_slice);
                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
        println!("{:?}", listener);
    }
}

