#[cfg(test)]
mod tests {
    use super::*;
    use crate::client;

    #[test]
    fn test_accept_connection() {
        let server = client::Client::new();
        assert_eq!(server.accept_connection(), "Connection accepted!");
    }

    #[test]
    fn test_message_routing() {
        let server = client::Client::new();
        assert_eq!(server.route_message("Hello"), "Message routed!");
    }

    #[test]
    fn test_connection_management() {
        let server = client::Client::new();
        assert_eq!(server.manage_connection(), "Connection managed!");
    }
}