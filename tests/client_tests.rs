#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;
    use http_rustclient::client::run_example_connection;
    
    #[test]
    fn test_example_connection() {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(run_example_connection());

        assert!(result.is_ok());
    }
}
