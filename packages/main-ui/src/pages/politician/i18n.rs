use dioxus_translate::translate;

translate! {
    PoliticianStanceTranslate;

    title: {
        ko: "국회의원",
        en: "Politician",
    },

    supportive: {
        ko: "찬성",
        en: "Supportive",
    },

    against: {
        ko: "반대",
        en: "Against",
    },

    neutral: {
        ko: "중립",
        en: "Neutral",
    }

    no_stance: {
        ko: "의견 없음",
        en: "No stance",
    },

    more: {
        ko: "더보기",
        en: "MORE",
    }

    name: {
        ko: "이름",
        en: "NAME",
    }

    party: {
        ko: "정당",
        en: "PARTY",
    }

    district: {
        ko: "지역구",
        en: "DISTRICT",
    }

    stance_on_crypto: {
        ko: "암호화폐에 대한 입장",
        en: "STANCE ON CRTPTO",
    }

    proclaim: {
        ko: "PROCLAIM",
        en: "PROCLAIM",
    }

    tooltip: {
        ko: "Proclaim 은 해당 의원의 암호화폐에 대한 정책적 긍정 및 부정에 대한 의사를 표현하는 것입니다. 해당 의원실 소속원만 변경할 수 있습니다.",
        en: "Proclaim is a way to express the policy of a politician on cryptocurrency. Only the staff of the politician can change it.",
    }

    change_stance: {
        ko: "입장 변경",
        en: "CHANGE STANCE",
    }

    search_title: {
        ko: "검색하기",
        en: "SEARCH",
    }

    name_placeholder: {
        ko: "이름을 입력해주세요.",
        en: "Enter the name.",
    }
    
    stance_placeholder: {
        ko: "암호화폐에 대한 입장",
        en: "Stance",
    }

    party_placeholder: {
        ko: "정당을 선택해주세요.",
        en: "Select the party.",
    }

    city_placeholder: {
        ko: "도시",
        en: "City",
    }

    district_placeholder: {
        ko: "지역구",
        en: "District",
    }
    
    search: {
        ko: "검색",
        en: "SEARCH",
    }

    clear: {
        ko: "초기화",
        en: "CLEAR",
    }

    email: {
        ko: "이메일",
        en: "EMAIL",
    }

    email_placeholder: {
        ko: "이메일을 입력해주세요.",
        en: "Enter the email.",
    }
    
    agree_email_verification: {
        ko: "해당 의원의 소속원임을 확인하고 이메일 인증에 동의합니다.",
        en: "I confirm that I am a member of the relevant representative's office and agree to email verification.",
    }

    verify_email: {
        ko: "이메일 인증",
        en: "VERIFY EMAIL",
    }

    confirm_email: {
        ko: "이메일 확인",
        en: "CONFIRM EMAIL",
    }

    explanation_confirm_email1: {
        ko: "‘{email}’로 이메일을 확인하거나 스팸함을 확인하고 확인 링크를 클릭하여 인증을 완료해주세요. 이메일을 받지 못하거나 링크를 클릭할 수 없는 경우, ",
        en: "Check inbox or spam of ‘{email}’ and please click an attached link to complete verification. If you couldn’t receive an email or click the link, please click ",
    }

    here: {
        ko: "여기",
        en: "here",
    }

    explanation_confirm_email2: {
        ko: "를 클릭하여 문의해주세요.",
        en: " to contact us.",
    }

    confirm_verification: {
        ko: "인증 확인",
        en: "CONFIRM VERIFICATION",
    }
}
