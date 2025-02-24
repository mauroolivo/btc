pub struct TxFetcher {
    api_url: String,
}
impl TxFetcher {
    pub fn new(testnet: bool) -> Self {
        if testnet {
            TxFetcher{api_url: String::from("...")}
        } else {
            TxFetcher{api_url: String::from("...")
            }
        }
    }
    pub fn fetch(testnet: bool) -> String {


        "...".to_string()
    }
}
