use bdk::prelude::*;

#[derive(
    Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, schemars::JsonSchema,
)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
#[schemars(bound = "T: JsonSchema")]
pub struct ListItemsResponse<T>
where
    T: Clone
        + serde::Serialize
        + serde::de::DeserializeOwned
        + aide::OperationInput
        + aide::OperationOutput
        + schemars::JsonSchema,
{
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

impl<T> From<(Vec<T>, Option<String>)> for ListItemsResponse<T>
where
    T: Clone
        + serde::de::DeserializeOwned
        + serde::Serialize
        + aide::OperationInput
        + aide::OperationOutput
        + JsonSchema,
{
    fn from((items, bookmark): (Vec<T>, Option<String>)) -> Self {
        Self { items, bookmark }
    }
}

// impl<'de, T> serde::Deserialize<'de> for ListItemsResponse<T>
// where
//     T: Clone
//         + serde::Serialize
//         + serde::Deserialize<'de>
//         + aide::OperationInput
//         + aide::OperationOutput
//         + schemars::JsonSchema,
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         use serde::de::{self, MapAccess, Visitor};
//         use std::fmt;

//         struct ListItemsResponseVisitor<T>(std::marker::PhantomData<T>);

//         impl<'de, T> Visitor<'de> for ListItemsResponseVisitor<T>
//         where
//             T: Clone
//                 + serde::Serialize
//                 + serde::Deserialize<'de>
//                 + aide::OperationInput
//                 + aide::OperationOutput
//                 + schemars::JsonSchema,
//         {
//             type Value = ListItemsResponse<T>;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("struct ListItemsResponse")
//             }

//             fn visit_map<V>(self, mut map: V) -> Result<ListItemsResponse<T>, V::Error>
//             where
//                 V: MapAccess<'de>,
//             {
//                 let mut items = None;
//                 let mut next_bookmark = None;

//                 while let Some(key) = map.next_key()? {
//                     match key {
//                         "items" => {
//                             if items.is_some() {
//                                 return Err(de::Error::duplicate_field("items"));
//                             }
//                             items = Some(map.next_value()?);
//                         }
//                         "next_bookmark" => {
//                             if next_bookmark.is_some() {
//                                 return Err(de::Error::duplicate_field("next_bookmark"));
//                             }
//                             next_bookmark = Some(map.next_value()?);
//                         }
//                         _ => {
//                             let _ = map.next_value::<serde::de::IgnoredAny>()?;
//                         }
//                     }
//                 }

//                 let items = items.ok_or_else(|| de::Error::missing_field("items"))?;
//                 Ok(ListItemsResponse { items, next_bookmark })
//             }
//         }

//         deserializer.deserialize_struct(
//             "ListItemsResponse",
//             &["items", "next_bookmark"],
//             ListItemsResponseVisitor(std::marker::PhantomData),
//         )
//     }
// }
