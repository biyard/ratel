use crate::common::*;
use crate::features::sub_team::types::TeamProfileLink;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// A single field in a parent team's customizable application form. The parent
/// admin can add/remove fields, reorder them, and change required flags.
/// Changes are NOT retroactively applied to in-flight applications — each
/// `SubTeamApplication` stores a `form_snapshot` at submit time.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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

    /// When set, the apply page prefills this field with the applicant
    /// team's matching attribute (e.g. `display_name`) and renders the
    /// field read-only. `None` means the applicant types a free answer.
    #[serde(default)]
    pub linked_to: Option<TeamProfileLink>,

    /// System-seeded default field that admins can't edit or delete
    /// from the form builder. Used for the auto-populated "팀 이름"
    /// (linked to team.display_name) and "설립 목적" (team.description)
    /// rows that ship with every parent-eligible team.
    #[serde(default)]
    pub locked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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
        linked_to: Option<TeamProfileLink>,
    ) -> Self {
        Self::new_with_lock(
            parent_team_pk,
            label,
            field_type,
            required,
            order,
            options,
            linked_to,
            false,
        )
    }

    pub fn new_with_lock(
        parent_team_pk: Partition,
        label: String,
        field_type: SubTeamFormFieldType,
        required: bool,
        order: i32,
        options: Vec<String>,
        linked_to: Option<TeamProfileLink>,
        locked: bool,
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
            linked_to,
            locked,
        }
    }
}
