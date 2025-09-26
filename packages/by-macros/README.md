# Biyard Macros
The `api_model` macro generates an API model with database and repository support.

## Features
- **Generates a database table** based on the struct definition.
- **Creates repository functions** (`insert`, `update`, `delete`, `find`, `find_one`, and functions to create tables).
- **Auto-generates API client functions** for RESTful interactions.
- **Supports action-based API calls** (`action` and `action_by_id`).
- **Integrates with SQLx for efficient database queries.**

---

## 📌 Key Structures

### **1. Original Structure**
The main struct includes functions to obtain the `Client` (for web interactions) and `Repository` (for server-side operations).

- Given `ExampleModel` as an `api_model`, the following methods are available:
  - `ExampleModel::get_client(endpoint)` → Returns `ExampleModelClient`
  - `ExampleModel::get_repository(postgres_pool)` → Returns `ExampleModelRepository` (only for `server` feature)

### **2. API Client Structure**
- Named with the suffix `Client` (e.g., `ExampleModelClient`).
- Provides API actions based on the `base` attribute.

### **3. Repository Structure**
- Named with the suffix `Repository` (e.g., `ExampleModelRepository`).
- Used for direct database interactions (currently supports PostgreSQL).

### **4. Summary Model**
- Named with the suffix `Summary` (e.g., `ExampleModelSummary`).
- Used for bulk data retrieval in query actions.
- Includes only fields marked with `summary`.

---

## 📌 Client Structure

### **Actions**

#### Types of Actions
- **Query Action (`query_action`, `queryable`)**
  - Describes bulk data listing.
  - Defined as a field attribute.

- **Read Action (`read_action`)**
  - Fetches a single entity by a unique identifier.

- **Action (`action`)**
  - Supports custom REST API actions.

- **Action by ID (`action_by_id`)**
  - Defines API actions that require an ID parameter.

#### Query Action (`query_action`, `queryable`)
Query action is used to retrieve multiple records from the database in a paginated manner.
It is automatically generated for models that have the `queryable` or `query_action` attribute defined in a field.

- It returns a paginated response wrapped in a `QueryResponse<T>` structure where `iter_type` is `QueryResponse`.
  - At this, `iter_type` must implement `From<(i64, T)>` trait.
- The query parameters such as `size` and `page` are supported to control the data retrieval.
- The response contains both the data and metadata (e.g., total count, pagination details).
- `Query param` structure, which is named by suffix of `Query` is composed with fields described as `queryable` or `query_action`

**Example Usage:**

```no_run
let client = ExampleModel::get_client("https://api.example.com");
let query_params = ExampleModelQuery::new(10).with_page(2);
let response = client.query(query_params).await?;
println!("Total Count: {}", response.total_count);
for item in response.items {
    println!("ID: {}, Name: {}", item.id, item.name);
}
```

##### Differences between `queryable` and `query_action**
- `queryable` only makes a field to `Query` structure.
- `query_action` makes a field and a function to `Query** structure

**Example Usage:**

```no_run
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};

pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct QueryResponse<T> {
    pub items: Vec<T>,
    pub total_count: i64,
}

impl<T> From<(Vec<T>, i64)> for QueryResponse<T> {
    fn from((items, total_count): (Vec<T>, i64)) -> Self {
        QueryResponse { items, total_count }
    }
}

#[api_model(base = "/examples", iter_type = QueryResponse)]
pub struct ExampleModel {
    #[api_model(summary, primary_key)]
    pub id: String,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(summary, query_action = list_by_status)]
    pub status: i32,
    #[api_model(summary, queryable)]
    pub area: String,
}

