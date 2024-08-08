pub fn validate(ip: &str) -> bool {
    ip.parse::<std::net::IpAddr>().is_ok()
}
