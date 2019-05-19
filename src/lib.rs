extern crate hyper;
extern crate hyper_native_tls;
extern crate yup_oauth2;

use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::convert::TryInto;
use yup_oauth2::{service_account_key_from_file, GetToken, ServiceAccountAccess, Token};

struct Client {
    token: Token,
}

impl Client {
    pub fn new(path: String) -> Self {
        let client_secret = service_account_key_from_file(&path.to_string()).unwrap();
        let hyper_client =
            hyper::Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));

        let mut access = ServiceAccountAccess::new(client_secret, hyper_client);
        let token = access.token(&vec!["https://www.googleapis.com/auth/pubsub"]).unwrap();

        Self {
            token,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use mockito::mock;

    const TEST_KEY_PATH: &'static str = "examples/test_key.json";

    #[test]
    fn client_setup() {
        let _m = mock("GET", "/token").with_status(200).create();

    }
}
