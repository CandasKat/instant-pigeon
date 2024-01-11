#[cfg(test)]
mod tests {
    use crate::server;

    #[test]
    fn test_accept_connection() {
        let server = server::Server::new();
        assert_eq!(server.accept_connection(), "Connection accepted!");
    }

    #[test]
    fn test_message_routing() {
        let server = server::Server::new();
        assert_eq!(server.route_message("Hello"), "Message routed!");
    }

    #[test]
    fn test_connection_management() {
        let server = server::Server::new();
        assert_eq!(server.manage_connection(), "Connection managed!");
    }
}