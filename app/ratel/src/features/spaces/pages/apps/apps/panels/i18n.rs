use crate::features::spaces::pages::apps::apps::panels::*;

translate! {
    PanelsTranslate;

    // Topbar
    breadcrumb_apps: { en: "Apps", ko: "앱" },
    breadcrumb_panels: { en: "Panels", ko: "패널" },
    type_badge: { en: "Panels", ko: "패널" },
    topbar_title: { en: "Panel Settings", ko: "패널 설정" },
    back_aria: { en: "Back", ko: "뒤로" },

    // Footer
    footer_saving: { en: "Saving...", ko: "저장 중..." },
    footer_saved: { en: "Changes saved", ko: "변경사항 저장됨" },

    // Total quotas
    total_quotas: { en: "Total quotas", ko: "총 쿼터" },
    total_quotas_hint: {
        en: "Setting the quota to 0 allows anyone to participate without a limit.",
        ko: "쿼터를 0으로 설정하면, 인원 제한 없이 누구나 참여하도록 설정할 수 있습니다.",
    },
    total_label: { en: "Total", ko: "총계" },
    allocated_label: { en: "allocated", ko: "할당됨" },
    unassigned_label: { en: "unassigned", ko: "미할당" },

    // Attribute groups
    attribute_groups: { en: "Attribute groups", ko: "속성 그룹" },
    attribute_groups_hint: {
        en: "Select which attributes to use for this space",
        ko: "이 스페이스에서 사용할 속성을 선택하세요",
    },
    attr_university: { en: "University", ko: "대학교" },
    attr_age: { en: "Age", ko: "나이" },
    attr_gender: { en: "Gender", ko: "성별" },
    attr_university_desc: {
        en: "Participation limited to users with a verified university affiliation.",
        ko: "대학 소속이 인증된 사용자만 참여 가능합니다.",
    },
    attr_age_desc: {
        en: "Split participation by age brackets (adult / minor / generation).",
        ko: "연령대(성인/미성년자/세대)별로 참여를 나눕니다.",
    },
    attr_gender_desc: {
        en: "Enforce a gender balance across the participant pool.",
        ko: "참가자 풀에서 성별 균형을 유지합니다.",
    },

    // Collective panel
    collective_title: { en: "Collective Panel Attributes", ko: "Collective 패널 속성" },
    collective_hint: {
        en: "Users must have these attributes to participate.",
        ko: "참여하기 위해서는 해당 속성을 가지고 있어야 합니다.",
    },
    collective_empty: {
        en: "No attributes selected. Toggle attributes above to add.",
        ko: "선택된 속성이 없습니다. 위에서 속성을 토글하세요.",
    },
    move_to_conditional_aria: { en: "Move to conditional", ko: "Conditional로 이동" },
    move_age_to_conditional: { en: "Move Age to Conditional", ko: "Age를 Conditional로 이동" },
    move_gender_to_conditional: { en: "Move Gender to Conditional", ko: "Gender를 Conditional로 이동" },

    // Conditional table
    conditional_title: { en: "Conditional Panel Attributes", ko: "Conditional 패널 속성" },
    conditional_hint: {
        en: "Participation is based on quotas, and participants are distributed by attribute.",
        ko: "인원 수 기반으로 참여가 이루어지며 속성별로 참여자를 분배합니다.",
    },
    conditional_over_allocated: {
        en: "Over-allocated:",
        ko: "초과 할당:",
    },
    conditional_over_allocated_detail: {
        en: " The sum of max quotas exceeds total quotas. Some attributes may not reach their max quota.",
        ko: " 최대 쿼터의 합이 총 쿼터를 초과합니다. 일부 속성의 최대 쿼터가 충족되지 않을 수 있습니다.",
    },
    conditional_empty: {
        en: "No conditional attributes configured. Move Collective attributes above into Conditional to set quotas.",
        ko: "설정된 conditional 속성이 없습니다. 위의 Collective 속성을 Conditional로 이동하여 쿼터를 설정하세요.",
    },
    th_attribute_group: { en: "Attribute group", ko: "속성 그룹" },
    th_attributes: { en: "Attributes", ko: "속성" },
    th_ratio: { en: "Ratio", ko: "비율" },
    th_max_quotas: { en: "Max quotas", ko: "최대 쿼터" },

    // Value labels
    val_male: { en: "Male", ko: "남성" },
    val_female: { en: "Female", ko: "여성" },
    val_adult: { en: "Adult", ko: "성인" },
    val_minor: { en: "Minor", ko: "미성년자" },
    val_verified: { en: "Verified", ko: "인증됨" },
    val_generation: { en: "Generation", ko: "세대" },

    // Viewer fallback
    viewer_no_access: {
        en: "You do not have access to view this panel.",
        ko: "이 패널을 조회할 수 없습니다.",
    },
    viewer_back: { en: "Back", ko: "뒤로" },
}
