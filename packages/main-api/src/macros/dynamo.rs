#[macro_export]
macro_rules! transact_write {
    ($cli:expr $(, $tx:expr )* $(,)?) => {{
        use aws_sdk_dynamodb::types::TransactWriteItem;

        let __items: Vec<TransactWriteItem> = vec![$($tx),*];

        $cli
            .transact_write_items()
            .set_transact_items(Some(__items))
            .send()
            .await.map_err(Into::<aws_sdk_dynamodb::Error>::into)
    }};
}

#[macro_export]
macro_rules! transact_write_items {
    ($cli:expr, $tx:expr $(,)? ) => {{
        $cli.transact_write_items()
            .set_transact_items(Some($tx))
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)
    }};
}
