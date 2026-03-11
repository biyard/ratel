use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SpaceActionSettingTab {
    pub label: &'static str,
    pub target: Route,
}

impl SpaceActionSettingTab {
    pub fn new(label: &'static str, target: Route) -> Self {
        Self { label, target }
    }
}

#[derive(Clone, Copy, DioxusController)]
pub struct Context {
    title: Signal<String>,
    tabs: Signal<Vec<SpaceActionSettingTab>>,
}

pub fn use_space_actions_context() -> Context {
    use_context()
}

impl Context {
    pub fn init() -> Result<Self, Loading> {
        let srv = Self {
            title: use_signal(|| String::default()),
            tabs: use_signal(|| Vec::new()),
        };

        use_context_provider(move || srv);
        Ok(srv)
    }

    pub fn mutate_title_and_tabs(
        &mut self,
        title: &'static str,
        tabs: Vec<SpaceActionSettingTab>,
    ) -> &mut Self {
        let mut ctx = self.clone();

        use_future(move || {
            let tabs = tabs.clone();
            async move {
                ctx.set_title(title).set_tabs(tabs.clone());
            }
        });

        self
    }

    pub fn set_title(&mut self, title: &'static str) -> &mut Self {
        self.title.set(title.to_string());

        self
    }

    pub fn set_tabs(&mut self, tabs: Vec<SpaceActionSettingTab>) -> &mut Self {
        self.tabs.set(tabs);

        self
    }

    pub fn is_empty(&self) -> bool {
        self.title().is_empty()
    }
}
