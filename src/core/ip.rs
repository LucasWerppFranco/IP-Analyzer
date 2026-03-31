use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct IpAddr {
    pub octets: [u8; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct IpWithCidr {
    pub ip: IpAddr,
    pub prefix: u8,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat,
    InvalidOctet,
    InvalidPrefix,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "Invalid IP format. Expected: x.x.x.x/y"),
            ParseError::InvalidOctet => write!(f, "Invalid octet value (must be 0-255)"),
            ParseError::InvalidPrefix => write!(f, "Invalid prefix (must be 0-32)"),
        }
    }
}

impl IpAddr {
    pub fn new(octets: [u8; 4]) -> Self {
        Self { octets }
    }

    pub fn from_u32(value: u32) -> Self {
        Self {
            octets: [
                ((value >> 24) & 0xFF) as u8,
                ((value >> 16) & 0xFF) as u8,
                ((value >> 8) & 0xFF) as u8,
                (value & 0xFF) as u8,
            ],
        }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.octets[0] as u32) << 24)
            | ((self.octets[1] as u32) << 16)
            | ((self.octets[2] as u32) << 8)
            | (self.octets[3] as u32)
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.octets[0], self.octets[1], self.octets[2], self.octets[3])
    }
}

pub fn parse(ip_str: &str) -> Result<IpWithCidr, ParseError> {
    let parts: Vec<&str> = ip_str.split('/').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidFormat);
    }

    let ip_part = parts[0];
    let prefix_part = parts[1];

    let prefix: u8 = prefix_part.parse().map_err(|_| ParseError::InvalidPrefix)?;
    if prefix > 32 {
        return Err(ParseError::InvalidPrefix);
    }

    let octets: Vec<&str> = ip_part.split('.').collect();
    if octets.len() != 4 {
        return Err(ParseError::InvalidFormat);
    }

    let mut result_octets = [0u8; 4];
    for (i, octet_str) in octets.iter().enumerate() {
        let octet: u8 = octet_str.parse().map_err(|_| ParseError::InvalidOctet)?;
        result_octets[i] = octet;
    }

    Ok(IpWithCidr {
        ip: IpAddr::new(result_octets),
        prefix,
    })
}
