use std::net::Ipv4Addr;

pub fn calculate_subnet(ip: &str, cidr: u8) -> Option<String> {
    if !super::ipv4::validate(ip) {
        return None;
    }

    let ip: Ipv4Addr = ip.parse().ok()?;

    let prefix_mask = !((1 << (32 - cidr)) - 1);
    let network = u32::from(ip) & prefix_mask;

    let broadcast = network | !prefix_mask;
    let first_ip = network + 1;
    let last_ip = broadcast - 1;
    let num_hosts = u32::from(broadcast - network) - 1;

    let subnet_mask = Ipv4Addr::from(prefix_mask);
    let wildcard_mask = Ipv4Addr::from(!prefix_mask);

    Some(format_subnet_info(
        ip, cidr, Ipv4Addr::from(network), Ipv4Addr::from(broadcast),
        Ipv4Addr::from(first_ip), Ipv4Addr::from(last_ip), subnet_mask, wildcard_mask, num_hosts,
    ))
}

pub fn calculate_subnet_with_mask(ip: &str, mask: &str) -> Option<String> {
    if !super::ipv4::validate(ip) || !super::ipv4::validate(mask) {
        return None;
    }

    let subnet_mask: Ipv4Addr = mask.parse().ok()?;
    let prefix_mask = u32::from(subnet_mask);

    let cidr = prefix_mask.count_ones() as u8;

    Some(calculate_subnet(ip, cidr)?)
}

fn format_subnet_info(
    ip: Ipv4Addr, 
    cidr: u8, 
    network: Ipv4Addr, 
    broadcast: Ipv4Addr, 
    first_ip: Ipv4Addr, 
    last_ip: Ipv4Addr, 
    subnet_mask: Ipv4Addr, 
    _wildcard_mask: Ipv4Addr, // meh
    num_hosts: u32
) -> String {
    format!(
        "IP: {}\nCIDR: /{}\nSubnet Mask: {}\nNetwork: {}\nBroadcast: {}\nUsable IP range: {} - {}\nNumber of Hosts: {}",
        ip, cidr, subnet_mask, network, broadcast, first_ip, last_ip, num_hosts
    )
}