async fn test_query() {
   let cli = ExampleModel::get_client("http://localhost:3000");

   // it makes an API call to GET http://localhost:3000/examples?param-type=query&area=test_area&size=10
   // res will be `Result<QueryResponse<ExampleModel>>`. If `iter_type` is described as and other generic response, it will be the described type. Don't forget implementing `From<(i64,T)>` trait for the reponse type.
   // And also abbreviated Result must be defined or imported in this file.
   let res = cli.query(ExampleModelQuery::new(10).with_area("test_area".to_string())).await;

   // it makes an API call to GET http://localhost:3000/examples?param-type=query&area=test_area&size=10&page=2
   cli.query(ExampleModelQuery::new(10).with_page(2).with_area("test_area".to_string())).await;

   // `list_by_status` is generated by `query_action` attribute.
   // it makes an API call to GET http://localhost:3000/examples?param-type=query&status=3&size=10&page=2
   cli.list_by_status(10, Some(2.to_string()), 3).await;
}
```

##### Principle of query
For composition of query url, It uses `Param` structure. ex) `ExampleModelParam`.
It will declared as below:

```no_run
#[serde(rename = "kebab-case", tag = "param-type")]
pub struct ExampleModelParam {
    Query(ExampleModelQuery)
    Read(ExampleModelReadAction)
}
```

If no `read_action`, `Param` is only declared with Query.
At this, `Query` is always defined even if no `queryable` or `query_action`.
If no `query*` is described, `ExampleModelQuery` only has `size` and `bookmark` fields.
Note that `page` will be described in `bookmark` as string in SQL.

##### Handling requests in server side
Server side handle can handle `Param` or `Query` structure.
If server side handler handle only `Query` like `ExampleModelQuery`, it ignores `param-type` query argument.
If you don't have a plan to support `read_action`, you can only hanle `Query` instead of `Param`.


#### Read Action (`read_action`)
Read action(`read_action`) can be described as an structure attribute or a field attribute.

##### Structure attribute
It can be formed by three types; a function only, multiple functions, functions with additional parameters.
Structure attribute can be utilized in two circumstances; 
- defining a API call without any parameter
- adding a custom field to a specific read action.

The below explains how to use `read_action` as structure attribute.

- The simplest definition defines only a function.
  - `read_action = get_data`
- Multiple functions define two or more functions.
  - `read_action = [get_data, verify]`
- Functions with additional parameter define functions with additional parameter, which this structure does not have.
  - `read_action = [get_data, verify(code = String, email = String)]`

##### Field attribute
It supports a single function and multiple functions form.
Also it allows you to change to a custom parameter type.
In contrast of `structure attiribute`, field

- a single and multiple functions definition are same with `structure attribute`.
  - `read_action = get_data`, `read_action = [get_data, verify]`
  - Basically, `read_action` in field attribute makes a field with the same type to `ReadAction` structure.
- If you want to change action type, you can use `related` attiribute.
  - `related = CustomActionRequest` makes `Client` use `CustomActionRequest`


#### Action (`action`)

#### Action by id (`action_by_id`)

## 📌 Repository Structure

- Provides functions to interact with the database.
- Implements CRUD operations via SQLx.
- Automatically generates SQL table creation scripts.

#### Example Usage
```no_run
let repo = ExampleModel::get_repository(pool);
let new_item = repo.insert("Example Name").await?;
let updated_item = repo.update("123", ExampleModelRepositoryUpdateRequest { name: Some("Updated") }).await?;
let deleted = repo.delete("123").await?;
```

---

## 📌 Example

```no_run
#[api_model(
    base = "/examples",
    table = example_table,
    iter_type = QueryResponse,
    action_by_id = delete,
    action = [no_param, empty]
)]
pub struct ExampleModel {
    #[api_model(summary, primary_key, read_action = find_by_id)]
    pub id: String,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(summary)]
    pub name: String,
}
```

---

## **📌 Conclusion**
The `#[api_model]` macro automatically generates **database models, API request structures, clients, and action enums**.  
This enables **efficient RESTful API and database interactions**.


## Postgres Setup
### Add `set_updated_at` function
- Below function will be used by auto updated TIMESTAMP like `updated_at`

