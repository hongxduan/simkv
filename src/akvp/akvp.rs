///! Advance Key-Value Protocol implement
///!
///! author: Duan HongXing
///! date: 5 Apr, 2025
///!
pub struct AkvpMessage {
    protocol: String,
    pub command: String,
    ts: u32,
    key: String,
    args: String,
    ttl: i32,

    body: Vec<u8>,
}

impl AkvpMessage {
    pub fn parse(message: &Vec<u8>) -> Self {
        println!("{:?}", message);

        // psudo code
        let protocol = "AKVP/1".to_string();
        let command = "GET".to_string();
        let ts = 1;
        let key = "k1".to_string();
        let args = "ex".to_string();
        let ttl = -1;

        let body: Vec<u8> = Vec::new();

        let akvp_msg = AkvpMessage {
            protocol,
            command,
            ts,
            key,
            args,
            ttl,
            body,
        };

        akvp_msg
    }
}
