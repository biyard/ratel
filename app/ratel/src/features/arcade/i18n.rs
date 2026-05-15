use dioxus_translate::*;

translate! {
    ArcadeLayoutTranslate;

    brand: { en: "RATEL ARCADE", ko: "RATEL ARCADE" },
    brand_sub: { en: "Ratel Arcade · v1", ko: "라텔 오락실 · v1" },
    tab_home: { en: "Arcade", ko: "오락실" },
    tab_leaderboard: { en: "Leaderboard", ko: "리더보드" },
    chip_unit: { en: "chips", ko: "칩" },
    chip_aria: { en: "Open chip exchange", ko: "칩 환전 열기" },
    admin_create_round: { en: "Create round", ko: "라운드 생성" },
}

translate! {
    ArcadeExchangeModalTranslate;

    title: { en: "Exchange RP → chips", ko: "RP → 칩 환전" },
    subtitle: {
        en: "Convert your RatelPoints into arcade chips. Chips can be staked across arcade games.",
        ko: "RatelPoint를 아케이드 칩으로 환전합니다. 칩은 오락실의 모든 게임에 사용됩니다.",
    },
    input_label: { en: "RP to convert", ko: "환전할 RP" },
    ratio_label: { en: "Current rate", ko: "현재 환율" },
    ratio_value: { en: "1 RP → {$chips} chips", ko: "1 RP → {$chips} 칩" },
    receive_label: { en: "You'll receive", ko: "받을 칩" },
    receive_value: { en: "{$chips} chips", ko: "{$chips} 칩" },
    confirm_btn: { en: "Confirm exchange", ko: "환전 확정" },
    cancel_btn: { en: "Cancel", ko: "취소" },
    redeem_disabled_note: {
        en: "Chip → RP redemption isn't available yet (v2).",
        ko: "칩 → RP 역환전은 v2에 활성화됩니다.",
    },
    error_amount: { en: "Enter an amount above zero.", ko: "0보다 큰 값을 입력하세요." },
    success_summary: {
        en: "Exchanged. New balance: {$balance} chips.",
        ko: "환전 완료. 잔액: {$balance} 칩.",
    },
}
