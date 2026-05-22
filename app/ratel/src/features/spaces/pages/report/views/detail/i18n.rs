use crate::*;

translate! {
    ReportDetailTranslate;

    back_aria: { en: "Back", ko: "뒤로" },
    breadcrumb_root: { en: "Reports", ko: "보고서" },
    breadcrumb_separator: { en: "/", ko: "/" },
    autosave_just_now: { en: "Auto-saved · just now", ko: "자동 저장됨 · 방금" },
    autosave_unsaved: { en: "Unsaved changes", ko: "저장 안됨" },
    autosave_saving: { en: "Saving…", ko: "저장 중…" },
    share_btn: { en: "Share", ko: "공유" },
    export_btn: { en: "Export", ko: "내보내기" },
    pdf_download_btn: { en: "PDF Download", ko: "PDF 다운로드" },
    publish_btn: { en: "Publish", ko: "발행" },
    publish_modal_title: { en: "Publish report", ko: "보고서 게시" },
    publish_modal_body: {
        en: "Publishing makes the report's PDF visible to all space members. They will be able to download it from the space settings sidebar.",
        ko: "게시하는 순간 보고서 PDF가 모든 스페이스 멤버에게 공개되며, 사이드바에서 다운로드할 수 있습니다.",
    },
    publish_modal_confirm: { en: "Publish", ko: "게시" },
    publish_modal_cancel: { en: "Cancel", ko: "취소" },
    publishing_label: { en: "Publishing…", ko: "게시 중…" },

    banner_eyebrow: { en: "Document Editor", ko: "문서 편집기" },
    banner_title: { en: "Build your action report", ko: "액션 보고서를 작성하세요" },
    banner_text: {
        en: "Draft narrative, then use Insert Data to pull aggregates from any analyze you've built.",
        ko: "본문을 작성한 뒤 Insert Data로 분석에서 만든 집계를 불러와 차트를 삽입하세요.",
    },

    title_placeholder: { en: "Untitled report", ko: "제목 없는 보고서" },
    subtitle_placeholder: { en: "Subtitle (optional)", ko: "부제 (선택)" },
    body_placeholder: {
        en: "Tell the story — toolbar handles formatting, Insert Data pulls in aggregates.",
        ko: "본문을 작성하세요 — 툴바로 서식을 적용하고 Insert Data로 데이터를 삽입할 수 있어요.",
    },
    insert_data: { en: "Insert data", ko: "데이터 삽입" },

    fmt_block_paragraph: { en: "Paragraph", ko: "본문" },
    fmt_block_h1: { en: "Heading 1", ko: "제목 1" },
    fmt_block_h2: { en: "Heading 2", ko: "제목 2" },
    fmt_block_h3: { en: "Heading 3", ko: "제목 3" },
    fmt_image: { en: "Insert image", ko: "이미지 삽입" },
    fmt_youtube: { en: "Embed YouTube", ko: "YouTube 삽입" },
    fmt_table: { en: "Insert table", ko: "표 삽입" },
    fmt_prompt_image: { en: "Image URL", ko: "이미지 URL" },
    fmt_prompt_youtube: { en: "YouTube URL", ko: "YouTube URL" },
    fmt_prompt_table: { en: "Table size (rows x cols)", ko: "표 크기 (행 x 열)" },
    fmt_prompt_link: { en: "URL", ko: "URL" },
    fmt_youtube_invalid: {
        en: "Could not parse the YouTube URL.",
        ko: "YouTube URL을 인식할 수 없습니다.",
    },
    fmt_table_range: {
        en: "Rows 1-20, columns 1-10.",
        ko: "행은 1~20, 열은 1~10 범위로 입력해 주세요.",
    },

    outline_heading: { en: "Outline", ko: "목차" },
    outline_empty: {
        en: "No headings yet — add an H1/H2/H3 and it'll appear here.",
        ko: "아직 헤딩이 없어요 — H1/H2/H3을 추가하면 여기에 나타납니다.",
    },
    meta_heading: { en: "Meta", ko: "메타" },
    meta_author: { en: "Author", ko: "작성자" },
    meta_created: { en: "Created", ko: "만든 날짜" },
    meta_edited: { en: "Edited", ko: "수정" },

    picker_eyebrow: { en: "/data — pick an aggregate", ko: "/data — 집계 선택" },
    picker_title: { en: "Pick an analyze item to chart it", ko: "차트로 만들 분석 항목을 골라주세요" },
    picker_close_aria: { en: "Close picker", ko: "피커 닫기" },
    picker_analyze_label: { en: "Analyze", ko: "분석" },
    picker_respondents_unit: { en: "respondents", ko: "응답자" },
    picker_items_unit: { en: "items", ko: "항목" },
    picker_tab_poll: { en: "Poll", ko: "Poll" },
    picker_tab_quiz: { en: "Quiz", ko: "Quiz" },
    picker_tab_discussion: { en: "Discussion", ko: "Discussion" },
    picker_tab_follow: { en: "Follow", ko: "Follow" },
    picker_empty: {
        en: "No items in this analyze for the selected source.",
        ko: "이 분석에는 선택한 소스의 항목이 없습니다.",
    },
}
