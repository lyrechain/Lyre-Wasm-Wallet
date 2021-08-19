use mogwai::prelude::*;

use crate::{
    pages::{github_icon, mail_icon, telegram_icon, twitter_icon},
    routing::Route,
};

pub fn load_checkpoint() -> ViewBuilder<HtmlElement> {
    builder! {
          <section id="hero-root" class="frow height-100vh background-cover  p-20">
      <div class="l-shell frow p-20 row-center width-100 fixed-max-width-368">
        <div class="frow img-normal col-md-4-8 width-100">
          <img src="assets/Lyre-Chain-Horizontal-Tokens.png" alt="" srcset=""/>

        </div>
        <div class="frow width-100 mt-20">

          <a href=String::from(Route::Create) class="module module-border-wrap frow fontin-regular mr-5">"NEW
            WALLET"</a>
          <a href=String::from(Route::Recover) class="frow fontin-regular ml-5">"LOAD WALLET"</a>
        </div>
        <div id="hero-social" class="frow width-100 mt-20 row-start">
          <div class="frow">
            <div class="frow col-xs-1-4">
              <a href="https://twitter.com/lyrechain">{ twitter_icon() }</a>
            </div>
            <div class="frow col-xs-1-4">
              <a href="https://telegram.com/lyrechain">{ telegram_icon() }</a>
            </div>
            <div class="frow col-xs-1-4">
              <a href="https://github.com/lyrechain">{ github_icon() }</a>
            </div>
            <div class="frow col-xs-1-4">
              <a href="mailto:mail@lyrechain.ch">{ mail_icon() }</a>
            </div>
          </div>
        </div>
      </div>
    </section>
      }
}
