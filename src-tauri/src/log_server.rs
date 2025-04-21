use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// Shared buffer for storing text from TCP clients
static LOG_BUFFER: once_cell::sync::Lazy<Arc<Mutex<String>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(String::with_capacity(1_000_000))));

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 8192];
    while match stream.read(&mut buffer) {
        Ok(size) if size > 0 => {
            println!("Received {size} data");
            // Append received text to the shared buffer
            if let Ok(text) = String::from_utf8(buffer[0..size].to_vec()) {
                println!("Locking");
                if let Ok(mut log_buffer) = LOG_BUFFER.lock() {
                    println!("Got lock");
                    log_buffer.push_str(&text);
                }
            }
            true
        }
        Ok(_) => false, // Connection closed by client
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // No incoming data, wait a bit before checking again
            thread::sleep(std::time::Duration::from_millis(100));
            true
        }
        Err(e) => {
            println!("Error occurred, terminating connection: {}", e);
            // Send confirmation before closing
            let _ = stream.write(b"Connection closed\n").unwrap();
            false
        }
    } {}
}

// Define a struct to return both the sender and thread handle
pub struct TcpServer {
    pub stop: std::sync::mpsc::Sender<()>,
    pub thread: std::thread::JoinHandle<()>,
}

// impl TcpServer {
//     fn stop_and_wait(self) {
//         self.stop.send(()).unwrap();
//         self.thread.join().unwrap();
//     }
// }

pub fn start_tcp_server() -> TcpServer {
    const PORT: u16 = 54560;

    let (tx, rx) = std::sync::mpsc::channel::<()>();

    let thread = thread::spawn(move || {
        let listener = match TcpListener::bind(format!("127.0.0.1:{PORT}")) {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind to port {PORT}: {}", e);
                return;
            }
        };

        println!("TCP server listening on port {PORT}");

        // Set listener to non-blocking to check for stop signal
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");

        loop {
            // Check if we've received the stop signal
            if rx.try_recv().is_ok() {
                println!("Shutting down TCP server");
                break;
            }

            match listener.accept() {
                Ok((stream, _)) => {
                    println!("New connection established");
                    // stream will be nonblocking if listener is
                    handle_client(stream);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No incoming connection, wait a bit before checking again
                    thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    });

    TcpServer { stop: tx, thread }
}

// Add a command to get the current buffer contents
#[tauri::command]
pub fn get_log_buffer() -> String {
    LOG_BUFFER.lock().unwrap().clone()
}

// Test module for the TCP server functionality
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_tcp_server() {
        // Start the TCP server in a separate thread
        let server = start_tcp_server();

        // Stop server on panic
        let old_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            eprintln!("Test panicked: {}", panic_info);
            let _ = server.stop.send(()).unwrap();
            old_hook(panic_info);
        }));

        // Give the server some time to start
        thread::sleep(Duration::from_millis(100));

        // Connect to the server
        match TcpStream::connect("127.0.0.1:54560") {
            Ok(mut stream) => {
                // Send data to the server
                let test_message = "Hello, TCP server!";
                stream
                    .write_all(test_message.as_bytes())
                    .expect("Failed to write to server");

                // Give the server some time to process
                thread::sleep(Duration::from_millis(100));

                // Verify the message was added to the buffer
                let buffer_content = get_log_buffer();
                assert!(
                    buffer_content.contains(test_message),
                    "Buffer should contain our test message"
                );

                // Close the connection
                drop(stream);
            }
            Err(e) => {
                panic!("Failed to connect to TCP server: {}", e);
            }
        }
    }

    #[test]
    fn test_multiple_messages() {
        let server = start_tcp_server();

        // Stop server on panic
        let old_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            eprintln!("Test panicked: {}", panic_info);
            server.stop.send(()).unwrap();
            old_hook(panic_info);
        }));

        // Ensure server is running
        thread::sleep(Duration::from_millis(100));

        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:54560") {
            // Clear the buffer first by getting its current state
            let _ = get_log_buffer();

            // Send multiple messages
            let messages = vec!["Message 1", "Message 2", "Message 3"];

            for msg in &messages {
                stream
                    .write_all(msg.as_bytes())
                    .expect("Failed to write to server");
                thread::sleep(Duration::from_millis(50));
            }

            println!("Wrote all data. Waiting a bit");
            thread::sleep(Duration::from_millis(50));

            // Check if all messages are in the buffer
            let buffer_content = get_log_buffer();
            for msg in messages {
                assert!(
                    buffer_content.contains(msg),
                    "Buffer should contain message: {}",
                    msg
                );
            }
        } else {
            panic!("Could not connect to TCP server");
        }
    }
}
