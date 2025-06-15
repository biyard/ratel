#[derive(Debug, Clone, PartialEq)]
pub enum RatelResource {
    Post { team_id: i64 },
    Reply { team_id: i64 },
    News,
    Promotions,
    Space { space_id: i64 },
    Team { team_id: i64 },
}
