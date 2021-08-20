use mogwai::prelude::*;

use crate::{
    pages::{github_icon, mail_icon, telegram_icon, twitter_icon},
    routing::Route,
};

pub fn load_checkpoint() -> ViewBuilder<HtmlElement> {
    builder! {
    <section id="shell-root" class="frow height-100vh background-cover  p-20">
      <div class="l-shell frow  row-center width-100 fixed-max-width-368">
        <div class="frow img-normal col-md-4-8 width-100 image-40">
          <img src="assets/Lyre-Chain-Horizontal-Tokens.png" alt="" srcset="" />
        </div>
        <div class="frow mt-20 mb-20 text-left row-start">
          <h1 class="compagnon-roman white">"The Community Micro-Savings"</h1>
          <h1 class="compagnon-roman white">"De-Fi Bank"</h1>
        </div>
        <div class="frow width-100 text-center">
          <a href=String::from(Route::Create) class="shell-button-large fontin-regular">"Register with Email"</a>
        </div>

        <div class="frow width-100 mt-10 row-between mb-20">
          <a href=String::from(Route::WalletCheckpoint) class="shell-button module module-border-wrap frow fontin-regular mr-5">"Seed Phrase"</a>
          <a href=String::from(Route::Recover) class="frow shell-button fontin-regular ml-5">"Peer Recovery"</a>
        </div>

      </div>
    </section>
    }
}
