use by_macros::DioxusController;

type Signal<T> = fn() -> T;

#[derive(DioxusController)]
pub struct Controller {
    pub d1: Signal<String>,
    pub d2: Signal<Vec<String>>,
}

#[test]
fn test_dioxus_controller() {
    let ctrl = Controller {
        d1: || "Hello".to_string(),
        d2: || vec![],
    };

    assert_eq!(ctrl.d1(), "Hello");
    assert_eq!(ctrl.d2().len(), 0);
}
