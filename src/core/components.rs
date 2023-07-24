#[derive(Clone)]
pub struct HashComponents {
    pub master_key: String,
    pub salt: String,
    pub iterations_count: i32,
}

impl HashComponents {
    pub fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.clone().split('$').collect();
        
        let master_key: String = parts[3].to_string();
        let salt: String = parts[5].to_string();
        let iterations_count: i32 = parts[6].to_string().parse::<i32>().unwrap();

        HashComponents { master_key, salt, iterations_count }
    }
}

