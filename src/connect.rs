use reqwest;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub struct Request {
    file: String,
    host: String,
    port: String,
    path: String
}

impl Request {
    pub fn new(file: String, host: String, port: String, path: String) -> Request {
        Request {
            file,
            host,
            port,
            path
        }
    }

    pub async fn connect_to_server(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!(
            "http://{host}:{port}{path}",
            host = self.host,
            port = self.port,
            path = self.path
        );
        let client = reqwest::Client::new();
        let res = client.post(&url)
            .json(&self.file)
            .send();

        res.await
    }

    pub fn response(&self) {
        // code goes here
    }
}
