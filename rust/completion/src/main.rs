use::std::net::TcpListener;
use::std::io::{Read,Write};
use::std::sync::Arc;

mod dictionary;
mod handler;

use crate::dictionary::create_dictionary;
use crate::handler::handle_request;


fn main() {
    println!("Hello, world!");

    let port = 8000;
    let address = format!("127.0.0.1:{}", port);

    let dictionary = Arc::new(create_dictionary());

    let listener = TcpListener::bind(&address)
        .expect("failed to bind port");
    
    println!("server started on {}", port);

    for stream in listener.incoming()
    {
        match stream
        {
            Ok(mut stream) => 
            {
                let dict_clone = Arc::clone(&dictionary);
                // Thread ici non optimisé mais demande autre bibliothèques
                std::thread::spawn(move || 
                {
                    let mut buffer = [0; 1024];
                    if let Ok(n) = stream.read(&mut buffer)
                    {
                        let request = String::from_utf8_lossy(&buffer[..n]);
                        let response = handle_request(&request, &dict_clone);
                        let _ = stream.write_all(response.as_bytes());
                    }   

                });
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
}
