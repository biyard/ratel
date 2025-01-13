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