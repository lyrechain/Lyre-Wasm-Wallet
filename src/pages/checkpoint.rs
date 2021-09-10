use mogwai::prelude::*;
use web_sys::HtmlInputElement;

use crate::{
    pages::{github_icon, mail_icon, telegram_icon, twitter_icon},
    routing::Route,
};

pub fn load_checkpoint() -> ViewBuilder<HtmlElement> {
    let (tx, rx) = txrx();

    builder! {
      <div class = "frow" >
      <div class="frow width-100 row-center">
            <div class="frow width-50">
                <div class="frow">
                    <img src="assets/LyreChain-Logo-Full.svg" alt="" srcset="" />
                </div>
            </div>
        </div>
        <div class="frow direction-column row-center width-100">
            <div class="frow width-80">
                <img src="assets/Lyre-Tokens-horizontal.png" alt="" srcset="" />
            </div>
        </div>
        <div class="frow mb-20 p-10 text-left row-center">
            <h1 class="compagnon-roman color-glow color-tertiary">"The Community De-Fi Bank"</h1>
        </div>
        <div class="frow">
            <div class="frow width-100 mb-20 text-center">
                <a href=String::from(Route::Create) class="shell-button-large fontin-regular" on:click=tx.contra_map(|_:
                    &Event| {
                      //pubkey()
                    })>"Create Wallet"</a>
            </div>

            <div class="frow width-100 mt-10 row-between mb-20">
                <a href=String::from(Route::WalletCheckpoint)
                    class="shell-button module module-border-wrap frow fontin-regular mr-5">"Seed Phrase"</a>
                <a href=String::from(Route::Recover) class="frow shell-button fontin-regular ml-5">"Peer Recovery"</a>
            </div>
        </div>
      </div>
    }
}

use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use log::{trace, Level};
use rand::rngs::OsRng;

pub(crate) fn pubkey() -> Keypair {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    trace!("{:?}", &keypair);

    keypair
}

pub(crate) fn build_key() -> ViewBuilder<HtmlElement> {
    builder! {
      <div class="frow">
        { format!("{:?}", pubkey()) }
    </div>
    }
}
