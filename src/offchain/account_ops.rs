use json::object;
use log::{trace, Level};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize)]
struct SolApi {
    jsonrpc: String,
    id: u8,
    method: String,
    params: Vec<String>,
}

pub async fn airdrop() {
    use serde_json::json;
    /*let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": [
            "A75Yb8t45Gio55vVAbSn67PvVYNbnkMtVzmUZFFf4Buj",
            {
                "commitment": "finalized"
            }
        ]
    });*/
    let uri = "https://api.devnet.solana.com";

        let data = SolApi {
            jsonrpc: "2.0".into(),
            id: 2,
            method: "getBalance".into(),
            params: vec!["A75Yb8t45Gio55vVAbSn67PvVYNbnkMtVzmUZFFf4Buj".into()],
        };

        let mut res = surf::post(uri)
            .body(surf::Body::from_json(&data).unwrap())
            .await
            .unwrap();
        trace!("{:#?}", res.body_string().await.unwrap());
}
