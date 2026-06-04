use dioxus_translate::*;

translate! {
    LaunchpadPartnerTranslate;

    // Entry button on the rewards page
    convert_cta: { en: "Request point conversion", ko: "포인트 전환 요청하기" },
    modal_title: { en: "Convert points to token", ko: "포인트 토큰 전환" },

    // Step 1 — intro
    intro_title: { en: "Convert your points into token", ko: "포인트를 토큰으로 전환하세요" },
    intro_body: {
        en: "Points are converted at the round's fixed token ratio. Issued token can be redeemed for stablecoin from the treasury floor at any time.",
        ko: "포인트는 라운드의 정해진 비율로 토큰으로 발행되며, 언제든 트레저리 바닥가로 스테이블코인으로 환전할 수 있습니다.",
    },
    intro_continue: { en: "Continue", ko: "계속하기" },
    go_convert: { en: "Convert on Launchpad", ko: "Launchpad에서 전환하기" },
    go_convert_hint: {
        en: "You'll connect a wallet and enter the amount on Launchpad to finish.",
        ko: "Launchpad에서 지갑 연결과 금액 입력 후 전환이 완료됩니다.",
    },

    // Step 2 — wallet
    wallet_title: { en: "Connect your wallet", ko: "지갑 연동" },
    wallet_body: {
        en: "Connect the wallet that will receive the token, or paste its address.",
        ko: "토큰을 받을 지갑을 연결하거나, 지갑 주소를 입력하세요.",
    },
    wallet_metamask: { en: "Continue with MetaMask", ko: "MetaMask로 계속하기" },
    wallet_kaia: { en: "Continue with Kaia Wallet", ko: "Kaia Wallet로 계속하기" },
    wallet_connecting: { en: "Connecting…", ko: "연결 중…" },
    wallet_connected_label: { en: "Receiving wallet", ko: "수령 지갑" },
    wallet_change: { en: "Change", ko: "변경" },

    // Step 3 — amount
    amount_title: { en: "Register conversion", ko: "전환 등록" },
    amount_body: {
        en: "Enter the points to deduct from your service balance for this round.",
        ko: "기업 서비스에서 차감할 포인트를 입력하고 이번 라운드에 등록합니다.",
    },
    amount_balance_label: { en: "Available points", ko: "보유 포인트" },
    amount_placeholder: { en: "Points to convert", ko: "전환할 포인트" },
    amount_max: { en: "Max", ko: "전액" },
    submit: { en: "Submit request", ko: "전환 요청" },

    // Navigation
    back: { en: "Back", ko: "이전" },
    next: { en: "Next", ko: "다음" },

    // Results
    success_toast: { en: "Point conversion request submitted.", ko: "포인트 전환 요청이 완료되었습니다." },
    err_invalid_amount: { en: "Enter a valid amount within your balance.", ko: "보유 포인트 내에서 올바른 금액을 입력하세요." },
    err_need_wallet: { en: "Connect or enter a wallet first.", ko: "지갑을 먼저 연결하거나 입력하세요." },
    err_wallet_not_found: { en: "Wallet not found. Install or enable the extension.", ko: "지갑을 찾을 수 없습니다. 확장 프로그램을 설치/활성화하세요." },
}
