use crate::common::*;
use crate::features::profile::components::*;
use crate::features::profile::i18n::ProfileTranslate;
use crate::features::spaces::pages::actions::gamification::controllers::profile::get_my_global_profile;

/// The global player profile page, mounted at `/me/profile`.
///
/// Loads the authenticated user's global gamification profile and
/// composes the hero card, stats grid, creator earnings card, and
/// dungeon standings list.
#[component]
pub fn GlobalPlayerProfilePage() -> Element {
    let tr: ProfileTranslate = use_translate();
    let user_ctx = crate::features::auth::hooks::use_user_context();

    let profile = use_loader(move || async move { get_my_global_profile().await })?;
    let profile = profile();

    let user_name = user_ctx()
        .user
        .as_ref()
        .map(|u| u.display_name.clone())
        .unwrap_or_default();

    rsx! {
        SeoMeta { title: "{tr.profile_title}" }

        div {
            class: "flex flex-col gap-4 py-6 px-4 mx-auto w-full max-w-desktop max-mobile:px-3 max-mobile:py-4",
            "data-testid": "global-player-profile-page",

            h1 { class: "text-xl font-bold text-text-primary", "{tr.profile_title}" }

            HeroPlayerCard { profile: profile.clone(), user_name }
            StatsGrid { profile: profile.clone() }
            CreatorEarningsCard { earnings_xp: profile.creator_earnings_xp }
            DungeonStandingsList {}
        }
    }
}
