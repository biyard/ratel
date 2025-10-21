use uuid::Uuid;

use super::*;
use crate::features::spaces::sprint_leagues::{PlayerImage, SpriteSheet};
use crate::tests::{create_test_user, get_test_aws_config};
use crate::types::{EntityType, Partition};
use crate::utils::aws::DynamoClient;

#[tokio::test]
async fn test_sprint_league_creation() {
    let cli = DynamoClient::mock(get_test_aws_config()).client;

    let user = create_test_user(&cli).await;

    let post_id = Uuid::new_v4().to_string();
    let space_pk = Partition::Space(post_id);

    let sprint_league =
        SprintLeague::new(space_pk.clone()).expect("Failed to create sprint league");

    sprint_league
        .create(&cli)
        .await
        .expect("failed to create sprint league");
    let metadata = SprintLeagueMetadata::query_begins_with_sk(
        &cli,
        &sprint_league.pk,
        EntityType::SprintLeague,
    )
    .await
    .expect("failed to query sprint league metadata");

    assert_eq!(metadata.len(), 1);
    let player_image = PlayerImage {
        select: SpriteSheet {
            json: "select.json".to_string(),
            image: "select.png".to_string(),
        },
        run: SpriteSheet {
            json: "run.json".to_string(),
            image: "run.png".to_string(),
        },
        win: "win.png".to_string(),
        lose: "lose.png".to_string(),
    };

    let player_1 = SprintLeaguePlayer::new(
        sprint_league.pk.clone(),
        0,
        "Player 1".into(),
        "Description".into(),
        player_image.clone(),
    )
    .unwrap();

    let player_2 = SprintLeaguePlayer::new(
        sprint_league.pk.clone(),
        1,
        "Player 2".into(),
        "Description".into(),
        player_image.clone(),
    )
    .unwrap();

    let player_3 = SprintLeaguePlayer::new(
        sprint_league.pk.clone(),
        2,
        "Player 3".into(),
        "Description".into(),
        player_image.clone(),
    )
    .unwrap();

    let txs = vec![
        player_1.create_transact_write_item(),
        player_2.create_transact_write_item(),
        player_3.create_transact_write_item(),
    ];

    cli.transact_write_items()
        .set_transact_items(Some(txs))
        .send()
        .await
        .expect("failed to create players");

    let metadata = SprintLeagueMetadata::query_begins_with_sk(
        &cli,
        &sprint_league.pk,
        EntityType::SprintLeague,
    )
    .await
    .expect("failed to query sprint league metadata");
    assert_eq!(metadata.len(), 4);
    let players: Vec<SprintLeaguePlayer> = metadata
        .into_iter()
        .filter_map(|m| {
            if let SprintLeagueMetadata::SprintLeaguePlayer(player) = m {
                Some(player)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(players.len(), 3);

    let is_voted = SprintLeague::is_voted(&cli, &space_pk, &user.pk)
        .await
        .expect("Failed to check if user voted");
    assert!(!is_voted);
    let res = SprintLeague::vote(&cli, &space_pk, &user.pk, &player_2.sk, None).await;

    assert!(res.is_ok());

    let is_voted = SprintLeague::is_voted(&cli, &space_pk, &user.pk)
        .await
        .expect("Failed to check if user voted");

    assert!(is_voted);

    // let (players, _) = SprintLeaguePlayer::query_order_by_votes(&cli, &space_pk, None)
    //     .await
    //     .expect("Failed to query players by votes");
    // assert_eq!(players[0].sk, player_2.sk);
    // assert_eq!(players[0].votes, 1);

    let res = SprintLeague::vote(&cli, &space_pk, &user.pk, &player_1.sk, None).await;
    assert!(res.is_err(), "Sprint league double voting should fail");
}
