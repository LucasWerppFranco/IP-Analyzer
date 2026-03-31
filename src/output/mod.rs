use crate::core::subnet::SubnetResult;

pub fn print_subnet_result(result: &SubnetResult) {
    println!("1st Valid IP -> {}", result.first_host);
    println!("Last Valid IP -> {}", result.last_host);
    println!("Broadcast -> {}", result.broadcast);
    println!("Mask -> {}", result.mask);
    println!("Quantity: {}", result.host_count);
}
