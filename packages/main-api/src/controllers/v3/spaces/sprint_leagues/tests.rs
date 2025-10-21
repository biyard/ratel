use crate::controllers::v3::spaces::tests::setup_space;
use crate::features::spaces::sprint_leagues::{
    CreatePlayerRequest, PlayerImage, SprintLeagueResponse, SpriteSheet,
};
use crate::tests::v3_setup::TestContextV3;
use crate::types::SpaceType;
use crate::*;

async fn setup_sprint_league(ctx: &TestContextV3, space_pk: &str) -> SprintLeagueResponse {
    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;
    let players = vec![
        CreatePlayerRequest {
            name: "LEE JUN".into(),
            description: "The fastest runner in the east.".into(),
            player_image: PlayerImage {
                run: SpriteSheet {
                    json: "https://metadata.ratel.foundation/assets/lee_jun_run.json".into(),
                    image: "https://metadata.ratel.foundation/assets/lee_jun_run.webp".into(),
                },
                win: "https://metadata.ratel.foundation/assets/lee_jun_win.png".into(),
                lose: "https://metadata.ratel.foundation/assets/lee_jun_lose.png".into(),
                select: SpriteSheet {
                    json: "https://metadata.ratel.foundation/assets/lee_jun_selected.json".into(),
                    image: "https://metadata.ratel.foundation/assets/lee_jun_selected.webp".into(),
                },
            },
        },
        CreatePlayerRequest {
            name: "KIM MOON".into(),
            description: "A sprinter with unmatched agility.".into(),
            player_image: PlayerImage {
                run: SpriteSheet {
                    json: "https://metadata.ratel.foundation/assets/kim_moon_run.json".into(),
                    image: "https://metadata.ratel.foundation/assets/kim_moon_run.webp".into(),
                },
                win: "https://metadata.ratel.foundation/assets/kim_moon_win.png".into(),
                lose: "https://metadata.ratel.foundation/assets/kim_moon_lose.png".into(),
                select: SpriteSheet {
                    json: "https://metadata.ratel.foundation/assets/kim_moon_selected.json".into(),
                    image: "https://metadata.ratel.foundation/assets/kim_moon_selected.webp".into(),
                },
            },
        },
        CreatePlayerRequest {
            name: "LEE JAE".into(),
            description: "LEE JAE".into(),
            player_image: PlayerImage {
                run: SpriteSheet {
                    json: "https://metadata.ratel.foundation/assets/lee_jae_run.json".into(),
                    image: "https://metadata.ratel.foundation/assets/lee_jae_run.webp".into(),
                },
                win: "https://metadata.ratel.foundation/assets/lee_jae_win.png".into(),
                lose: "https://metadata.ratel.foundation/assets/lee_jae_lose.png".into(),
                select: SpriteSheet {
                    json: "https://metadata.ratel.foundation/assets/lee_jae_selected.json".into(),
                    image: "https://metadata.ratel.foundation/assets/lee_jae_selected.webp".into(),
                },
            },
        },
    ];
    let (status, _, res) = put! {
        app: app,
        path: format!("/v3/spaces/{}/sprint-leagues", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "players": players
        },
        response_type: SprintLeagueResponse,
    };
    assert_eq!(status, 200);
    res
}
#[tokio::test]
pub async fn test_sprint_league_upsert() {
    let (ctx, space_pk) = setup_space(SpaceType::SprintLeague).await;

    let sprint_league = setup_sprint_league(&ctx, &space_pk).await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;

    let encoded_pk = percent_encoding::percent_encode(
        space_pk.to_string().as_bytes(),
        percent_encoding::NON_ALPHANUMERIC,
    )
    .to_string();

    let (status, _, res) = get! {
        app: app,
        path: format!("/v3/spaces/{}/sprint-leagues", encoded_pk),
        headers: headers.clone(),
        response_type: SprintLeagueResponse
    };
    assert_eq!(status, 200);
    for (org, res) in sprint_league.players.iter().zip(res.players.iter()) {
        assert_eq!(org.name, res.name);
        assert_eq!(org.description, res.description);
        assert_eq!(org.player_image.run.json, res.player_image.run.json);
        assert_eq!(org.player_image.run.image, res.player_image.run.image);
        assert_eq!(org.player_image.win, res.player_image.win);
        assert_eq!(org.player_image.lose, res.player_image.lose);
        assert_eq!(org.player_image.select.json, res.player_image.select.json);
        assert_eq!(org.player_image.select.image, res.player_image.select.image);
    }
    // Modify player names
    let next_players: Vec<CreatePlayerRequest> = res
        .players
        .into_iter()
        .map(|player| CreatePlayerRequest {
            name: format!("{} UPDATED", player.name),
            description: player.description,
            player_image: player.player_image,
        })
        .collect();

    let (status, _, res) = put! {
        app: app,
        path: format!("/v3/spaces/{}/sprint-leagues", space_pk.to_string()),
        headers: headers.clone(),
        body: {
            "players": next_players,
        },
        response_type: SprintLeagueResponse,
    };

    assert_eq!(status, 200);
    for (org, res) in next_players.iter().zip(res.players.iter()) {
        assert_eq!(org.name, res.name);
    }
}

#[tokio::test]
async fn test_sprint_league_vote() {
    let (ctx, space_pk) = setup_space(SpaceType::SprintLeague).await;
    let (_, user2_headers) = ctx.create_another_user().await;

    let sprint_league = setup_sprint_league(&ctx, &space_pk).await;

    let TestContextV3 {
        app,
        test_user: (_user, headers),
        ..
    } = ctx;

    let player = sprint_league.players.first().unwrap();

    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/sprint-leagues/votes", space_pk),
        headers: headers.clone(),
        body: { "player_sk": player.sk.to_string() }
    };

    assert_eq!(status, 200);

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/sprint-leagues", space_pk),
        headers: headers.clone(),
        response_type: SprintLeagueResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.players.len(), 3);
    assert_eq!(
        body.players
            .iter()
            .find(|p| p.sk == player.sk)
            .unwrap()
            .votes,
        1
    );
    assert_eq!(body.votes, 1);
    assert_eq!(body.is_voted, true);

    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/sprint-leagues/votes", space_pk),
        headers: user2_headers.clone(),
        body: { "player_sk": player.sk.to_string() }
    };

    assert_ne!(status, 200);
}
