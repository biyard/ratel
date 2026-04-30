use crate::common::ListResponse;
use crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz;
use crate::features::spaces::pages::actions::models::SpaceAction;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct AnalyzeQuizItem {
    pub quiz_id: SpaceQuizEntityType,
    pub title: String,
    pub questions_count: usize,
}

#[get("/api/spaces/{space_id}/apps/analyzes/quizzes?bookmark", role: SpaceUserRole)]
pub async fn list_analyze_quizzes(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<AnalyzeQuizItem>> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();

    let mut opt = SpaceQuiz::opt()
        .sk(EntityType::SpaceQuiz(String::default()).to_string())
        .scan_index_forward(false)
        .limit(20);
    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (quizzes, bookmark) = SpaceQuiz::query(cli, space_pk, opt).await?;

    // Quiz title lives on `SpaceAction` (CompositePartition(space_id, quiz_id),
    // sk = SpaceAction). Batch-get titles in one round trip.
    let action_keys: Vec<(CompositePartition<SpacePartition, String>, EntityType)> = quizzes
        .iter()
        .filter_map(|quiz| match &quiz.sk {
            EntityType::SpaceQuiz(quiz_id) => Some((
                CompositePartition(space_id.clone(), quiz_id.clone()),
                EntityType::SpaceAction,
            )),
            _ => None,
        })
        .collect();

    let actions = if action_keys.is_empty() {
        Vec::new()
    } else {
        SpaceAction::batch_get(cli, action_keys)
            .await
            .unwrap_or_default()
    };
    let title_by_quiz_id: std::collections::HashMap<String, String> = actions
        .into_iter()
        .map(|action| (action.pk.1.clone(), action.title))
        .collect();

    let items = quizzes
        .into_iter()
        .map(|quiz| {
            let quiz_id = SpaceQuizEntityType::from(quiz.sk.clone());
            let title = title_by_quiz_id
                .get(&quiz_id.to_string())
                .cloned()
                .unwrap_or_default();
            AnalyzeQuizItem {
                quiz_id,
                title,
                questions_count: quiz.questions.len(),
            }
        })
        .collect();

    Ok(ListResponse { items, bookmark })
}
