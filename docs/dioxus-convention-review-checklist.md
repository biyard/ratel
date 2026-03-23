# Dioxus Convention Review Checklist

이 체크리스트는 `app/` 하위 파일 변경 시 PR 리뷰에 적용합니다.
전체 규칙은 `/docs/dioxus-convention.md`를 참고하세요.

---

## 1. 모듈 구조

- [ ] `views/`는 `{view_name}/mod.rs` 구조를 따르는가? 뷰 전용 컴포넌트는 해당 디렉토리 내에 있는가?
- [ ] `components/`에는 2개 이상의 뷰에서 공유되는 컴포넌트만 있는가?
- [ ] 서버 전용 모듈은 `#[cfg(feature = "server")]`로 게이트되어 있는가?
- [ ] 웹 전용 모듈은 `#[cfg(not(feature = "server"))]` 또는 `#[cfg(feature = "web")]`로 게이트되어 있는가?

## 2. Import

- [ ] 모든 소스 파일이 `use crate::*;`로 시작하는가?
- [ ] 컨트롤러 파일은 추가로 `use crate::models::*;`를 사용하는가?
- [ ] `lib.rs`에 `type Result<T> = common::Result<T>;`가 정의되어 있는가?
- [ ] 개별 sibling 모듈 import를 흩뿌리지 않고 `use crate::*;`를 사용하는가?

## 3. 컴포넌트

- [ ] `#[component]` 어트리뷰트를 사용하고 `Element`를 반환하는가?
- [ ] 선택적 props는 `Option<T>`, 기본값 props는 `#[props(default)]`를 사용하는가?
- [ ] 리스트 렌더링 시 `key`를 제공하는가?
- [ ] 인라인 스타일 대신 Tailwind CSS 클래스를 사용하는가?
- [ ] 네이밍 규칙을 따르는가? (PascalCase, Modal/Service 접미사 등)
- [ ] `Switch` 컴포넌트 사용 시 `label` prop을 통해 적절한 `aria-label`(accessible name)이 설정되는가? (`role="switch"`와 `aria-checked`는 label 유무와 상관없이 항상 렌더링됨)

## 4. 라우트

- [ ] Route enum에 `#[rustfmt::skip]`이 적용되어 있는가?
- [ ] `PageNotFound` catch-all 라우트가 포함되어 있는가?
- [ ] 타입화된 경로 파라미터 (`SpacePartition`, `TeamPartition`)를 사용하는가? (raw `String` 금지)
- [ ] 중첩 라우터는 `ChildRouter`를 사용하는가? (`Router` 직접 중첩 금지)

## 5. 레이아웃

- [ ] 레이아웃 컴포넌트에서 `Outlet::<Route> {}`를 정확히 한 번 사용하는가?
- [ ] Provider(컨텍스트 주입)와 Layout(UI 크롬)이 별도의 `#[layout(...)]`으로 분리되어 있는가?

## 6. 컨트롤러 (서버 함수)

- [ ] `#[get]`, `#[post]`, `#[patch]`, `#[put]`, `#[delete]` 매크로를 사용하는가?
- [ ] `Result<T>`를 반환하는가? (`Json<T>` 금지)
- [ ] DynamoDB 접근은 `crate::config::get().dynamodb()`를 사용하는가?
- [ ] 핸들러 함수명에 `_handler` 접미사가 있는가?
- [ ] DTO에 `#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]`가 적용되어 있는가?

### 타입 주입 (FromRequestParts) — 중요

- [ ] **컨트롤러 본문에서 세션/유저/스페이스를 수동으로 추출하지 않는가?** 매크로 어트리뷰트의 타입 주입 (`user: User`, `role: SpaceUserRole` 등)을 사용해야 한다.
- [ ] 반복되는 추출 로직이 있다면 `FromRequestParts`를 구현한 별도 타입으로 분리했는가?
- [ ] `FromRequestParts` 구현 시 `parts.extensions`에 캐싱하는가? (중복 DB 조회 방지)
- [ ] `FromRequestParts` 구현이 `#[cfg(feature = "server")]`로 게이트되어 있는가?

