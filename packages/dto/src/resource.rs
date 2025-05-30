#[derive(Debug, Clone, PartialEq)]
pub enum RatelResource {
    Post { team_id: i64 },
    Reply { team_id: i64 },
    News,
    Promotions,
}
