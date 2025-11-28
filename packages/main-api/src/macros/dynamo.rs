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

#[macro_export]
macro_rules! transact_write_all_items {
    ($cli:expr, $tx:expr $(,)? ) => {{
        let mut iter = $tx.chunks(100);

        while let Some(txs) = iter.next() {
            $cli.transact_write_items()
                .set_transact_items(Some(txs.to_vec()))
                .send()
                .await
                .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        }
    }};
}

#[macro_export]
macro_rules! transact_write_all_items_with_failover {
    ($cli:expr, $tx:expr $(,)? ) => {{
        let mut iter = $tx.chunks(100);

        while let Some(txs) = iter.next() {
            for _ in 0..3 {
                match $cli
                    .transact_write_items()
                    .set_transact_items(Some(txs.to_vec()))
                    .send()
                    .await
                    .map_err(Into::<aws_sdk_dynamodb::Error>::into)
                {
                    Ok(_) => break,
                    Err(e) => {
                        eprintln!("Error in transact write items: {:?}", e);
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                }
            }
        }
    }};
}
