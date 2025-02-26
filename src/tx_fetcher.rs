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
    pub async fn fetch(&self, txid: &str) -> Result<String, reqwest::Error> {

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
}
#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::tx::Tx;
    use super::*;
    #[tokio::test]

    async fn test_fetch() {

        let txId = "...";
        let tf = TxFetcher::new(false);
        let result = tf.fetch(txId);

        match result.await {
            Ok(result) => {
                println!("{:#?}", result);
                let mut raw_tx = hex::decode(result).unwrap();
                if raw_tx[4] == 0 {
                    raw_tx.remove(4);
                    raw_tx.remove(4);
                }
                let mut stream = Cursor::new(raw_tx);
                let tx = Tx::parse(&mut stream, false).unwrap();

                // tx.locktime = little_endian_to_int(raw[-4:])
                // assert_eq!(tx.id(), txId);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}