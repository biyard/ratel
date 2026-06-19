//! Fan a freshly-created inbox notification out to the recipient's registered
//! push tokens (Android FCM today; iOS later). Best-effort: any failure is
//! logged, never propagated — the in-app inbox row is already written, so the
//! notification is never lost even if push delivery fails.
#![cfg(feature = "server")]

use crate::common::models::notification::{UserDeviceToken, UserInboxNotification};
use crate::common::utils::fcm::{self, PushMessage, PushOutcome};
use crate::common::*;
use crate::features::notifications::i18n::NotificationsTranslate;
use dioxus_translate::Language;

/// Max device tokens we push to per notification (a user rarely has more).
const MAX_DEVICES: i32 = 50;

pub async fn fan_out_push(n: UserInboxNotification) {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let opts = UserDeviceToken::opt_with_bookmark(None)
        .sk("USER_DEVICE_TOKEN".to_string())
        .limit(MAX_DEVICES);
    let tokens = match UserDeviceToken::query(cli, n.pk.clone(), opts).await {
        Ok((rows, _)) => rows,
        Err(e) => {
            crate::error!("push fanout: device-token query failed: {e}");
            return;
        }
    };
    if tokens.is_empty() {
        return;
    }

    // Recipient locale isn't stored yet → default to Korean (primary locale).
    let tr = NotificationsTranslate::new(&Language::Ko);
    let (title, body, _avatar) = n.payload.get_contents(&tr, &Language::Ko);
    let msg = PushMessage {
        title,
        body,
        url: n.payload.url().to_string(),
    };

    for t in tokens {
        if fcm::send_to_token(&t.token, &msg).await == PushOutcome::Stale {
            // Prune dead tokens so they don't accumulate / waste future sends.
            let _ = UserDeviceToken::delete(cli, &t.pk, Some(t.sk)).await;
        }
    }
}
