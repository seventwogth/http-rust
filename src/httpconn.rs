pub fn build_http_get_request(host: &str, path: &str) -> String {
    format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    )
}
