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
    sub_team_app_submitted_title: {
        en: "{team} submitted a sub-team application",
        ko: "{team}팀이 하위팀 신청을 제출했습니다",
    },
    sub_team_app_approved_title: {
        en: "{parent} approved your sub-team application",
        ko: "{parent}팀이 하위팀 신청을 승인했습니다",
    },
    sub_team_app_rejected_title: {
        en: "{parent} rejected your sub-team application",
        ko: "{parent}팀이 하위팀 신청을 반려했습니다",
    },
    sub_team_app_returned_title: {
        en: "{parent} returned your sub-team application for revision",
        ko: "{parent}팀이 수정 요청과 함께 신청을 반송했습니다",
    },
    sub_team_ann_received_title: {
        en: "{parent} published an announcement",
        ko: "{parent}팀이 공지를 게시했습니다",
    },
    sub_team_ann_comment_title: {
        en: "{name} commented on your announcement",
        ko: "{name}님이 공지에 댓글을 남겼습니다",
    },
    sub_team_deregistered_title: {
        en: "{parent} removed your team as a sub-team",
        ko: "{parent}팀이 하위팀 관계를 해제했습니다",
    },
    sub_team_left_parent_title: {
        en: "{team} left as a sub-team",
        ko: "{team}팀이 하위팀에서 탈퇴했습니다",
    },
    sub_team_parent_deleted_title: {
        en: "{parent} was deleted — your team is no longer a sub-team",
        ko: "{parent}팀이 삭제되어 더 이상 하위팀이 아닙니다",
    },
}
