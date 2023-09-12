#[cfg(test)]
mod tests {
    use trustaction_cli::utils::reader::TrustactionEnv;


    #[test]
    fn get_env_struct() {
        match TrustactionEnv::initialise() {
            Ok(env) => println!("{:#?}", env),
            Err(e) => println!("ERROR: {}", e)
        }
    }
}