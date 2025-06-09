use crate::pages::threads::_id::spaces::_id::controller::Controller;

use super::*;
use bdk::prelude::*;
use dto::SpaceForm;

#[component]
pub fn SpacePage(
    #[props(default = Language::En)] lang: Language,
    feed_id: i64,
    id: i64,
) -> Element {
    let tr: SpaceTranslate = translate(&lang);
    let ctrl = Controller::new(lang, feed_id, id)?;
    let space = ctrl.space()?;

    let space_form = space.space_form;

    tracing::debug!("space {:?}", space);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        if space_form == SpaceForm::Legislation && space.id != 0 {
            LegislationPage { lang, feed_id, id }
        } else if space_form == SpaceForm::Poll && space.id != 0 {
            PollPage { lang, feed_id, id }
        } else if space_form == SpaceForm::Deliberation && space.id != 0 {
            DeliberationPage { lang, feed_id, id }
        } else if space_form == SpaceForm::Nft && space.id != 0 {
            NftPage { lang, feed_id, id }
        }
    }
}
