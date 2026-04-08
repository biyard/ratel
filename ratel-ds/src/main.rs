mod components;
mod pages;
mod router;

fn main() {
    dioxus::launch(router::App);
}
