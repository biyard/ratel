use crate::features::spaces::SpaceRequirementType;
use crate::types::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct SpaceRequirement {
    pub pk: CompositePartition,
    pub sk: EntityType,

    pub typ: SpaceRequirementType,

    pub related_pk: String, // Partition or CompositePartition serialized as string
    pub related_sk: EntityType, // EntityType

    #[dynamo(prefix = "REQ", name = "find_by_order", index = "gsi1", pk)]
    pub space_pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub order: i64,
}

impl SpaceRequirement {
    pub fn new(
        space_pk: Partition,
        typ: SpaceRequirementType,
        (related_pk, related_sk): (String, EntityType),
    ) -> Self {
        if !matches!(space_pk, Partition::Space(_)) {
            panic!("space_pk must be of Partition::Space type");
        }

        Self {
            pk: CompositePartition(space_pk.clone(), Partition::Requirement),
            sk: EntityType::SpaceRequirement(typ.to_string()),
            typ,
            related_pk,
            related_sk,
            space_pk,
            order: 1,
        }
    }

    pub fn keys(
        space_pk: &Partition,
        typ: Option<SpaceRequirementType>,
    ) -> (CompositePartition, EntityType) {
        (
            CompositePartition(space_pk.clone(), Partition::Requirement),
            EntityType::SpaceRequirement(typ.map_or("".to_string(), |t| t.to_string())),
        )
    }
}
