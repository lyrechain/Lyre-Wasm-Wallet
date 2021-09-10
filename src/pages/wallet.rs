use mogwai::prelude::*;
use web_sys::HtmlInputElement;

use crate::{
    pages::{github_icon, mail_icon, telegram_icon, twitter_icon},
    routing::Route,
};

use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use log::{trace, Level};
use rand::rngs::OsRng;

pub fn keypair_ops() -> ViewBuilder<HtmlElement> {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let pub_key = keypair.public.as_bytes();
    let base58_encoded = bs58::encode(pub_key).into_string();

    builder! {
      <div class = "frow height-100 direction-column row-center" >
        <div class="frow width-100">
            <h2 class="color-tertiary">
                "Solana Public Key"
            </h2>
        </div>
        <div class="frow">
            <p class="frow color-tertiary">
                { base58_encoded }
            </p>
        </div>
        <div class="frow">
        <p class="color-secondary">
            "Nothing much to see here other than a base58 encoded Solana address has been created via a secure CSPRNG and Ed25519 Key. You can inspect the code at https://github.com/lyrechain/Lyre-Wasm-Wallet."
        </p>
        <p class="color-secondary">
        " The wallet is under development. Help build our ecosystem by participating in our seed raise by visiting http://lyrechain.ch for more details like tokenomics and the whitepaper. If you have read the tokenomics remember to reach out at mail@lyrechain.ch if you want to support our seed raise."
        </p>
    </div>
      </div>
    }
}
