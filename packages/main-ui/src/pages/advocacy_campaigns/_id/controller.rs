use bdk::prelude::*;
use dto::AdvocacyCampaign;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    pub id: ReadOnlySignal<i64>,
    pub campaign: Resource<AdvocacyCampaign>,
}

impl Controller {
    pub fn new(lang: Language, id: ReadOnlySignal<i64>) -> std::result::Result<Self, RenderError> {
        let mut campaign = use_server_future(move || {
            let id = id();

            async move {
                AdvocacyCampaign::get_client(crate::config::get().main_api_endpoint)
                    .get(id)
                    .await
                    .unwrap_or_default()
            }
        })?;

        use_effect(move || {
            spawn(async move {
                campaign.restart();
            });
        });

        let ctrl = Self { lang, id, campaign };

        Ok(ctrl)
    }

    pub async fn handle_agree(&mut self) {
        let id = self.id();
        if self.campaign().unwrap_or_default().voted {
            tracing::debug!("User has already agreed to the campaign with id: {}", id);
            return;
        }
        match AdvocacyCampaign::get_client(crate::config::get().main_api_endpoint)
            .agree(id)
            .await
        {
            Ok(_) => {
                // Navigate to the advocacy campaigns page after agreeing
                tracing::debug!("Successfully agreed to the campaign with id: {}", id);
                self.campaign.restart();
            }
            Err(e) => {
                btracing::e!(e);
            }
        }
    }
}
