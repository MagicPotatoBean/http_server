use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
mod http;
fn main() {
    println!("Please enter private address in the form \"a.b.c.d\"");
    let mut response = String::default();
    if std::io::stdin().read_line(&mut response).is_ok() {
        let mut ip_iter = response.trim().split(".");
        if let (Some(a_str), Some(b_str), Some(c_str), Some(d_str)) = (
            ip_iter.next(),
            ip_iter.next(),
            ip_iter.next(),
            ip_iter.next(),
        ) {
            if let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
                a_str.parse::<u8>(),
                b_str.parse::<u8>(),
                c_str.parse::<u8>(),
                d_str.parse::<u8>(),
            ) {
                let http_pub_addr =
                    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), 80));
                std::thread::Builder::new()
                    .name("HTTP Server".to_owned())
                    .spawn(move || http::host_server(http_pub_addr, 128).unwrap())
                    .unwrap();
                let _ = std::io::stdin().read_line(&mut String::default());
                // Block until "\n" received on stdin
            }
        }
    }
}