``` sql
  CREATE OR REPLACE FUNCTION set_created_at()
    RETURNS TRIGGER AS $$
    BEGIN
      NEW.created_at := EXTRACT(EPOCH FROM now()) * 1000;

      RETURN NEW;
    END;
  $$ LANGUAGE plpgsql;
```

``` sql
  CREATE OR REPLACE FUNCTION set_updated_at()
    RETURNS TRIGGER AS $$
    BEGIN
      NEW.updated_at := EXTRACT(EPOCH FROM now()) * 1000;

      RETURN NEW;
    END;
  $$ LANGUAGE plpgsql;
```

- You can check if the function was created as below

``` sql
  SELECT proname FROM pg_proc WHERE proname = 'set_created_at;
```


## API Model
### Usage
- `crate::Result` must be declared.
- For using `ApiError` as result error type, you should implement `From<String>` into inner error type
- If you want to use fully customized error type, you should implement `From<reqwest::Error>`.

``` rust
#[cfg(feature = "server")]
use by_axum::aide;

use by_macros::api_model;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[api_model(base = "/topics/v1", iter_type=Vec, read_action = no_param_action)]
pub struct Topic {
    #[api_model(summary)]
    pub id: String,
    #[api_model(read_action = user_info)]
    pub wallet_address: String,
    #[api_model(read_action = [check_email,user_info])]
    pub email: String,
    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, queryable, query_action = search_by, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, queryable, action_by_id = update, read_action = user_info)]
    pub status: i32,
    #[api_model(summary, query_action = [search_by, date_from])]
    pub created_at: i64,
    pub is_liked: bool,

    pub updated_at: i64,
    #[api_model(action_by_id = like, related = CommentRequest)]
    #[api_model(action = comment, related = Comment)]
    pub comments: Vec<Comment>,

    #[api_model(action_by_id = update)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub content: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommentRequest {
    pub comment_id: String,
    pub is_liked: bool,
}
```

#### Expanded code

``` rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct Topic {
    pub id: String,
    pub wallet_address: String,
    pub email: String,
    pub title: String,
    pub description: String,
    pub status: i32,
    pub created_at: i64,
    pub is_liked: bool,
    pub updated_at: i64,
    pub comments: Vec<Comment>,
    pub tags: Vec<String>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TopicAction {
    Comment(Comment),
    Create(TopicCreateRequest),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicCreateRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TopicByIdAction {
    Update(TopicUpdateRequest),
    Like(CommentRequest),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicUpdateRequest {
    pub description: String,
    pub status: i32,
    pub tags: Vec<String>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicSummary {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: i32,
    pub created_at: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, by_macros::QueryDisplay)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicQuery {
    pub size: usize,
    pub bookmark: Option<String>,
    pub action: Option<TopicQueryActionType>,
    pub description: Option<String>,
    pub status: Option<i32>,
    pub created_at: Option<i64>,
}
impl TopicQuery {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            ..Self::default()
        }
    }
    pub fn with_bookmark(mut self, bookmark: String) -> Self {
        self.bookmark = Some(bookmark);
        self
    }
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }
    pub fn search_by(mut self, description: String, created_at: i64) -> Self {
        self.description = Some(description);
        self.created_at = Some(created_at);
        self.action = Some(TopicQueryActionType::SearchBy);
        self
    }
    pub fn date_from(mut self, created_at: i64) -> Self {
        self.created_at = Some(created_at);
        self.action = Some(TopicQueryActionType::DateFrom);
        self
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TopicQueryActionType {
    SearchBy,
    DateFrom,
}

impl Topic {
    pub fn get_client(endpoint: &str) -> TopicClient {
        TopicClient {
            endpoint: endpoint.to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct TopicClient {
    pub endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, by_macros::QueryDisplay)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicReadAction {
    pub action: Option<TopicReadActionType>,
    pub wallet_address: Option<String>,
    pub email: Option<String>,
    pub status: Option<i32>,
}
impl TopicReadAction {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn user_info(mut self, wallet_address: String, email: String, status: i32) -> Self {
        self.wallet_address = Some(wallet_address);
        self.email = Some(email);
        self.status = Some(status);
        self.action = Some(TopicReadActionType::UserInfo);
        self
    }
    pub fn check_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self.action = Some(TopicReadActionType::CheckEmail);
        self
    }

    pub fn no_param_action(mut self) -> Self {
        self.action = Some(TopicReadActionType::NoParamAction);
        self
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TopicReadActionType {
    UserInfo,
    CheckEmail,
}
impl TopicClient {
    pub async fn no_param_action(
        &self,
    ) -> crate::Result<Topic> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let params = TopicReadAction::new().no_param_action();
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn user_info(
        &self,
        wallet_address: String,
        email: String,
        status: i32,
    ) -> crate::Result<Topic> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let params = TopicReadAction::new().user_info(wallet_address, email, status);
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn check_email(&self, email: String) -> crate::Result<Topic> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let params = TopicReadAction::new().check_email(email);
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn query(&self, params: TopicQuery) -> crate::Result<Vec<TopicSummary>> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn get(&self, id: &str) -> crate::Result<Topic> {
        let endpoint = format!("{}{}/{}", self.endpoint, "/topics/v1", id);
        rest_api::get(&endpoint).await
    }
    pub async fn search_by(
        &self,
        size: usize,
        bookmark: Option<String>,
        description: String,
        created_at: i64,
    ) -> crate::Result<Vec<TopicSummary>> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let params = TopicQuery {
            size,
            bookmark,
            action: Some(TopicQueryActionType::SearchBy),
            description: Some(description),
            created_at: Some(created_at),
            ..TopicQuery::default()
        };
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn date_from(
        &self,
        size: usize,
        bookmark: Option<String>,
        created_at: i64,
    ) -> crate::Result<Vec<TopicSummary>> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let params = TopicQuery {
            size,
            bookmark,
            action: Some(TopicQueryActionType::DateFrom),
            created_at: Some(created_at),
            ..TopicQuery::default()
        };
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn act(&self, params: TopicAction) -> crate::Result<Topic> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        rest_api::post(&endpoint, params).await
    }
    pub async fn create(&self, title: String, description: String) -> crate::Result<Topic> {
        let endpoint = format!("{}{}", self.endpoint, "/topics/v1");
        let req = TopicAction::Create(TopicCreateRequest { title, description });
        rest_api::post(&endpoint, req).await
    }
    pub async fn act_by_id(&self, id: &str, params: TopicByIdAction) -> crate::Result<Topic> {
        let endpoint = format!("{}{}/{}", self.endpoint, "/topics/v1", id);
        rest_api::post(&endpoint, params).await
    }
    pub async fn update(
        &self,
        id: &str,
        description: String,
        status: i32,
        tags: Vec<String>,
    ) -> crate::Result<Topic> {
        let endpoint = format!("{}{}/{}", self.endpoint, "/topics/v1", id);
        let req = TopicByIdAction::Update(TopicUpdateRequest {
            description,
            status,
            tags,
        });
        rest_api::post(&endpoint, req).await
    }
}
```

### Define a model with parent ID
- If a resource is based on a specific parent resource, you can specify a parent ID with `:`.
  - Usually, the ID should be kebab-case by REST API convention.

``` rust
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[api_model(base = "/topics/v1/:topic-id/comments", iter_type=Vec)]
pub struct Comment {
    pub id: String,
    #[api_model(action = comment, related = String, read_action = search_by)]
    pub content: String,
    #[api_model(action_by_id = update, related = i64)]
    pub updated_at: i64,
}
```

#### Expanded code

