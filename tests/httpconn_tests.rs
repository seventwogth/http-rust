#[cfg(test)]
mod tests {
    use http_rustclient::httpconn::build_http_get_request;

    #[test]
    fn build_http_get_request_should_return_validrequest() {
        let result = build_http_get_request("host", "path");
            assert_eq!(result, "GET path HTTP/1.1\r\nHost: host\r\nConnection: close\r\n\r\n")
    }
}