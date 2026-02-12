use super::*;
use common::components::{TiptapEditor, Typo, Variant, Weight};

#[component]
pub fn ViewerPage(space_id: SpacePartition) -> Element {
    let mut content = use_signal(|| {
        r#"<section><h1>연구 개요</h1><ol><li><strong>연구 배경 및 필요성</strong><ul><li>공론형 여론조사와 블록체인 적용 가능성에 대한 요약입니다.</li><li>블록체인 기술을 통해 공정하고 투명한 여론조사를 실현하고자 합니다.</li></ul></li><li><strong>의제의 특성</strong><ul><li>다양한 참여자들의 의견을 반영하는 공론형 여론조사입니다.</li><li>블록체인 기술을 활용하여 데이터의 무결성과 투명성을 보장합니다.</li></ul></li></ol><table><tr><th>작성자</th><th>댓글 내용</th></tr><tr><td>참여자2</td><td>댓글 예시 내용</td></tr><tr><td>참여자3</td><td>두번째 댓글 예시</td></tr><tr><td>참여자1</td><td>세번째 댓글 예시</td></tr><tr><td>참여자2</td><td>네번째 댓글 예시</td></tr><tr><td>참여자4</td><td>다섯번째 댓글 예시</td></tr></table></section><section><h1>조사 설계 및 데이터 수집</h1><ol><li><strong>조사 설계</strong><ul><li>공론형 여론조사와 블록체인 적용 가능성에 대한 요약.</li><li>조사는 '블록체인 기반 공론형 여론조사'로 명명되었습니다.</li><li>현재 상태는 'InProgress'입니다.</li></ul></li><li><strong>공론형 여론조사 진행 절차</strong><ul><li>여론조사는 비동의 간음죄 도입에 대한 주제로 진행되었습니다.</li><li>찬성, 반대, 유보의 세 가지 옵션이 제공되었습니다.</li></ul></li><li><strong>블록체인 기술 적용 방안</strong><ul><li>블록체인 기술을 활용하여 투표의 투명성과 신뢰성을 높였습니다.</li><li>투표 결과는 블록체인에 기록되어 조작 불가능합니다.</li></ul></li><li><strong>분석 방법 및 데이터 수집 범위</strong><ul><li>응답자들의 답변은 블록체인을 통해 기록되었습니다.</li><li>찬성 1표, 반대 1표, 유보 1표가 기록되었습니다.</li><table border=\"1\"><tr><th>응답자 ID</th><th>답변</th><th>생성 시간</th></tr><tr><td>USER#001</td><td>찬성</td><td>1700000025</td></tr><tr><td>USER#002</td><td>반대</td><td>1700000026</td></tr><tr><td>USER#003</td><td>유보</td><td>1700000027</td></tr></table></ul></li></ol></section><section><h1>분석 결과</h1><ol><li><strong>설문 응답 분포 및 사전-사후 변화</strong><ul><li>찬성 1건, 반대 1건, 유보 1건의 응답이 있었습니다.</li><li>사전-사후 변화 데이터는 없습니다.</li></ul></li><li><strong>게시물/댓글 주요 논점</strong><ul><li>토론 게시물 1건과 댓글 5건이 존재합니다.</li><li>주요 논점은 제공된 데이터에서 확인할 수 없습니다.</li></ul></li><li><strong>TF-IDF / LDA / Text Network 요약</strong><ul><li>TF-IDF 분석에서 '동의'는 2.34, '무고'는 1.27의 점수를 받았습니다.</li><li>LDA 분석에서 '동의'는 토픽1, '무고'는 토픽2에 속합니다.</li><li>Text Network 분석에서 '동의'와 '무고' 사이에 가중치 3의 연결이 있습니다.</li><table><tr><th>노드</th><th>중심성</th><th>차수 중심성</th></tr><tr><td>동의</td><td>0.24</td><td>0.82</td></tr><tr><td>무고</td><td>0.18</td><td>0.6</td></tr></table></ul></li><li><strong>통합 분석</strong><ul><li>통합 분석 결과는 제공된 데이터에서 확인할 수 없습니다.</li></ul></li></ol></section><section><h1>결론 및 제언</h1><ol><li><strong>연구 결과 요약</strong><ul><li>블록체인 기반 공론형 여론조사의 가능성에 대한 토론이 활발하게 진행되었다.</li><li>주요 토론 주제는 동의와 무고에 관한 내용이었다.</li><li>찬성, 반대, 유보 등 다양한 의견이 제시되었다.</li></ul></li><li><strong>의견 변화의 패턴과 원인</strong><ul><li>참여자들 사이에서 의견의 변화는 뚜렷하게 관찰되지 않았다.</li><li>대부분의 참여자들은 자신의 입장을 유지한 채 토론에 참여했다.</li><li>의견 변화의 주요 원인에 대한 데이터는 없음.</li></ul></li><li><strong>방법론 및 정책적 제언</strong><ul><li>블록체인 기반 공론형 여론조사의 방법론적 타당성이 입증되었다.</li><li>추후 연구에서는 더 다양한 주제와 참여자를 포함시켜야 한다.</li><li>정책적으로는 블록체인 기술을 활용한 투명하고 공정한 여론조사 시스템 구축이 제언된다.</li></ul></li></ol><table><thead><tr><th>키워드</th><th>TF-IDF</th></tr></thead><tbody><tr><td>동의</td><td>2.34</td></tr><tr><td>무고</td><td>1.27</td></tr></tbody></table></section>"#.to_string()
    });

    let mut editable = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-4 flex-1 h-full",
            button { onclick: move |_| editable.toggle(), "변경" }
            div { "상태 : {editable}" }
            Typo { variant: Variant::H1, weight: Weight::Extrabold, "TIPTAP EDITOR" }
            TiptapEditor {
                class: "w-full h-full",
                content: content(),
                editable: editable(),
                placeholder: "Type here...",
                on_content_change: move |html: String| {
                    content.set(html);
                },
            }
        }
    }
}
