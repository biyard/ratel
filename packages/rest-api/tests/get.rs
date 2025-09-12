#[derive(serde::Serialize)]
struct QueryParamsTest {
    key: String,
    value: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Response {
    pub title: String,
}

#[derive(Debug, serde::Deserialize)]
struct ApiError {
    _message: String,
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError {
            _message: err.to_string(),
        }
    }
}

#[tokio::test]
async fn test_get_with_query() -> Result<(), ApiError> {
    let url = "https://jsonplaceholder.typicode.com/posts";
    let query_params = vec![("id", "1")];
    let req = create_get_request_with_params(&url, &query_params);
    println!("{:?}", req.url().to_string());
    let res = rest_api::get_with_query::<Vec<Response>, ApiError, _>(url, &query_params).await?;
    // println!("{:?}", res);
    assert_eq!(
        res[0].title,
        "sunt aut facere repellat provident occaecati excepturi optio reprehenderit"
    );
    Ok(())
}

fn create_get_request_with_params<P>(url: &str, query_params: &P) -> reqwest::Request
where
    P: serde::Serialize + ?Sized,
{
    let client = reqwest::Client::builder().build().unwrap();

    let req = client.get(url).query(query_params);

    req.build().unwrap()
}

#[test]
fn test_query_params_encoding() {
    let url = "https://api.example.com/test";
    let query_params = QueryParamsTest {
        key: "key with spaces".to_string(),
        value: "value&special=chars".to_string(),
    };
    let expected = format!("{url}?key=key+with+spaces&value=value%26special%3Dchars");

    let res = create_get_request_with_params(url, &query_params);
    assert_eq!(res.url().to_string(), expected);
}

#[test]
fn test_query_params_with_unicode() {
    let url = "https://api.example.com/test";
    let query_params = QueryParamsTest {
        key: "한글키".to_string(),
        value: "한글값".to_string(),
    };
    let expected =
        format!("{url}?key=%ED%95%9C%EA%B8%80%ED%82%A4&value=%ED%95%9C%EA%B8%80%EA%B0%92");

    let res = create_get_request_with_params(url, &query_params);
    assert_eq!(res.url().to_string(), expected);
}

#[test]
fn test_query_params_with_special_chars() {
    let url = "https://api.example.com/test";
    let query_params = QueryParamsTest {
        key: "key!@#$%".to_string(),
        value: "value^&*()".to_string(),
    };
    let expected = format!("{url}?key=key%21%40%23%24%25&value=value%5E%26*%28%29");
    let res = create_get_request_with_params(url, &query_params);
    assert_eq!(res.url().to_string(), expected);
}

#[test]
fn test_query_params_empty() {
    let url = "https://api.example.com/test";
    let res = create_get_request_with_params(url, &None::<()>);
    assert_eq!(res.url().to_string(), url);
}

// struct Hook {}

// impl rest_api::RequestHooker for Hook {
//     fn before_request(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
//         req.header("Authorization", format!("Bearer token"))
//     }
// }

// #[test]
// fn test_api_hook() {
//     rest_api::add_hook(Hook {});
// }
