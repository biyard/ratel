use crate::common::*;

translate! {
    GamificationTranslate;

    level_label: { en: "LV", ko: "Lv" },
    xp_suffix: { en: "XP", ko: "XP" },
    streak_suffix: { en: "day streak", ko: "일 연속" },
    combo_label: { en: "Combo", ko: "콤보" },
    xp_progress_aria: {
        en: "Experience points progress",
        ko: "경험치 진행도",
    },
    you_label: { en: "YOU", ko: "나" },
    dungeon_label: { en: "Dungeon", ko: "던전" },
    floor_label: { en: "Floor", ko: "층" },
    explorers_label: { en: "Explorers", ko: "탐험가" },
    chapters_label: { en: "Chapters", ko: "챕터" },

    // Quest Map labels
    quest_map_loading: { en: "Loading quest map…", ko: "퀘스트 맵 로딩 중…" },
    quest_begin: { en: "BEGIN", ko: "시작" },
    quest_locked: { en: "Locked", ko: "잠김" },
    quest_cleared: { en: "Cleared", ko: "클리어" },
    quest_role_gated: { en: "Role Required", ko: "역할 필요" },
    quest_projected_xp: { en: "Projected XP", ko: "예상 XP" },
    quest_earned_xp: { en: "Earned XP", ko: "획득 XP" },

    chapter_active_badge: { en: "ACTIVE", ko: "진행중" },
    chapter_passed_badge: { en: "PASSED", ko: "완료" },
    chapter_locked_badge: { en: "LOCKED", ko: "잠김" },
    chapter_unlock_hint: { en: "Complete the previous chapter to unlock", ko: "이전 챕터를 완료하면 잠금 해제됩니다" },
    chapter_progress: { en: "Progress", ko: "진행도" },

    past_chapters_summary: { en: "chapters cleared", ko: "챕터 완료" },
    expand_past_chapters: { en: "Show completed chapters", ko: "완료된 챕터 보기" },
    collapse_past_chapters: { en: "Hide completed chapters", ko: "완료된 챕터 숨기기" },

    // Quest Briefing overlay
    briefing_xp_at_stake: { en: "XP at stake", ko: "획득 가능 XP" },
    briefing_begin: { en: "BEGIN", ko: "시작" },
    briefing_cancel: { en: "Cancel", ko: "취소" },
    briefing_time_remaining: { en: "Time remaining", ko: "남은 시간" },
    briefing_retries: { en: "Retries", ko: "재시도" },
    briefing_prerequisites: { en: "Prerequisites", ko: "선행 조건" },
    briefing_met: { en: "Met", ko: "충족" },
    briefing_not_met: { en: "Not met", ko: "미충족" },
    briefing_unlocks_next: { en: "Unlocks next", ko: "다음 잠금 해제" },
    briefing_base: { en: "base", ko: "기본" },
    briefing_explorers: { en: "explorers", ko: "탐험가" },
    briefing_combo: { en: "combo", ko: "콤보" },
    briefing_streak: { en: "streak", ko: "연속" },

    // Completion Overlay
    completion_xp_earned: { en: "XP earned!", ko: "XP 획득!" },
    completion_level_up: { en: "LEVEL UP!", ko: "레벨 업!" },
    completion_quest_unlocked: { en: "New quest unlocked", ko: "새 퀘스트 잠금 해제" },
    completion_chapter_complete: { en: "Chapter Complete!", ko: "챕터 완료!" },
    completion_role_upgraded: { en: "Role Upgraded!", ko: "역할 승급!" },
    completion_tap_dismiss: { en: "Tap to continue", ko: "탭하여 계속" },

    // Chapter Editor
    ch_pill: { en: "CH", ko: "CH" },
    add_chapter: { en: "Add chapter", ko: "챕터 추가" },
    add_chapter_hint: { en: "Create a new chapter for your quest map", ko: "퀘스트 맵에 새 챕터를 만드세요" },
    chapter_name: { en: "Chapter name", ko: "챕터 이름" },
    actor_role_label: { en: "Actor role", ko: "참여 역할" },
    benefit_label: { en: "Completion benefit", ko: "완료 보상" },
    benefit_xp_only: { en: "XP only", ko: "XP만" },
    benefit_role_upgrade: { en: "Role upgrade", ko: "역할 승급" },
    benefit_role_and_xp: { en: "Role upgrade + XP", ko: "역할 승급 + XP" },
    depends_on_label: { en: "Depends on", ko: "선행 조건" },
    click_to_connect: { en: "Click a quest to set as dependency", ko: "선행 퀘스트를 클릭하세요" },
    delete_chapter: { en: "Delete", ko: "삭제" },
    collapse: { en: "Collapse", ko: "접기" },
    preview_participant: { en: "Preview as Participant", ko: "참여자로 미리보기" },
}
