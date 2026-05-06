use dioxus_translate::*;

translate! {
    TeamMembershipTranslate;

    section_label_prefix: { en: "Team ·", ko: "팀 ·" },
    section_label_strong: { en: "Membership", ko: "멤버십" },

    hero_title_en: { en: "Team Membership", ko: "Team Membership" },
    hero_title_ko: { en: "팀 멤버십", ko: "팀 멤버십" },
    hero_desc_prefix: {
        en: "Your current plan, ",
        ko: "현재 사용 중인 플랜과 ",
    },
    hero_desc_credits: { en: "Credits", ko: "크레딧" },
    hero_desc_suffix: {
        en: ", and purchase history at a glance.",
        ko: ", 구매 내역을 한눈에 확인하세요.",
    },

    // Current plan card
    current_plan_label: { en: "Current Plan", ko: "현재 플랜" },
    credits_label: { en: "Credits", ko: "크레딧" },
    credits_remaining_hint: { en: "Remaining", ko: "남은 크레딧" },
    expires_label: { en: "Expires", ko: "만료" },
    expires_unlimited: { en: "Unlimited", ko: "무제한" },
    expires_auto_renew: {
        en: "Auto-renews on the expiration date.",
        ko: "만료일에 자동 결제됩니다.",
    },

    // Scheduled downgrade banner
    downgrade_prefix: {
        en: "On next billing cycle, the team will be downgraded to ",
        ko: "다음 결제 주기에 ",
    },
    downgrade_suffix: { en: ".", ko: " 플랜으로 다운그레이드됩니다." },

    // Tier descriptions
    tier_free_desc: {
        en: "Basic membership open to everyone",
        ko: "누구나 참여 가능한 기본 멤버십",
    },
    tier_pro_desc: {
        en: "Reward spaces for small communities",
        ko: "소규모 커뮤니티를 위한 보상 스페이스",
    },
    tier_max_desc: {
        en: "Reward spaces for large communities",
        ko: "대규모 커뮤니티를 위한 보상 스페이스",
    },
    tier_vip_desc: {
        en: "Reward spaces for influencers and marketing partners",
        ko: "인플루언서 및 마케팅 파트너를 위한 보상 스페이스",
    },
    tier_enterprise_desc: {
        en: "Custom partner plan for enterprises and organizations",
        ko: "기업 및 기관 맞춤형 파트너 멤버십",
    },
    enterprise_label: { en: "Enterprise", ko: "엔터프라이즈" },

    // Purchase history card
    history_title: { en: "Purchase History", ko: "구매 내역" },
    history_count_suffix_one: { en: "record", ko: "건" },
    history_count_suffix_many: { en: "records", ko: "건" },
    th_type: { en: "Type", ko: "유형" },
    th_amount: { en: "Amount", ko: "금액" },
    th_payment_id: { en: "Payment ID", ko: "결제 ID" },
    th_date: { en: "Date", ko: "날짜" },

    history_empty_title: { en: "No Purchases Yet", ko: "구매 내역 없음" },
    history_empty_desc: {
        en: "Purchase history will appear here after your first transaction.",
        ko: "첫 결제 이후 구매 내역이 여기에 표시됩니다.",
    },

    // Viewer (non-admin) placeholder
    no_permission: {
        en: "You don't have permission to view this page.",
        ko: "이 페이지를 볼 수 있는 권한이 없습니다.",
    },
}
