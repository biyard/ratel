use dioxus_aws::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub data: Signal<Vec<String>>,
}

impl Controller {
    pub fn new() -> Result<Self, RenderError> {
        let mut ctrl = Self {
            data: use_signal(|| vec![]),
        };
        use_context_provider(|| ctrl);

        let _ = use_server_future(move || async move {
            // TODO: implement init state logic from server
            // reqwest::get(format!("/v2/collections/{collection_id}"))
            //     .await?
            //     .json::<Collection>()
            //     .await
        })?
        .value();

        Ok(ctrl)
    }

    pub fn add_item(&mut self, item: String) {
        let mut data = self.data.write();
        data.push(item);
    }

    pub fn remove_item(&mut self, index: usize) {
        let mut data = self.data.write();
        data.remove(index);
    }
}
