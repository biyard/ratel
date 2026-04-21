use dioxus_translate::*;

translate! {
    NotificationsTranslate;
    panel_title: { en: "Notifications", ko: "알림" },
    mark_all_read: { en: "Mark all as read", ko: "모두 읽음" },
    empty: { en: "No notifications yet", ko: "새 알림이 없습니다" },
    reply_title: { en: "{name} replied to your comment", ko: "{name}님이 답글을 남겼습니다" },
    mention_title: { en: "{name} mentioned you", ko: "{name}님이 나를 언급했습니다" },
    space_status_title: { en: "{space} is now {status}", ko: "{space}가 {status}로 변경되었습니다" },
    space_invite_title: { en: "{name} invited you to {space}", ko: "{name}님이 {space}에 초대했습니다" },
    relative_now: { en: "just now", ko: "방금" },
    relative_minute: { en: "{n}m ago", ko: "{n}분 전" },
    relative_hour: { en: "{n}h ago", ko: "{n}시간 전" },
    relative_day: { en: "{n}d ago", ko: "{n}일 전" },
}
