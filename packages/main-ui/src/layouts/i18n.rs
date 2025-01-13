use dioxus_translate::translate;

translate! {
    HeaderTranslate;

    signup: {
        ko: "회원가입",
        en: "Sign up",
    },

    login: {
        ko: "로그인",
        en: "Login",
    },
}

translate! {
    CongratulationPopupTranslate;

    welcome: {
        ko: "환영합니다!",
        en: "Welcome!",
    },

    congratulation: {
        ko: "‘서비스명’에 오신 것을 환영합니다!\n익명성과 신뢰를 바탕으로 안전한 투표 환경을 제공합니다.",
        en: "Welcome to 'Service Name'!\nWe provide a safe voting environment based on anonymity and trust.",
    },

    start_poll: {
        ko: "투표 시작하기",
        en: "Start voting",
    },
}

translate! {
    UserSetupPopupTranslate;

    welcome: {
        ko: "환영합니다!",
        en: "Welcome!",
    },

    enter_nickname: {
        ko: "닉네임을 입력해주세요",
        en: "Please enter your nickname",
    },

    special_characters: {
        ko: "특수문자는 입력할 수 없습니다.",
        en: "Special characters are not allowed.",
    },

    agree_email: {
        ko: "[필수]이메일 및 계정주소 수집에 동의합니다.",
        en: "[Required]I agree to collect email and account address.",
    },

    next: {
        ko: "다음",
        en: "Next",
    },
}

translate! {
    SignupPopupTranslate;

    continue_with_google: {
        ko: "Google로 계속하기",
        en: "Continue with Google",
    },

    quick_sign_in: {
        ko: "간편 로그인",
        en: "Quick Sign-in",
    },
}

translate! {
    MenusTranslate;

    home: {
        ko: "HOME",
        en: "HOME",
    },
    topics: {
        ko: "TOPICS",
        en: "TOPICS",
    },
    politicians: {
        ko: "POLITICIAN STATUS",
        en: "POLITICIAN STATUS",
    },
    patrons: {
        ko: "PATRONS",
        en: "PATRONS",
    },
}
