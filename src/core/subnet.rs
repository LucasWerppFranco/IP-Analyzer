use crate::core::ip::{IpAddr, IpWithCidr};

#[derive(Debug)]
pub struct SubnetResult {
    pub network: IpAddr,
    pub broadcast: IpAddr,
    pub first_host: IpAddr,
    pub last_host: IpAddr,
    pub mask: IpAddr,
    pub host_count: u32,
    pub prefix: u8,
}

pub fn calculate(ip_with_cidr: IpWithCidr) -> SubnetResult {
    let prefix = ip_with_cidr.prefix;
    let ip_u32 = ip_with_cidr.ip.to_u32();

    let mask_u32 = if prefix == 0 {
        0
    } else {
        (!0u32) << (32 - prefix)
    };

    let network_u32 = ip_u32 & mask_u32;
    let broadcast_u32 = if prefix == 32 {
        network_u32
    } else {
        network_u32 | (!mask_u32)
    };

    let first_host_u32 = if prefix == 32 {
        network_u32
    } else if prefix >= 31 {
        network_u32
    } else {
        network_u32 + 1
    };

    let last_host_u32 = if prefix == 32 {
        broadcast_u32
    } else if prefix >= 31 {
        broadcast_u32
    } else {
        broadcast_u32 - 1
    };

    let host_count = if prefix >= 31 {
        0
    } else {
        (broadcast_u32 - network_u32 - 1)
    };

    SubnetResult {
        network: IpAddr::from_u32(network_u32),
        broadcast: IpAddr::from_u32(broadcast_u32),
        first_host: IpAddr::from_u32(first_host_u32),
        last_host: IpAddr::from_u32(last_host_u32),
        mask: IpAddr::from_u32(mask_u32),
        host_count,
        prefix,
    }
}
