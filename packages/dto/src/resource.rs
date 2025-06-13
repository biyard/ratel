#[derive(Debug, Clone, PartialEq)]
pub enum RatelResource {
    Post { team_id: i64 },
    Reply { team_id: i64 },
    News,
    Promotions,
    Space { space_id: i64 },

    InviteMember { team_id: i64, group_id: i64 },
    UpdateGroup { team_id: i64, group_id: i64 },
    DeleteGroup { team_id: i64, group_id: i64 },
}
