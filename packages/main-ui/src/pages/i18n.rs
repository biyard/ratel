use dioxus_translate::*;

translate! {
    PagesTranslate;

    _text: {
        ko: "홈페이지",
        en: "homepage",
    },
}

translate! {
    FinishedTopicTranslate;

    finished_topic: {
        ko: "종료된 투표",
        en: "Finished Voting",
    },

    closed_voting: {
        ko: "투표 마감",
        en: "Voting Closed",
    },

    cumulative_donations: {
        ko: "누적 기부금",
        en: "Cumulative Donations",
    },
}

translate! {
    DescriptionWrapperTranslate;

    inner_dangerous: {
        ko: "이 국민투표는 찬반 선택과 함께 <b>기부금</b>으로 열정과 의지를 표현하는 방식입니다. </br>  \
        <b>기부금은 실제 기부가 아니며</b>, 투표 참여의 상징적 의미를 더하기 위해 사용됩니다.  \
        <div class=\"mt-[10px]\"> \
            <span style=\"color:red\">*</span> \
            기부금 금액은 선택 사항이며, 투표 결과에 영향을 미치지 않습니다.\
        </div>",
        en: "This national vote is a way to express passion and determination with <b>donations</b> along with the choice of for or against. </br>  \
        <b>The donation is not an actual donation</b> and is used to add symbolic meaning to voting participation.  \
        <div class=\"mt-[10px]\"> \
            <span style=\"color:red\">*</span> \
            The donation amount is optional and does not affect the voting results.\
        </div>",
    },
}

translate! {
    ContentWrapperTranslate;

    cumulative_donations: {
        ko: "누적 기부금",
        en: "Cumulative Donations",
    },
}

translate! {
    UpcomingTopicsTranslate;

    soon_voting: {
        ko: "다가올 투표",
        en: "Upcoming Voting",
    },
}

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

    logout: {
        ko: "로그아웃",
        en: "Logout",
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
