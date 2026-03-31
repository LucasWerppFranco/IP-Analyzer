pub fn run(ip: String) {
    match crate::core::ip::parse(&ip) {
        Ok(parsed) => {
            let result = crate::core::subnet::calculate(parsed);
            crate::output::print_subnet_result(&result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
