# IP-Analyzer Documentation

## Overview

IP-Analyzer is a command-line tool for analyzing IP address ecosystems. It provides various commands to validate, calculate, and extract information from IP addresses and subnets.

## Architecture

The project follows a modular architecture separating CLI handling, core logic, and output formatting:

```
src/
├── main.rs          # Entry point
├── cli.rs           # Command-line argument parsing (Clap)
├── commands/        # CLI command handlers
│   ├── mod.rs       # Command dispatcher
│   └── subnet.rs    # Subnet command implementation
├── core/            # Core business logic
│   ├── mod.rs
│   ├── ip.rs        # IP address parsing and structures
│   └── subnet.rs    # Subnet calculations
├── menu/            # Interactive menu system (TUI)
└── output/          # Output formatting and display
```

## Commands

### `subnet <ip>/<prefix>`

Calculates subnet information for a given IP address with CIDR notation.

**Example:**
```bash
subnet 192.168.1.0/24
```

**Output:**
```
1st Valid IP -> 192.168.1.1
Last Valid IP -> 192.168.1.254
Broadcast -> 192.168.1.255
Mask -> 255.255.255.0
Quantity: 254
```

**How it works:**

1. **Parsing** (`core/ip.rs`):
   - Parses the input string in format `x.x.x.x/y`
   - Validates octets are in range 0-255
   - Validates prefix is in range 0-32
   - Returns an `IpWithCidr` struct

2. **Calculation** (`core/subnet.rs`):
   - Converts IP to 32-bit unsigned integer
   - Calculates subnet mask from prefix: `mask = 0xFFFFFFFF << (32 - prefix)`
   - Network address: `network = ip & mask`
   - Broadcast address: `broadcast = network | ~mask`
   - First host: `network + 1` (except for /31 and /32)
   - Last host: `broadcast - 1` (except for /31 and /32)
   - Host count: `broadcast - network - 1` (0 for /31 and /32)

3. **Output** (`output/mod.rs`):
   - Formats results in human-readable format
   - Displays first/last valid IPs, broadcast, mask, and host count

**Supported formats:**
- Class A networks: `10.0.0.0/8`
- Class B networks: `172.16.0.0/16`
- Class C networks: `192.168.1.0/24`
- Any CIDR prefix from /0 to /32

### Other Commands

- `calc <ip>` - Performs calculations on an IP address
- `validate <ip>` - Validates an IP address format
- `info <ip>` - Displays information about an IP address

## Running the Program

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- Cargo (included with Rust)

### Build

```bash
cargo build --release
```

The compiled binary will be at `target/release/IP-Analyzer`.

### Run with Cargo

```bash
# General format
cargo run -- <command> <arguments>

# Subnet command examples
cargo run -- subnet 192.168.1.0/24
cargo run -- subnet 10.0.0.0/8
cargo run -- subnet 172.16.0.0/16
```

### Run the compiled binary

After building:

```bash
./target/release/IP-Analyzer subnet 192.168.1.0/24
```

### Development mode

For development with debug output:

```bash
cargo run -- subnet 192.168.1.0/24
```

## Error Handling

The program validates inputs and provides clear error messages:

- Invalid IP format: `"Error: Invalid IP format. Expected: x.x.x.x/y"`
- Invalid octet value: `"Error: Invalid octet value (must be 0-255)"`
- Invalid prefix: `"Error: Invalid prefix (must be 0-32)"`

## License

See the project root for license information.
