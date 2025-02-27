pub struct TxFetcher {
    api_url: String,
}

impl TxFetcher {
    pub fn new(testnet: bool) -> Self {

        dotenv::dotenv().ok();
        let base_url: String = std::env::var("BASE_URL")
            .expect("Missing .env file or value");
        let tnt = if testnet == true {
            "/testnet"
        } else {
            ""
        };

        TxFetcher{api_url: format!("{}{}/api", base_url, tnt)}
    }
    pub async fn fetch_async(&self, txid: &str) -> Result<String, reqwest::Error> {

        let url = format!("{}/tx/{}/hex", self.api_url, txid);

        println!("{}", url);

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .send()
            .await
            .unwrap()
            .text()
            .await;
        response
    }
    pub fn fetch_sync(&self, txid: &str) -> Result<String, reqwest::Error> {

        let url = format!("{}/tx/{}/hex", self.api_url, txid);
        println!("{}", url);
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .send()?
            .text();
        response
    }
}
#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::tx::Tx;
    use super::*;
    #[tokio::test]
    async fn fetch_async_test() {
        // segwit testnet, to do c202201f6c18beb46710e5d3a46bd8775c57648cd9d7aef1be441d170ca8cdb5
        // main legacy 452c629d67e41baec3ac6f04fe744b4b9617f8f859c63b3002f8684e7a4fee03
        let tx_id = "ee51510d7bbabe28052038d1deb10c03ec74f06a79e21913c6fcf48d56217c87"; // main legacy
        let tf = TxFetcher::new(false);
        let result = tf.fetch_async(tx_id);

        match result.await {
            Ok(result) => {
                println!("{:#?}", result);
                let raw_tx = hex::decode(result).unwrap();
                // coming soon segwit
                // if raw_tx[4] == 0 {
                //    raw_tx.remove(4);
                //    raw_tx.remove(4);
                // }
                let mut stream = Cursor::new(raw_tx);
                let tx = Tx::parse(&mut stream, false).unwrap();
                assert_eq!(tx.id(), tx_id);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    #[test]
    fn fetch_sync_test() {
        let tx_id = "ee51510d7bbabe28052038d1deb10c03ec74f06a79e21913c6fcf48d56217c87"; // main legacy
        let tf = TxFetcher::new(false);
        let result = tf.fetch_sync(tx_id);

        match result {
            Ok(result) => {
                println!("{:#?}", result);
                let raw_tx = hex::decode(result).unwrap();
                // coming soon segwit
                // if raw_tx[4] == 0 {
                //    raw_tx.remove(4);
                //    raw_tx.remove(4);
                // }
                let mut stream = Cursor::new(raw_tx);
                let tx = Tx::parse(&mut stream, false).unwrap();
                assert_eq!(tx.id(), tx_id);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}