``` rust
pub struct Comment {
    pub id: String,
    pub content: String,
    pub updated_at: i64,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum CommentAction {
    Comment(String),
}
impl CommentClient {
    pub async fn act(&self, topic_id: &str, params: CommentAction) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}", self.endpoint, path);
        rest_api::post(&endpoint, params).await
    }
    pub async fn comment(&self, topic_id: &str, request: String) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}", self.endpoint, path);
        rest_api::post(&endpoint, request).await
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum CommentByIdAction {
    Update(i64),
}
impl CommentClient {
    pub async fn act_by_id(
        &self,
        topic_id: &str,
        id: &str,
        params: CommentByIdAction,
    ) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}/{}", self.endpoint, path, id);
        rest_api::post(&endpoint, params).await
    }
    pub async fn update(&self, topic_id: &str, id: &str, request: i64) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}/{}", self.endpoint, path, id);
        rest_api::post(&endpoint, request).await
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CommentSummary {}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, by_macros::QueryDisplay)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CommentQuery {
    pub size: usize,
    pub bookmark: Option<String>,
}
impl CommentQuery {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            ..Self::default()
        }
    }
    pub fn with_bookmark(mut self, bookmark: String) -> Self {
        self.bookmark = Some(bookmark);
        self
    }
}
impl CommentClient {}
impl Comment {
    pub fn get_client(endpoint: &str) -> CommentClient {
        CommentClient {
            endpoint: endpoint.to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct CommentClient {
    pub endpoint: String,
}
impl CommentClient {
    pub async fn query(
        &self,
        topic_id: &str,
        params: CommentQuery,
    ) -> crate::Result<Vec<CommentSummary>> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}", self.endpoint, path);
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
    pub async fn get(&self, topic_id: &str, id: &str) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}/{}", self.endpoint, path, id);
        rest_api::get(&endpoint).await
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, by_macros::QueryDisplay)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CommentReadAction {
    pub action: Option<CommentReadActionType>,
    pub content: Option<String>,
}
impl CommentReadAction {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn search_by(mut self, content: String) -> Self {
        self.content = Some(content);
        self.action = Some(CommentReadActionType::SearchBy);
        self
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum CommentReadActionType {
    SearchBy,
}
impl CommentClient {
    pub async fn search_by(&self, topic_id: &str, content: String) -> crate::Result<Comment> {
        let path = format!("/topics/v1/{}/comments", topic_id,);
        let endpoint = format!("{}{}", self.endpoint, path);
        let params = CommentReadAction::new().search_by(content);
        let query = format!("{}?{}", endpoint, params);
        rest_api::get(&query).await
    }
}

```

## Troublshooting
### Panic with parent route
Assume `Panel` model is a child endpoint of `organizations`.
It means that `Panel` model has a field mapping to organizations in `many_to_one`.
At this, `org_id` is the foreign reference key of `id` of `organizations` table.
And implicitly, `:org-id` in `base` will be `org_id` at the last field.

The below code generates `Client` having all function with the first parent id(`org_id`, which is snake_case of `:org-id`).

If action was described in `org_id` field, it generates `create` function in `Client` structure as `pub fn create(&self, org_id: &str, name: String, user_count: u64, org_id: String)`.
At this, the first parameter(`org_id: &str`) and the last parameter(`org_id: String`) are conflict.

In order to overcome this issue, there are two options.

- rename `:org-id` to `:organization-id`.
  - It generate `create` function as `pub fn create(&self, organization_id: &str, name: String, user_count: u64, org_id: String)`
- However, usually you might not need to make a additional parameter or `org_id` because all `Client` functions have `org-id` as their first argument. And it may indicate the `org_id: String`.
  - This means usually, `many_to_one` relationship will be expressed as a child endpoint in HTTP API concept.

``` rust
#[derive(validator::Validate)]
#[api_model(base = "organizations/v2/:org-id/panels", table = panels, iter_type=QueryResponse)]
pub struct Panel {
    #[api_model(summary, primary_key, action = delete, read_action = [get_panel, find_by_id])]
    pub id: String,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [create], action_by_id = update)]
    pub name: String,
    #[api_model(summary, action = [create], action_by_id = update)]
    pub user_count: u64,
    #[api_model(summary, many_to_one = organizations, action = [create])]
    pub org_id: String,
}
```
