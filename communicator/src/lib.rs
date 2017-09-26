pub mod client;
pub mod network;


#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
        let d = "123";
        let mut ds = d.to_string();
        ds.push_str(" hello");
        println!("{}", ds);
        let s3 = ds + d;
        println!("{}{}", s3, d);
        panic!("at the disco");
    }
}
