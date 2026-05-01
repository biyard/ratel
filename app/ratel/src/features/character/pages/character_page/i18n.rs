use crate::*;

translate! {
    CharacterPageTranslate;

    page_title: { en: "Character", ko: "캐릭터" },

    // Hero — Level / XP / SP
    level_label: { en: "Level", ko: "레벨" },
    xp_title: { en: "Character XP", ko: "캐릭터 XP" },
    xp_to_next: { en: "XP to Level", ko: "다음 레벨까지" },
    xp_total_earned: { en: "Total XP earned", ko: "누적 XP" },
    sp_label: { en: "Skill Points", ko: "스킬 포인트" },
    sp_hint_ready: { en: "points ready to spend", ko: "사용 가능 포인트" },
    sp_hint_one_ready: { en: "1 point ready to spend", ko: "1 포인트 사용 가능" },
    sp_hint_empty: { en: "Earn XP to grant more", ko: "XP를 모아 포인트를 획득하세요" },
    sp_hint_default: { en: "Spend on the tree below", ko: "아래 스킬 트리에서 사용하세요" },

    // Section header
    skill_tree_title: { en: "Skill Tree", ko: "스킬 트리" },
    skill_tree_hint: {
        en: "+5% per level · Max +50% at L10",
        ko: "레벨당 +5% · L10에서 최대 +50%",
    },

    // Skill cards — names and copy
    money_tree_name: { en: "Money Tree", ko: "머니트리" },
    money_tree_sub: {
        en: "RatelPoint earning boost",
        ko: "RatelPoint 획득 부스트",
    },
    money_tree_desc: {
        en: "Boosts every RatelPoint payout you receive from any space's reward, applied multiplicatively before the amount is credited to your balance.",
        ko: "모든 스페이스 보상에서 받는 RatelPoint 지급액에 곱셈 부스트를 적용하여, 잔액에 적립되기 전에 가산됩니다.",
    },

    ranker_name: { en: "Ranker", ko: "랭커" },
    ranker_sub: {
        en: "SpaceXP & Character XP boost",
        ko: "스페이스XP 및 캐릭터 XP 부스트",
    },
    ranker_desc: {
        en: "Boosts the bonus portion of every SpaceActivity you record. Compounds: more XP per action → faster character leveling → more SP for future skills.",
        ko: "모든 스페이스 활동에서 적립되는 보너스 XP를 부스트합니다. 복리 효과: 활동당 더 많은 XP → 더 빠른 레벨업 → 더 많은 SP.",
    },

    influencer_name: { en: "Influencer", ko: "인플루언서" },
    influencer_sub: {
        en: "Lower Hot threshold for your spaces",
        ko: "내 스페이스의 핫 노출 기준 완화",
    },
    influencer_desc: {
        en: "Lowers the participants-required-for-Hot threshold for spaces you own — at L6 your space surfaces with just 4 participants instead of the global 10.",
        ko: "내가 소유한 스페이스가 핫에 노출되는 데 필요한 참여자 수를 완화합니다. L6에서는 기본 10명 대신 4명만 있으면 노출됩니다.",
    },

    sweeper_name: { en: "Sweeper", ko: "싹쓸이" },
    sweeper_sub: {
        en: "Higher owner bonus on your spaces",
        ko: "내 스페이스 소유자 보너스 증가",
    },
    sweeper_desc: {
        en: "When a participant claims a reward in a space you own, the owner-bonus you receive goes up by +5% per level. At L6 you take 40% of every payout instead of the default 10%.",
        ko: "내가 소유한 스페이스에서 참여자가 보상을 청구할 때, 소유자 보너스가 레벨당 +5%씩 증가합니다. L6에서는 기본 10% 대신 40%를 받습니다.",
    },

    // Buttons / states
    levelup_label: { en: "Level Up", ko: "레벨 업" },
    maxed_label: { en: "Maxed", ko: "최대치" },
    locked_label: { en: "Locked", ko: "잠김" },
    coming_soon: { en: "v2 · Coming soon", ko: "v2 · 곧 출시" },
    not_released: { en: "Not yet released", ko: "아직 출시되지 않음" },
    next_boost: { en: "Next: +", ko: "다음: +" },
    next_was: { en: "% (was +", ko: "% (현재 +" },
    max_reached: { en: "Maximum boost reached", ko: "최대 부스트 도달" },

    // Footer labels
    level_meta_label: { en: "Level", ko: "레벨" },
    status_meta_label: { en: "Status", ko: "상태" },
    sp_unit: { en: "SP", ko: "SP" },
}
