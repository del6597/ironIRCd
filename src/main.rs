use std::net::{TcpStream, TcpListener};
use std::thread;

fn handle_client(stream: TcpStream) {
	println!("Someone connected!");
}

fn main() {
    println!("Starting Iron IRCd");

	let listener = TcpListener::bind("127.0.0.1:6667").unwrap();

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				thread::spawn(move || {
					handle_client(stream)
				});
			}
			Err(e) => { /* Connection Failed */ }
		}
	}
}
