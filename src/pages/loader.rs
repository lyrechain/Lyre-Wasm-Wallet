use mogwai::prelude::*;

pub(crate) fn launch_loader(catch_phrase: &str) -> ViewBuilder<HtmlElement> {
    builder! {
        <div class="frow">
            <div class="frow width-50 hue">
                <img src="assets/LyreChain-Logo-Full.svg" alt="" srcset=""/>
            </div>
            <div id="fill-text" class="frow mt-20 width-100">
                <h3 class="color-tertiary color-glow compagnon-roman hue">{catch_phrase}</h3>
            </div>
        </div>
    }
}
