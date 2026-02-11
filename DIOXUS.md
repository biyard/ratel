# DIOXUS.md

Dioxus 공식 예제 기반 정리 문서. Fullstack/Web 중심으로 작성.

---

## 목차

1. [기본 앱 구조](#1-기본-앱-구조)
2. [RSX 문법](#2-rsx-문법)
3. [UI 구성](#3-ui-구성)
4. [상태 관리](#4-상태-관리)
5. [비동기 처리](#5-비동기-처리)
6. [라우팅](#6-라우팅)
7. [풀스택](#7-풀스택)
8. [에셋 & 스타일링](#8-에셋--스타일링)
9. [API & 유틸리티](#9-api--유틸리티)
10. [레퍼런스](#10-레퍼런스)
11. [앱 데모 예제](#11-앱-데모-예제)

---

## 1. 기본 앱 구조

### 최소 앱 (`hello_world.rs`)

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            h1 { "Hello, Dioxus!" }
        }
    }
}
```

- `dioxus::launch(app)` 으로 앱 시작
- 컴포넌트는 `Element`를 반환하는 함수
- `rsx!` 매크로로 UI 선언

---

## 2. RSX 문법

### 종합 문법 가이드 (`rsx_usage.rs`)

```rust
fn app() -> Element {
    let name = "Dioxus";

    rsx! {
        // 기본 엘리먼트
        h1 { "Hello {name}" }

        // 속성 지정
        div {
            class: "container",
            style: "color: red;",
            "styled text"
        }

        // 조건부 렌더링
        if show {
            p { "보입니다" }
        }

        // 반복 렌더링
        for item in items.iter() {
            li { key: "{item.id}", "{item.name}" }
        }

        // 자식 컴포넌트
        MyComponent { title: "props 전달" }

        // 위험한 HTML 삽입 (XSS 주의)
        div { dangerous_inner_html: "<b>raw html</b>" }
    }
}
```

### 축약 문법 (`shorthand.rs`)

```rust
// 변수명이 props/속성명과 동일하면 축약 가능
let class = "my-class";
let onclick = move |_| { /* ... */ };

rsx! {
    div { class, onclick, "축약 가능" }
}
```

### Spread 연산자 (`spread.rs`)

```rust
#[component]
fn Button(#[props(extends = button)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        button { ..attributes, "클릭" }
    }
}

// 사용 시
rsx! {
    Button { class: "btn", onclick: handler }
}
```

---

## 3. UI 구성

### 비활성화 상태 (`disabled.rs`)

```rust
fn app() -> Element {
    let mut disabled = use_signal(|| false);

    rsx! {
        button {
            disabled: disabled(),
            onclick: move |_| disabled.toggle(),
            "토글"
        }
    }
}
```

### 이벤트 버블링 (`nested_listeners.rs`)

```rust
rsx! {
    div {
        onclick: move |_| log("외부 클릭"),
        button {
            onclick: move |evt| {
                evt.stop_propagation(); // 버블링 중단
                log("내부 클릭");
            },
            "클릭"
        }
    }
}
```

### SVG 렌더링 (`svg.rs`)

```rust
rsx! {
    svg {
        view_box: "0 0 100 100",
        circle {
            cx: "50", cy: "50", r: "40",
            fill: "red"
        }
    }
}
```

### 리스트 렌더링 (`simple_list.rs`)

```rust
// 이터레이터 사용
rsx! {
    ul {
        {items.iter().map(|item| rsx! {
            li { key: "{item.id}", "{item.name}" }
        })}
    }
}

// for 루프 사용
rsx! {
    ul {
        for item in items.iter() {
            li { key: "{item.id}", "{item.name}" }
        }
    }
}

// 조건부 렌더링
rsx! {
    if let Some(item) = optional_item {
        p { "{item}" }
    }
}
```

---

## 4. 상태 관리

### Signal 기본 (`signals.rs`, `repo_readme.rs`)

```rust
fn Counter() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        button { onclick: move |_| count += 1, "+1" }
        button { onclick: move |_| count -= 1, "-1" }
        p { "카운트: {count}" }
    }
}
```

- `Signal`은 `Copy` 타입이라 클로저에서 클론 불필요
- `+=`, `-=` 등 연산자 직접 사용 가능

### Memo (파생 상태) (`signals.rs`)

```rust
fn app() -> Element {
    let mut count = use_signal(|| 0);
    let doubled = use_memo(move || count() * 2);
    let is_even = use_memo(move || count() % 2 == 0);

    rsx! {
        p { "원본: {count}, 두 배: {doubled}, 짝수: {is_even}" }
    }
}
```

- `use_memo`는 의존성이 변경될 때만 재계산
- 자동 의존성 추적 (명시적 deps 불필요)

### Effect (부작용) (`signals.rs`)

```rust
fn app() -> Element {
    let count = use_signal(|| 0);

    use_effect(move || {
        // count가 변경될 때마다 실행
        println!("count 변경됨: {}", count());
    });

    rsx! { /* ... */ }
}
```

### Context API (`context_api.rs`)

```rust
// 부모 컴포넌트에서 제공
fn Parent() -> Element {
    use_context_provider(|| Signal::new(DarkMode(false)));

    rsx! { Child {} }
}

// 자식 컴포넌트에서 소비
fn Child() -> Element {
    let dark_mode = use_context::<Signal<DarkMode>>();

    rsx! {
        p { "다크모드: {dark_mode().0}" }
    }
}
```

- props drilling 없이 상태 공유
- 커스텀 Hook으로 감싸서 사용 가능

### Global Signal (`global.rs`)

```rust
// 앱 전체에서 접근 가능한 전역 상태
static COUNT: GlobalSignal<i32> = Signal::global(|| 0);
static DOUBLED: GlobalMemo<i32> = Signal::global_memo(|| COUNT() * 2);

fn app() -> Element {
    rsx! {
        button { onclick: move |_| *COUNT.write() += 1, "증가" }
        p { "값: {COUNT}, 두 배: {DOUBLED}" }
    }
}
```

### Reducer 패턴 (`reducer.rs`)

```rust
enum Action {
    Increment,
    Decrement,
    Reset,
}

fn reducer(state: &mut i32, action: Action) {
    match action {
        Action::Increment => *state += 1,
        Action::Decrement => *state -= 1,
        Action::Reset => *state = 0,
    }
}

fn app() -> Element {
    let mut state = use_signal(|| 0);

    let dispatch = move |action: Action| {
        state.with_mut(|s| reducer(s, action));
    };

    rsx! {
        button { onclick: move |_| dispatch(Action::Increment), "+" }
        button { onclick: move |_| dispatch(Action::Decrement), "-" }
        button { onclick: move |_| dispatch(Action::Reset), "리셋" }
        p { "{state}" }
    }
}
```

### Store 패턴 (`todomvc_store.rs`)

```rust
#[derive(Store)]
struct AppState {
    todos: Vec<Todo>,
    filter: Filter,
}
```

- `#[derive(Store)]`로 세분화된 반응성 자동 생성
- 각 필드별 accessor 자동 생성

### ErrorBoundary (`error_handling.rs`)

```rust
fn app() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |error| rsx! {
                p { "에러 발생: {error}" }
            },
            RiskyComponent {}
        }
    }
}
```

---

## 5. 비동기 처리

### use_future (백그라운드 태스크) (`future.rs`)

```rust
fn app() -> Element {
    let mut count = use_signal(|| 0);

    // 컴포넌트 마운트 시 한 번 실행
    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            count += 1;
        }
    });

    rsx! { p { "타이머: {count}" } }
}
```

### use_resource (데이터 패칭) (`signals.rs`)

```rust
fn app() -> Element {
    let mut query = use_signal(|| "rust".to_string());

    // query가 변경되면 자동으로 다시 실행
    let response = use_resource(move || async move {
        reqwest::get(format!("https://api.example.com/search?q={}", query()))
            .await?
            .json::<SearchResult>()
            .await
    });

    rsx! {
        match &*response.read() {
            Some(Ok(data)) => rsx! { p { "{data}" } },
            Some(Err(e)) => rsx! { p { "에러: {e}" } },
            None => rsx! { p { "로딩 중..." } },
        }
    }
}
```

### use_loader (Suspense용 로더) (`suspense.rs`, `dog_app.rs`)

```rust
// 자동으로 SuspenseBoundary와 연동
fn DogImage() -> Element {
    let url = use_loader(|| async {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await?.json::<DogApi>().await
    })?; // ?로 로딩 중일 때 suspend

    rsx! { img { src: "{url}" } }
}

fn app() -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! { p { "로딩 중..." } },
            DogImage {}
        }
    }
}
```

### use_action (사용자 트리거) (`dog_app.rs`)

```rust
fn app() -> Element {
    let fetch_dog = use_action(|| async {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await?.json::<DogApi>().await
    });

    rsx! {
        button {
            onclick: move |_| fetch_dog.trigger(),
            disabled: fetch_dog.pending(),
            "새 이미지"
        }
        if let Some(Ok(url)) = fetch_dog.value()() {
            img { src: "{url}" }
        }
    }
}
```

### Streams (`streams.rs`)

```rust
fn app() -> Element {
    let mut items = use_signal(Vec::new);

    use_future(move || async move {
        let mut stream = create_stream();
        while let Some(item) = stream.next().await {
            items.write().push(item);
        }
    });

    rsx! {
        for item in items() {
            p { "{item}" }
        }
    }
}
```

### 비동기 중단 (`backgrounded_futures.rs`)

```rust
fn app() -> Element {
    let mut show = use_signal(|| true);

    // show가 false면 future가 일시중단됨
    if !show() {
        return rsx! { button { onclick: move |_| show.set(true), "다시 보기" } };
    }

    use_future(move || async move {
        // show()가 false로 바뀌면 이 future 일시중단
        loop { /* ... */ }
    });

    rsx! { /* ... */ }
}
```

---

## 6. 라우팅

### 기본 라우터 (`simple_router.rs`, `flat_router.rs`)

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]       // 공통 레이아웃
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/blog/:id")]
    Blog { id: String },
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn app() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav {
            Link { to: Route::Home {}, "홈" }
            Link { to: Route::About {}, "소개" }
        }
        Outlet::<Route> {}  // 자식 라우트 렌더링 위치
    }
}

#[component]
fn Blog(id: String) -> Element {
    rsx! { h1 { "블로그 #{id}" } }
}
```

### 중첩 라우터 (`router.rs`)

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(Nav)]
        #[route("/")]
        Home {},

        #[nest("/blog")]
            #[layout(BlogLayout)]
                #[route("/")]
                BlogList {},
                #[route("/:id")]
                BlogPost { id: String },
            #[end_layout]
        #[end_nest]

        #[redirect("/old", || Route::Home {})]
    #[end_layout]

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
```

### Link 컴포넌트 (`link.rs`)

```rust
rsx! {
    // 타입 안전한 내비게이션
    Link {
        to: Route::Blog { id: "123".into() },
        class: "link",
        "블로그로 이동"
    }

    // 외부 링크
    Link {
        to: "https://example.com",
        new_tab: true,
        "외부 링크"
    }
}
```

### 쿼리 파라미터 (`query_segment_search.rs`)

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/search?:query")]
    Search { query: SearchParams },
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
struct SearchParams {
    q: Option<String>,
    page: Option<u32>,
}
```

### Hash Fragment 상태 (`hash_fragment_state.rs`)

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/#:state")]
    Home { state: AppState },
}
```

- URL의 `#` 이후에 CBOR로 직렬화된 앱 상태를 저장/복원

### 라우트 리소스 (`router_resource.rs`)

```rust
#[component]
fn BlogPost(id: ReadSignal<String>) -> Element {
    // id가 ReadSignal이면 라우트 변경 시 자동 업데이트
    let data = use_resource(move || async move {
        fetch_post(&id()).await
    });

    rsx! { /* ... */ }
}
```

### 스크롤 복원 (`router_restore_scroll.rs`)

```rust
// 라우트 간 이동 시 스크롤 위치 자동 저장/복원
fn app() -> Element {
    rsx! {
        Router::<Route> {
            config: || RouterConfig::default().on_update(|state| {
                // 스크롤 위치 복원 로직
            })
        }
    }
}
```

---

## 7. 풀스택

### 서버 함수 기본 (`server_functions.rs`, `fullstack_hello_world.rs`)

```rust
// 서버에서만 실행되는 함수 (RPC 스타일)
#[server]
async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    // DB 쿼리 등 서버 로직
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&pool).await?;
    Ok(posts)
}

// GET 메서드로 노출
#[server(GetPosts, endpoint = "/api/posts")]
#[get]
async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    // ...
}

// 클라이언트에서 호출
fn PostList() -> Element {
    let posts = use_resource(|| get_posts());

    rsx! {
        match &*posts.read() {
            Some(Ok(posts)) => rsx! {
                for post in posts {
                    p { "{post.title}" }
                }
            },
            Some(Err(e)) => rsx! { p { "에러: {e}" } },
            None => rsx! { p { "로딩 중..." } },
        }
    }
}
```

### 커스텀 Axum 서버 (`custom_axum_serve.rs`)

```rust
#[tokio::main]
async fn main() {
    // Dioxus와 커스텀 Axum 라우트 결합
    let app = Router::new()
        .route("/api/custom", get(custom_handler))
        .merge(
            axum::Router::new()
                .serve_dioxus_application(ServeConfig::new(), app)
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn custom_handler() -> impl IntoResponse {
    Json(serde_json::json!({"status": "ok"}))
}
```

### 에러 처리 (`handling_errors.rs`)

```rust
// 방법 1: anyhow::Result
#[server]
async fn fallible() -> Result<String, ServerFnError> {
    Ok("success".into())
}

// 방법 2: HTTP 에러
#[server]
async fn not_found() -> Result<String, ServerFnError> {
    Err(ServerFnError::HttpError(StatusCode::NOT_FOUND, "Not Found".into()))
}

// 방법 3: 커스텀 에러 타입
#[derive(Debug, Clone, Serialize, Deserialize)]
enum AppError {
    NotFound,
    Unauthorized,
}

#[server]
async fn custom_error() -> Result<String, ServerFnError<AppError>> {
    Err(ServerFnError::WrappedServerError(AppError::Unauthorized))
}
```

### 커스텀 에러 페이지 (`custom_error_page.rs`)

```rust
#[server]
async fn render_404() -> Result<(), ServerFnError> {
    // HTTP 상태 코드 설정
    let response = server_context::response();
    response.set_status(StatusCode::NOT_FOUND);
    Ok(())
}
```

### Self-hosted API (`dog_app_self_hosted.rs`)

```rust
// use_loader: 서버에서 자동 실행, 클라이언트에서 hydration
fn DogList() -> Element {
    let dogs = use_loader(|| async {
        reqwest::get("https://dog.ceo/api/breeds/list/all")
            .await?.json::<DogBreeds>().await
    })?;

    rsx! { /* ... */ }
}

// use_action: 사용자 액션으로 트리거
fn ActionButton() -> Element {
    let action = use_action(|breed: String| async move {
        reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
            .await?.json::<DogImage>().await
    });

    rsx! {
        button {
            onclick: move |_| action.trigger("labrador".into()),
            "이미지 가져오기"
        }
    }
}
```

### 미들웨어 (`middleware.rs`)

```rust
use axum::middleware::{self, Next};

// 글로벌 미들웨어
async fn logging_middleware(req: Request, next: Next) -> Response {
    println!("{} {}", req.method(), req.uri());
    next.run(req).await
}

// 타임아웃 미들웨어
async fn timeout_middleware(req: Request, next: Next) -> Response {
    tokio::time::timeout(Duration::from_secs(10), next.run(req))
        .await
        .unwrap_or_else(|_| StatusCode::REQUEST_TIMEOUT.into_response())
}
```

### 로그인 폼 & 쿠키 (`login_form.rs`)

```rust
#[server]
async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    // 인증 로직
    if authenticate(&username, &password).await? {
        // 쿠키 설정
        let response = server_context::response();
        response.insert_header(
            "Set-Cookie",
            format!("session={}; HttpOnly; Path=/", create_session())
        );
        Ok(())
    } else {
        Err(ServerFnError::new("Invalid credentials"))
    }
}

fn LoginForm() -> Element {
    let login_action = use_action(|credentials: (String, String)| async move {
        login(credentials.0, credentials.1).await
    });

    rsx! {
        form {
            onsubmit: move |evt| {
                let data = evt.data();
                login_action.trigger((
                    data.get("username").unwrap(),
                    data.get("password").unwrap(),
                ));
            },
            input { r#type: "text", name: "username" }
            input { r#type: "password", name: "password" }
            button { r#type: "submit", "로그인" }
        }
    }
}
```

### 헤더 접근 (`header_map.rs`, `full_request_access.rs`)

```rust
#[server]
async fn with_headers(headers: HeaderMap) -> Result<String, ServerFnError> {
    let user_agent = headers.get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    Ok(format!("User-Agent: {user_agent}"))
}

// 전체 요청 객체 접근
#[server]
async fn full_access(request: axum::extract::Request) -> Result<String, ServerFnError> {
    let method = request.method().to_string();
    let uri = request.uri().to_string();
    Ok(format!("{method} {uri}"))
}
```

### 쿼리 파라미터 (`query_params.rs`)

```rust
// 방법 1: 직접 바인딩
#[server]
async fn search(q: String, page: Option<u32>) -> Result<Vec<Item>, ServerFnError> {
    // ...
}

// 방법 2: 구조체 역직렬화
#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    page: Option<u32>,
}

#[server]
async fn search(query: SearchQuery) -> Result<Vec<Item>, ServerFnError> {
    // ...
}
```

### 멀티파트 파일 업로드 (`multipart_form.rs`)

```rust
#[server]
async fn upload(data: MultipartFormData) -> Result<String, ServerFnError> {
    while let Some(field) = data.next_field().await? {
        let name = field.name().unwrap_or("unknown").to_string();
        let bytes = field.bytes().await?;
        // 파일 저장 로직
        tokio::fs::write(format!("/uploads/{name}"), &bytes).await?;
    }
    Ok("업로드 완료".into())
}
```

### 스트리밍 파일 업로드 (`streaming_file_upload.rs`)

```rust
#[server]
async fn stream_upload(file: FileStream) -> Result<(), ServerFnError> {
    let mut stream = file.into_stream();
    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        // 청크 단위 처리
    }
    Ok(())
}
```

### 서버 상태 관리 (`server_state.rs`)

```rust
// 방법 1: LazyLock (동기 초기화)
static DB: LazyLock<DbPool> = LazyLock::new(|| {
    DbPool::connect("postgres://localhost/db").unwrap()
});

// 방법 2: Extensions (Axum state)
#[server]
async fn with_state(
    Extension(pool): Extension<DbPool>
) -> Result<String, ServerFnError> {
    // pool 사용
    Ok("ok".into())
}

// 방법 3: FromRef
#[derive(Clone, FromRef)]
struct AppState {
    pool: DbPool,
    cache: RedisClient,
}
```

### Server-Sent Events (`server_sent_events.rs`)

```rust
#[server]
async fn subscribe() -> Result<EventStream, ServerFnError> {
    let stream = async_stream::stream! {
        loop {
            yield Ok(Event::default().data("heartbeat"));
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    };
    Ok(EventStream::new(stream))
}

fn app() -> Element {
    let mut messages = use_signal(Vec::new);

    use_future(move || async move {
        let mut stream = subscribe().await.unwrap();
        while let Some(Ok(event)) = stream.next().await {
            messages.write().push(event.data);
        }
    });

    rsx! { /* ... */ }
}
```

### WebSocket (`websocket.rs`)

```rust
#[server]
async fn ws_handler() -> Result<WebSocket<ChatMessage>, ServerFnError> {
    // 타입 안전한 WebSocket (CBOR 인코딩)
    Ok(WebSocket::new())
}

fn Chat() -> Element {
    let mut messages = use_signal(Vec::new);

    use_future(move || async move {
        let ws = ws_handler().await.unwrap();
        while let Some(msg) = ws.recv().await {
            messages.write().push(msg);
        }
    });

    rsx! { /* ... */ }
}
```

### 스트리밍 (다양한 인코딩) (`streaming.rs`)

```rust
// JSON, CBOR, Postcard 등 다양한 포맷 지원
#[server(encoding = "cbor")]
async fn stream_data() -> Result<StreamingResponse<Data>, ServerFnError> {
    let stream = async_stream::stream! {
        for i in 0..100 {
            yield Ok(Data { value: i });
        }
    };
    Ok(StreamingResponse::new(stream))
}
```

### 리디렉트 (`redirect.rs`)

```rust
#[server]
async fn submit_form(data: FormData) -> Result<(), ServerFnError> {
    // 처리 후 리디렉트
    let response = server_context::response();
    response.set_status(StatusCode::SEE_OTHER);
    response.insert_header("Location", "/success");
    Ok(())
}
```

### 인증 시스템 (`auth/`)

```rust
// axum-session-auth 기반 세션 인증
// SQLite 백엔드로 세션 저장
// 권한 관리 시스템 포함

#[server]
async fn login(
    session: Session<SessionSqlitePool>,
    username: String,
    password: String,
) -> Result<(), ServerFnError> {
    let user = User::authenticate(&username, &password).await?;
    session.sign_in(&user).await;
    Ok(())
}
```

### SSR Only (`ssr-only/`)

```rust
// 클라이언트 번들 없이 서버 사이드 렌더링만
fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(ServeConfig::new().incremental(
            IncrementalRendererConfig::default()
        ))
        .launch(app);
}
```

---

## 8. 에셋 & 스타일링

### CSS 모듈 (`css_modules.rs`)

```rust
#[css_module]
const STYLE: &str = include_str!("./style.module.css");

fn app() -> Element {
    rsx! {
        div { class: STYLE.container,  // 고유 클래스명 자동 생성
            h1 { class: STYLE.title, "CSS 모듈" }
        }
    }
}
```

### 정적 에셋 (`custom_assets.rs`)

```rust
fn app() -> Element {
    rsx! {
        // asset! 매크로로 크로스 플랫폼 에셋 참조
        img { src: asset!("/assets/logo.png") }
        Stylesheet { href: asset!("/assets/style.css") }
    }
}
```

### Meta 태그 & SEO (`meta.rs`, `meta_elements.rs`)

```rust
fn app() -> Element {
    rsx! {
        // Open Graph 메타 태그
        Meta { property: "og:title", content: "내 사이트" }
        Meta { property: "og:description", content: "설명" }
        Meta { property: "og:image", content: "https://example.com/image.png" }

        // 페이지 메타
        Meta { name: "description", content: "페이지 설명" }
        Title { "페이지 제목" }
    }
}
```

### Tailwind CSS 통합 (`tailwind/`)

```rust
fn app() -> Element {
    rsx! {
        Stylesheet { href: asset!("/assets/tailwind.css") }
        div {
            class: "flex flex-col items-center justify-center min-h-screen",
            h1 { class: "text-4xl font-bold text-blue-600", "Tailwind!" }
        }
    }
}
```

---

## 9. API & 유틸리티

### 포커스 제어 (`control_focus.rs`)

```rust
fn app() -> Element {
    let mut input_ref = use_signal(|| None::<MountedData>);

    rsx! {
        input {
            onmounted: move |data| input_ref.set(Some(data.data())),
        }
        button {
            onclick: move |_| async move {
                if let Some(el) = input_ref() {
                    el.set_focus(true).await.ok();
                }
            },
            "포커스"
        }
    }
}
```

### JS 평가 (`eval.rs`)

```rust
fn app() -> Element {
    rsx! {
        button {
            onclick: move |_| async move {
                // JavaScript 실행
                let result = eval("return document.title").await.unwrap();

                // Rust → JS 데이터 전송
                let mut eval = eval(r#"
                    let data = await dioxus.recv();
                    document.title = data;
                "#);
                eval.send("새 제목".into()).unwrap();
            },
            "JS 실행"
        }
    }
}
```

### 폼 처리 (`form.rs`)

```rust
fn app() -> Element {
    rsx! {
        form {
            onsubmit: move |evt| {
                let values = evt.values(); // HashMap<String, FormValue>
                let name = values.get("name").unwrap();
                let email = values.get("email").unwrap();
                // 처리 로직
            },
            input { name: "name", r#type: "text" }
            input { name: "email", r#type: "email" }
            button { r#type: "submit", "제출" }
        }

        // oninput으로 실시간 추적
        form {
            oninput: move |evt| {
                let values = evt.values();
                // 입력값 실시간 반영
            },
            input { name: "search" }
        }
    }
}
```

### 파일 업로드 (`file_upload.rs`)

```rust
fn app() -> Element {
    let mut files = use_signal(Vec::new);

    rsx! {
        // 드래그 앤 드롭 영역
        div {
            ondragover: move |evt| evt.prevent_default(),
            ondrop: move |evt| {
                evt.prevent_default();
                if let Some(file_data) = evt.data().files() {
                    for file in file_data.files() {
                        files.write().push(file);
                    }
                }
            },
            "파일을 여기에 드롭하세요"
        }

        // 파일 입력
        input {
            r#type: "file",
            multiple: true,
            onchange: move |evt| {
                if let Some(file_data) = evt.data().files() {
                    for file in file_data.files() {
                        files.write().push(file);
                    }
                }
            }
        }
    }
}
```

### 드래그 앤 드롭 칸반 (`drag_and_drop.rs`)

```rust
fn KanbanBoard() -> Element {
    rsx! {
        div {
            class: "board",
            for column in columns.iter() {
                div {
                    ondragover: move |evt| evt.prevent_default(),
                    ondrop: move |evt| {
                        let data = evt.data().get_data("text/plain");
                        // 컬럼 간 아이템 이동
                    },
                    for item in column.items.iter() {
                        div {
                            draggable: "true",
                            ondragstart: move |evt| {
                                evt.data().set_data("text/plain", &item.id);
                            },
                            "{item.title}"
                        }
                    }
                }
            }
        }
    }
}
```

### 요소 크기 읽기 (`read_size.rs`)

```rust
fn app() -> Element {
    let mut size = use_signal(|| (0.0, 0.0));

    rsx! {
        div {
            onmounted: move |data| async move {
                let rect = data.data().get_client_rect().await.unwrap();
                size.set((rect.width(), rect.height()));
            },
            "이 요소의 크기: {size().0} x {size().1}"
        }
    }
}
```

### Resize 감지 (`on_resize.rs`)

```rust
fn app() -> Element {
    let mut dimensions = use_signal(|| (0.0, 0.0));

    rsx! {
        div {
            onresize: move |data| {
                let rect = data.data().get_content_rect();
                dimensions.set((rect.width(), rect.height()));
            },
            "크기: {dimensions().0} x {dimensions().1}"
        }
    }
}
```

### Visible 감지 (`on_visible.rs`)

```rust
// Intersection Observer 기반 가시성 감지
fn app() -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div {
            onvisible: move |data| {
                visible.set(data.data().is_intersecting());
            },
            class: if visible() { "fade-in" } else { "fade-out" },
            "스크롤하면 나타납니다"
        }
    }
}
```

### 스크롤 제어 (`scroll_to_top.rs`, `scroll_to_offset.rs`)

```rust
fn app() -> Element {
    let mut container = use_signal(|| None::<MountedData>);

    rsx! {
        div {
            onmounted: move |data| container.set(Some(data.data())),
            // 긴 콘텐츠...
        }
        button {
            onclick: move |_| async move {
                if let Some(el) = container() {
                    el.scroll_to(ScrollBehavior::Smooth, ScrollOffset::top()).await.ok();
                }
            },
            "맨 위로"
        }
    }
}
```

### SSR 렌더링 (`ssr.rs`)

```rust
// 방법 1: VirtualDom으로 렌더링
let mut dom = VirtualDom::new(app);
dom.rebuild_in_place();
let html = dioxus_ssr::render(&dom);

// 방법 2: RSX 직접 렌더링
let html = dioxus_ssr::render_element(rsx! {
    div { "서버에서 렌더링" }
});

// 방법 3: 사전 렌더링 (hydration 가능)
let html = dioxus_ssr::pre_render(&dom);
```

### 커스텀 HTML 템플릿 (`custom_html.rs`)

```rust
fn app() -> Element {
    rsx! {
        // index.html 대신 커스텀 HTML 구조
        head {
            style { "body {{ margin: 0; }}" }
        }
        body {
            div { id: "app", "커스텀 HTML" }
        }
    }
}
```

### 로깅 (`logging.rs`)

```rust
fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("로거 초기화 실패");
    dioxus::launch(app);
}

fn app() -> Element {
    log::info!("앱 렌더링");
    log::debug!("디버그 메시지");
    rsx! { /* ... */ }
}
```

### 타이틀 설정 (`title.rs`)

```rust
fn app() -> Element {
    rsx! {
        // 웹: 페이지 타이틀
        document::Title { "내 앱 - 홈" }
        h1 { "메인 페이지" }
    }
}
```

### XSS 방지 (`xss_safety.rs`)

```rust
fn app() -> Element {
    let user_input = "<script>alert('xss')</script>";

    rsx! {
        // 자동 이스케이프 (안전)
        p { "{user_input}" }
        // 출력: &lt;script&gt;alert('xss')&lt;/script&gt;

        // dangerous_inner_html은 이스케이프하지 않음 (주의!)
        // div { dangerous_inner_html: user_input }
    }
}
```

---

## 10. 레퍼런스

### 제네릭 컴포넌트 (`generic_component.rs`)

```rust
#[component]
fn List<T: Display + PartialEq + Clone + 'static>(
    items: Vec<T>
) -> Element {
    rsx! {
        ul {
            for item in items.iter() {
                li { "{item}" }
            }
        }
    }
}
```

### Optional Props (`optional_props.rs`)

```rust
#[component]
fn MyComponent(
    required: String,                            // 필수
    #[props(default)] with_default: i32,         // 기본값 (0)
    #[props(default = 42)] custom_default: i32,  // 커스텀 기본값
    #[props(!optional)] explicit: Option<String>, // 명시적 Option (Some()으로 전달)
    #[props(optional)] implicit: Option<String>,  // 암시적 Option (값만 전달)
    bare_option: Option<String>,                  // 베어 Option (#[props(optional)]과 동일)
) -> Element {
    rsx! { /* ... */ }
}

// 사용
rsx! {
    MyComponent {
        required: "필수".into(),
        // with_default는 생략 가능 (0)
        custom_default: 100,
        explicit: Some("명시적".into()),
        implicit: "암시적",  // Some() 없이
        // bare_option 생략 가능
    }
}
```

### 웹 컴포넌트 (`web_component.rs`)

```rust
// 웹 컴포넌트를 타입 안전하게 래핑
#[component]
fn MyWebComponent(
    #[props(extends = web_component)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        my-custom-element { ..attributes, {children} }
    }
}
```

### 모든 이벤트 (`all_events.rs`)

```rust
rsx! {
    div {
        // 마우스
        onclick: |e| log!("click: {:?}", e.data()),
        ondoubleclick: |e| log!("dblclick"),
        onmouseenter: |e| log!("mouseenter"),
        onmouseleave: |e| log!("mouseleave"),
        onmousemove: |e| log!("mousemove"),

        // 키보드
        onkeydown: |e| log!("keydown: {}", e.key()),
        onkeyup: |e| log!("keyup"),
        onkeypress: |e| log!("keypress"),

        // 포커스
        onfocus: |e| log!("focus"),
        onblur: |e| log!("blur"),

        // 폼
        oninput: |e| log!("input: {}", e.value()),
        onchange: |e| log!("change"),

        // 스크롤
        onscroll: |e| log!("scroll"),

        // 클립보드
        oncopy: |e| log!("copy"),
        onpaste: |e| log!("paste"),
    }
}
```

---

## 11. 앱 데모 예제

### 계산기 (`calculator.rs`)

- 클로저 기반 상태 관리
- `Signal`이 `Copy` 타입이므로 클로저에서 자연스럽게 사용
- iOS 스타일 UI

### 카운터 리스트 (`counters.rs`)

```rust
fn app() -> Element {
    let mut counters = use_signal(Vec::<i32>::new);
    let sum = use_memo(move || counters().iter().sum::<i32>());

    rsx! {
        button { onclick: move |_| counters.write().push(0), "카운터 추가" }
        p { "합계: {sum}" }
        for (i, _) in counters().iter().enumerate() {
            div {
                key: "{i}",
                button { onclick: move |_| counters.write()[i] += 1, "+" }
                button { onclick: move |_| counters.write()[i] -= 1, "-" }
                span { "{counters()[i]}" }
            }
        }
    }
}
```

### TodoMVC (`todomvc.rs`)

- 전체 TodoMVC 스펙 구현
- `HashMap` 기반 저장소
- `use_memo`로 필터링된 목록 메모이제이션
- 키보드 이벤트 (Enter 키 감지)
- 컴포넌트 분리

### CRM 앱 (`crm.rs`)

- 라우팅 + 전역 상태 조합
- `GlobalSignal`로 고객 리스트 관리
- 폼 핸들링

### 날씨 앱 (`weather_app.rs`)

- 다중 API 통합 (지오코딩 + 날씨)
- Tailwind CSS 스타일링
- 에러 상태 처리
- 로딩 상태 관리

### WebSocket 채팅 (`websocket_chat.rs`)

- 양방향 WebSocket 통신
- 브로드캐스트 채널
- 메시지 영속성

### E-commerce 사이트 (`ecommerce-site/`)

- 풀스택 전자상거래 사이트
- FakeStoreAPI 통합
- 라우팅 기반 페이지 구조
- 컴포넌트 아키텍처 (home, product_page 등)

### HackerNews (`hackernews/`)

- SSR + 스트리밍
- 재귀 컴포넌트 (중첩 댓글)
- `SuspenseBoundary` 활용
- 중첩 컴포넌트 구조

### Hotdog (`hotdog/`)

- 풀스택 앱 (백엔드 + 프론트엔드)
- 서버 함수
- SQLite 통합
- 인증 시스템

---

## 핵심 패턴 요약

| 패턴 | 사용 시점 | Hook/API |
|------|----------|----------|
| 로컬 상태 | 컴포넌트 내 상태 | `use_signal` |
| 파생 상태 | 계산된 값 | `use_memo` |
| 전역 상태 | 앱 전체 공유 | `GlobalSignal` |
| 컨텍스트 | 하위 트리 공유 | `use_context_provider` / `use_context` |
| 부작용 | 상태 변경 반응 | `use_effect` |
| 백그라운드 태스크 | 비동기 작업 | `use_future` |
| 데이터 패칭 | 반응형 API 호출 | `use_resource` |
| Suspense 로딩 | SSR 호환 로딩 | `use_loader` + `SuspenseBoundary` |
| 사용자 액션 | 이벤트 기반 비동기 | `use_action` |
| 서버 함수 | 서버 RPC | `#[server]` |
| 라우팅 | 페이지 내비게이션 | `#[derive(Routable)]` + `Router` |
