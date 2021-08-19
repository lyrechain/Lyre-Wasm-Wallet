use log::{trace, Level};
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Splash,
    WalletCheckpoint,
    Recover,
    Create,
}

/// We'll use TryFrom::try_from to convert the window's url hash into a Route.
impl TryFrom<&str> for Route {
    type Error = String;

    fn try_from(s: &str) -> Result<Route, String> {
        trace!("route try_from: {}", s);
        // remove the scheme, if it has one
        let hash_split = s.split("#").collect::<Vec<_>>();
        let after_hash = match hash_split.as_slice() {
            [_, after] => Ok(after),
            _ => Err(format!("route must have a hash: {}", s)),
        }?;

        let paths: Vec<&str> = after_hash.split("/").collect::<Vec<_>>();
        trace!("route paths: {:?}", paths);

        match paths.as_slice() {
            [""] => Ok(Route::WalletCheckpoint),
            ["", ""] => Ok(Route::WalletCheckpoint),
            ["", "splash"] => Ok(Route::Splash),
            ["", "checkpoint"] => Ok(Route::WalletCheckpoint),
            ["", "create"] => Ok(Route::Create),
            ["", "recover"] => Ok(Route::Recover),
            r => Err(format!("unsupported route: {:?}", r)),
        }
    }
}

/// Convert the route into its hashed string.
/// This should match the inverse conversion in TryFrom above.
impl From<Route> for String {
    fn from(route: Route) -> String {
        match route {
            Route::Splash => "#/splash".into(),
            Route::WalletCheckpoint => "#/".into(),
            Route::Create => "#/create".into(),
            Route::Recover => "#/recover".into(),
        }
    }
}

impl From<&Route> for ViewBuilder<HtmlElement> {
    fn from(route: &Route) -> Self {
        match route {
            Route::Splash => builder! {
                <main>
                    <h1>"Welcome to the splash"</h1>
                </main>
            },
            Route::WalletCheckpoint => crate::pages::load_checkpoint(),
            Route::Create => builder! {
                <main>
                    <h1>"Create Wallet"</h1>
                </main>
            },
            Route::Recover => builder! {
                <main>
                    <h1>"Recover wallet"</h1>
                </main>
            },
        }
    }
}

impl From<&Route> for View<HtmlElement> {
    fn from(route: &Route) -> Self {
        ViewBuilder::from(route).into()
    }
}

/// Here we'll define some helpers for displaying information about the current route.
impl Route {
    pub fn nav_splash(&self) -> String {
        match self {
            Route::Splash => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }

    pub fn nav_checkpoint(&self) -> String {
        match self {
            Route::WalletCheckpoint => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }

    pub fn nav_create(&self) -> String {
        match self {
            Route::Create => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }

    pub fn nav_recover(&self) -> String {
        match self {
            Route::Recover => "nav-link active",
            _ => "nav-link",
        }
        .to_string()
    }
}

pub struct App {
    pub route: Route,
}

#[derive(Clone)]
pub enum AppModel {
    HashChange(String),
}

#[derive(Clone)]
pub enum AppView {
    PatchPage(Patch<View<HtmlElement>>),
    Error(String),
}

impl AppView {
    pub fn error(&self) -> Option<String> {
        match self {
            AppView::Error(msg) => Some(msg.clone()),
            _ => None,
        }
    }

    /// If the message is a new route, convert it into a patch to replace the current main page.
    pub fn patch_page(&self) -> Option<Patch<View<HtmlElement>>> {
        match self {
            AppView::PatchPage(patch) => Some(patch.clone()),
            _ => None,
        }
    }
}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

    fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::HashChange(hash) => {
                // When we get a hash change, attempt to convert it into one of our routes
                match Route::try_from(hash.as_str()) {
                    // If we can't, let's send an error message to the view
                    Err(msg) => tx.send(&AppView::Error(msg)),
                    // If we _can_, create a new view from the route and send a patch message to
                    // the view
                    Ok(route) => {
                        if route != self.route {
                            let view = View::from(ViewBuilder::from(&route));
                            self.route = route;
                            tx.send(&AppView::PatchPage(Patch::Replace {
                                index: 0,
                                value: view,
                            }));
                        }
                    }
                }
            }
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        builder! {
            <slot
                window:hashchange=tx.contra_filter_map(|ev:&Event| {
                    let hev = ev.dyn_ref::<HashChangeEvent>().unwrap().clone();
                    let hash = hev.new_url();
                    Some(AppModel::HashChange(hash))
                })
                patch:children=rx.branch_filter_map(AppView::patch_page)>
                /*<nav>
                    <ul>
                        <li class=self.route.nav_splash()>
                            <a href=String::from(Route::Splash)>"Splash"</a>
                        </li>
                        <li class=self.route.nav_checkpoint()>
                            <a href=String::from(Route::WalletCheckpoint)>"Checkpoint"</a>
                        </li>
                        <li class=self.route.nav_create()>
                            <a href=String::from(Route::Create)>"Create"</a>
                        </li>
                        <li class=self.route.nav_recover()>
                            <a href=String::from(Route::Recover)>"Recover"</a>
                        </li>
                    </ul>
                </nav>
                <pre>{rx.branch_filter_map(AppView::error)}</pre>*/
                {ViewBuilder::from(&self.route)}
            </slot>
        }
    }
}

#[cfg(test)]
mod test_route_try_from {
    use super::*;

    #[test]
    fn can_convert_string_to_route() {
        let s = "https://localhost:8080/#/";
        assert_eq!(Route::try_from(s), Ok(Route::WalletCheckpoint));
    }
}
