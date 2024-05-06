use crate::{
    http::{http_methods::get, http_request::HttpRequest},
    http_log,
};
use std::{
    net::{SocketAddr, TcpListener, TcpStream},
    sync::Arc,
    thread::{self},
    time::Duration,
};

pub mod http_methods;
pub mod http_request;
static PATH: &str = "./static";
/// Creates a TcpListener on the provided address, accepting all incoming requests and sending the request to
/// ```no_run
/// handle_connection()
/// ```
/// to respond
/// # Errors
/// Returns an IO error if the TcpListener fails to bind to the requested address.
pub fn host_server(priv_addr: SocketAddr, max_threads: usize) -> std::io::Result<()> {
    let listener = TcpListener::bind(priv_addr)?;
    let thread_count: Arc<()> = Arc::new(()); // Counts the number of threads spawned based on the weak count
    http_log!("==================== HTTP Server running on {priv_addr} ====================");
    for client in listener.incoming().flatten() {
        if Arc::strong_count(&thread_count) <= max_threads {
            /* Ignores request if too many threads are spawned */
            let passed_count = thread_count.clone();
            if thread::Builder::new()
                .name("ClientHandler".to_string())
                .spawn(move || handle_connection(passed_count, client))
                .is_err()
            {
                /* Spawn thread to handle request */
                http_log!("Failed to spawn thread");
            }
        }
    }

    drop(thread_count);
    Ok(())
}
/// Takes in a threadcounter and TcpStream, reading the entire TCP packet before responding with the requested data. The `thread_counter` variable is dropped at the end of the function, such that the strong count represents the number of threads spawned.
fn handle_connection(thread_counter: Arc<()>, client: TcpStream) {
    http_log!(
        "{} Thread(s) active.",
        Arc::strong_count(&thread_counter) - 1
    );
    let client_ip = client.peer_addr();
    client
        .set_read_timeout(Some(Duration::from_millis(100)))
        .expect("Should set read timeout");
    let mut packet = HttpRequest::new(client);
    if let Some(protocol) = packet.protocol() {
        match protocol.as_str() {
            "HTTP/1.1" | "undefined" => {
                if let Some(method) = packet.method() {
                    if let Ok(ip) = client_ip {
                        http_log!("Client {ip} made a {method} request");
                    } else {
                        http_log!("Client made a {method} request");
                    }
                    match method.to_lowercase().trim() {
                        "get" => get(packet, PATH),

                        _ => {
                            http_log!("Invalid method, request ignored.");
                            let _ = packet.respond_string("HTTP/1.1 405 Method Not Allowed\r\n\r\nUnknown request method. Allowed methods: \"GET\".\r\n");
                        }
                    }
                } else {
                    http_log!("No method provided");
                    let _ = packet.respond_string("HTTP/1.1 405 Method Not Allowed\r\n\r\nUnknown request method. Allowed methods: \"GET\".\r\n");
                }
            }
            proto => {
                http_log!("Client used invalid protocol: \"{proto}\"");
                let _ = packet.respond_string("Unknown protocol.");
            }
        }
    } else {
        http_log!("Client provided no protocol.");
    }

    drop(thread_counter); // Decrements the counter
}

#[macro_export]
macro_rules! http_log {
    () => {
        use std::io::Write;
        let current_time: DateTime<Utc> = Utc::now();
        std::fs::OpenOptions::new().append(true).open("logs/http.log").expect("Failed to open http.log file").write_all(format!("[{} UTC] {}:{}:{}\n", current_time.format("%Y-%m-%d %H:%M:%S"), file!(), line!(), column!()).as_bytes()).expect("Failed to write to log file");
        println!("[{} UTC] {}:{}:{}", current_time.format("%Y-%m-%d %H:%M:%S"), file!(), line!(), column!());
    };
    ($($arg:tt)*) => {{
        use std::io::Write;
        let current_time: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        std::fs::OpenOptions::new().append(true).open("logs/http.log").expect("Failed to open http.log file").write_all(format!("[{} UTC] {}:{}:{}: {}\n", current_time.format("%Y-%m-%d %H:%M:%S"), file!(), line!(), column!(), format!($($arg)*)).as_bytes()).expect("Failed to write to log file");
        println!("[{} UTC] {}:{}:{}: {}", current_time.format("%Y-%m-%d %H:%M:%S"), file!(), line!(), column!(), format!($($arg)*));
    }};
}
