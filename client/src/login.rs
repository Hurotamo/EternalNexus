// Constants for the login server IP address and port
pub const LOGIN_IP_ADDR: &str = "127.0.0.1";
pub const LOGIN_PORT: u16 = 8080;

// Struct to represent the login server configuration
pub struct LoginServerConfig {
    pub ip_addr: String,
    pub port: u16,
}

impl LoginServerConfig {
    // Function to create a new login server configuration
    pub fn new(ip_addr: String, port: u16) -> Self {
        LoginServerConfig { ip_addr, port }
    }

    // Function to write the login server IP address and port
    pub fn write_login_server_config(&self) -> String {
        format!("{}:{}", self.ip_addr, self.port)
    }
}