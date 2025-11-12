use super::*;
use crate::{features::spaces::SpaceRequirement, *};

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
pub struct SpaceRequirementDto {
    pub related_pk: String,
    pub related_sk: EntityType,
    pub order: i64,
    pub typ: SpaceRequirementType,
    pub responded: bool,
}

impl SpaceRequirementDto {
    pub fn new(
        req: SpaceRequirement,
        user: &Option<User>,
        resp: &Vec<SpaceRequirementResponse>,
    ) -> Self {
        let responded = if let Some(user) = user {
            let (pk, sk) = req
                .get_respondent_keys(&user.pk)
                .expect("failed to get respondent key");

            resp.iter()
                .any(|e| e.pk() == pk.to_string() && e.sk() == sk.to_string())
        } else {
            false
        };

        let mut dto: Self = req.into();
        dto.responded = responded;
        dto
    }
}

impl From<SpaceRequirement> for SpaceRequirementDto {
    fn from(req: SpaceRequirement) -> Self {
        Self {
            related_pk: req.related_pk,
            related_sk: req.related_sk,
            order: req.order,
            typ: req.typ,
            responded: false,
        }
    }
}
