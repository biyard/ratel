use crate::common::*;

/// A single field in a parent team's customizable application form. The parent
/// admin can add/remove fields, reorder them, and change required flags.
/// Changes are NOT retroactively applied to in-flight applications — each
/// `SubTeamApplication` stores a `form_snapshot` at submit time.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamFormField {
    pub pk: Partition,  // Partition::Team(parent_team_id)
    pub sk: EntityType, // EntityType::SubTeamFormField(field_id)

    pub created_at: i64,
    pub updated_at: i64,

    /// Human-readable field label shown on the apply page.
    pub label: String,
    pub field_type: SubTeamFormFieldType,

    #[serde(default)]
    pub required: bool,

    #[serde(default)]
    pub order: i32,

    /// For SingleSelect / MultiSelect — the allowed choices in display order.
    #[serde(default)]
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum SubTeamFormFieldType {
    #[default]
    ShortText,
    LongText,
    Number,
    Date,
    SingleSelect,
    MultiSelect,
    Url,
}

#[cfg(feature = "server")]
impl SubTeamFormField {
    pub fn new(
        parent_team_pk: Partition,
        label: String,
        field_type: SubTeamFormFieldType,
        required: bool,
        order: i32,
        options: Vec<String>,
    ) -> Self {
        let field_id = uuid::Uuid::new_v4().to_string();
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: parent_team_pk,
            sk: EntityType::SubTeamFormField(field_id),
            created_at: now,
            updated_at: now,
            label,
            field_type,
            required,
            order,
            options,
        }
    }
}
