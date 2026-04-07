use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::apps::apps::file::{
    delete_file_link, DeleteFileLinkRequest, FileLinkTarget,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RemoveQuizFileRequest {
    pub file_url: String,
}

#[delete("/api/spaces/{space_pk}/quizzes/{quiz_id}/files", role: SpaceUserRole)]
pub async fn remove_quiz_file(
    space_pk: SpacePartition,
    quiz_id: SpaceQuizEntityType,
    req: RemoveQuizFileRequest,
) -> Result<()> {
    SpaceQuiz::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk_partition: Partition = space_pk.clone().into();
    let link_target = FileLinkTarget::Quiz(quiz_id.to_string());
    let quiz_sk: EntityType = quiz_id.into();

    let quiz = SpaceQuiz::get(cli, &space_pk_partition, Some(quiz_sk.clone()))
        .await?
        .ok_or(Error::NotFound("Quiz not found".into()))?;

    let updated_files: Vec<File> = quiz
        .files
        .into_iter()
        .filter(|f| f.url.as_ref() != Some(&req.file_url))
        .collect();

    SpaceQuiz::updater(&space_pk_partition, &quiz_sk)
        .with_files(updated_files)
        .execute(cli)
        .await?;

    delete_file_link(
        space_pk,
        DeleteFileLinkRequest {
            file_url: req.file_url,
            link_target,
        },
    )
    .await?;

    Ok(())
}