### 서버-클라이언트 아키텍처 패턴

- [ ] 여러 모델 필드에 의존하는 계산된 불리언(예: "참여 가능 여부")이 서버 컨트롤러에서 한 번만 계산되어 응답 DTO 필드로 노출되는가? (클라이언트에서 동일한 조건을 중복 검사하지 않는가?)
- [ ] 동일한 불리언 조건이 여러 서버 위치에서 사용될 때 모델 구조체의 헬퍼 메서드(예: `SpaceCommon::is_participation_open()`)로 추출되어 있는가?

## 7. 모델 (DynamoEntity)

- [ ] `Default, Debug, Clone, Serialize, Deserialize, DynamoEntity, PartialEq`를 derive하는가?
- [ ] `pk: Partition`, `sk: EntityType` 필드를 사용하는가?
- [ ] 타임스탬프는 `i64` (밀리초)를 사용하는가?
- [ ] UUID 생성에 `uuid::Uuid::now_v7()`을 사용하는가?
- [ ] 모든 `impl` 블록이 `#[cfg(feature = "server")]`로 게이트되어 있는가?

## 8. 훅

- [ ] 모든 훅 함수명이 `use_` 접두사로 시작하는가?
- [ ] 데이터 로딩 훅은 `Result<Loader<T>, Loading>`을 반환하는가?
- [ ] 뮤테이션 후 `invalidate_query(&key)`를 호출하는가?

## 9. Provider / Context

- [ ] 서비스는 `init()` 메서드 → `use_context_provider`를 사용하는가?
- [ ] Provider 컴포넌트는 에셋(스크립트, 스타일) 로딩만 하는가? (UI 없음)
- [ ] reactive 필드(`Signal`, `Loader`, `Memo`, `ReadOnlySignal`, `Resource`)를 가진 context provider 구조체에 `DioxusController` derive가 있는가?
- [ ] mutation API(`open`, `close`, `push` 등)가 있는 서비스 구조체(`PopupService`, `ToastService` 등)에는 `DioxusController`를 적용하지 않았는가?
- [ ] `DioxusController`가 적용된 구조체에 `(self.field)()` 패턴의 수동 getter가 중복되지 않았는가?

## 10. Feature Flag

- [ ] 모든 JS interop 호출이 `#[cfg(not(feature = "server"))]`로 가드되어 있는가?
- [ ] 모든 DynamoDB 연산이 `#[cfg(feature = "server")]` 블록 안에 있는가?
- [ ] 조건부 derive는 `#[cfg_attr(...)]`를 사용하는가?

## 11. 번역 (i18n)

- [ ] `en`과 `ko` 번역을 모두 제공하는가?
- [ ] 구조체명이 `Translate` 접미사를 사용하는가? (예: `LoginModalTranslate`)
- [ ] `use_translate()`로 소비하는가? (수동 생성 금지)

## 12. 에러 처리 및 Toast

- [ ] 서버 전용 에러 variant에 `#[cfg(feature = "server")]`와 `#[serde(skip)]`이 있는가?
- [ ] 새 에러 variant에 `#[translate(...)]`가 있는가?
- [ ] `toast.error()`에 타입화된 `common::Error` variant를 전달하는가? (raw 문자열 금지: `toast.error("...")` 금지)
- [ ] `toast.error()` 호출 전에 수동으로 `.translate()`를 호출하지 않는가? (자동 번역됨)
- [ ] 새 사용자 대면 에러 메시지가 필요하면 `common::Error`에 variant를 추가했는가?

## 13. JS Interop

- [ ] `js_namespace` 배열이 JS 전역 경로와 정확히 일치하는가?
- [ ] JS 호출이 `#[cfg(not(feature = "server"))]`로 가드되어 있는가?

## 14. 쿼리 / 데이터 페칭

- [ ] 쿼리 키가 계층적 `Vec<String>`을 사용하는가?
- [ ] 뮤테이션 후 `invalidate_query`를 호출하는가?
- [ ] 단일 리소스는 `use_query`, 페이지네이션 목록은 `use_infinite_query`를 사용하는가?
