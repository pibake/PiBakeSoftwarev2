use reqwest;

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

    pub fn connect_to_server(&self) -> bool {
        let url = format!(
            "http://{host}:{port}{path}",
            host = self.host,
            port = self.port,
            path = self.path
        );

        let json = self.file;
        let client = reqwest::Client::new();
        let response = client.post(url)
            .json(json)
            .send();

        if response.status().is_success() {
            true
        } else {
            false
        }
    }
}
