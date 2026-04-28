//! Auto analysis pipeline — runs once per AnalyzeReport on insert via
//! the DDB stream Lambda. Computes the matched-user intersection from
//! the saved filters, then fans out per-source aggregations:
//!
//! - **Poll**: for every poll in the space, walk every response by a
//!   matched user and bucket choice indices into `OptionTally`s.
//!   Free-text answers (Short/Subjective) get collected as raw strings
//!   so the panel can list them verbatim.
//! - **Quiz**: for every quiz × matched user, fetch the latest attempt
//!   from the `quiz_user`-keyed GSI (one O(1) hit per pair), bucket
//!   answer indices, and count matches against the quiz's
//!   `correct_answers`.
//! - **Follow**: for every space-defined follow target, query its
//!   followers and intersect with the matched-user set. Top-N targets
//!   by follower-count-among-matched are surfaced; the rest dropped.
//!
//! The result row is written FIRST, then `AnalyzeReport.status` is
//! flipped to `Finish`. If the Lambda crashes mid-write, a retry would
//! find the result already exists; the upsert path overwrites cleanly.

use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use std::collections::HashMap;
use std::collections::HashSet;

const TOP_N_FOLLOW: usize = 30;

pub async fn process_analyze_report(
    cli: &aws_sdk_dynamodb::Client,
    report: &SpaceAnalyzeReport,
) -> Result<()> {
    let space_pk = report.pk.clone();
    let report_id = match &report.sk {
        EntityType::SpaceAnalyzeReport(id) => id.clone(),
        _ => {
            crate::error!("process_analyze_report: unexpected sk on report row");
            return Ok(());
        }
    };

    // 1. Determine matched user_pks. Empty filter list means "all
    //    space participants" — same semantics as the preview endpoint.
    let matched_users: HashSet<String> = if report.filters.is_empty() {
        services::intersection::list_participant_user_pks(cli, &space_pk)
            .await?
            .into_iter()
            .map(|p| p.to_string())
            .collect()
    } else {
        let (set, _) =
            services::intersection::intersect_filters(cli, &space_pk, &report.filters).await?;
        set
    };

    // 2. Run aggregations concurrently — they're independent reads.
    let (poll_aggregates, quiz_aggregates, follow_aggregates) = futures::future::try_join3(
        aggregate_polls(cli, &space_pk, &matched_users),
        aggregate_quizzes(cli, &space_pk, &matched_users),
        aggregate_follows(cli, &space_pk, &matched_users),
    )
    .await?;

    // 3. Persist the result row. Error::Aws from the underlying SDK
    //    converts via `#[from]` so `?` is enough.
    let mut result = SpaceAnalyzeReportResult::new(space_pk.clone(), report_id.clone());
    result.respondent_count = matched_users.len() as i64;
    result.poll_aggregates = poll_aggregates;
    result.quiz_aggregates = quiz_aggregates;
    result.follow_aggregates = follow_aggregates;
    result.create(cli).await?;

    // 4. Flip the parent report's status. Any consumers gated on
    //    `status==Finish` (list-card click, refetch) start working
    //    here. The DynamoEntity-derived updater exposes
    //    `with_<field>` setters for every plain field on the struct.
    SpaceAnalyzeReport::updater(report.pk.clone(), report.sk.clone())
        .with_status(AnalyzeReportStatus::Finish)
        .with_respondent_count(matched_users.len() as i64)
        .with_updated_at(get_now_timestamp_millis())
        .execute(cli)
        .await?;

    Ok(())
}

// ── Poll aggregation ──────────────────────────────────────────────

async fn aggregate_polls(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    matched_users: &HashSet<String>,
) -> Result<Vec<PollQuestionAggregate>> {
    use crate::features::spaces::pages::actions::actions::poll::{
        SpacePoll, SpacePollUserAnswer, SpacePollUserAnswerQueryOption,
    };

    // Pull every poll in the space — same prefix scan list_analyze_polls
    // uses. We need the question definitions to label tallies.
    let polls = list_all_polls(cli, space_pk).await?;
    if polls.is_empty() {
        return Ok(Vec::new());
    }

    let mut aggregates: Vec<PollQuestionAggregate> = Vec::new();

    for poll in polls {
        let poll_id = match &poll.sk {
            EntityType::SpacePoll(id) => id.clone(),
            _ => continue,
        };
        let poll_title = poll.title.clone();

        // Gather all answers for this poll — paginated through the
        // gsi by space_pk + poll_sk.
        let gsi_sk = EntityType::SpacePollUserAnswer(space_pk.to_string(), poll.sk.to_string());
        let mut answers_by_user: HashMap<String, Vec<crate::features::spaces::pages::actions::actions::poll::Answer>> = HashMap::new();
        let mut bookmark: Option<String> = None;
        loop {
            let opt = if let Some(b) = bookmark.clone() {
                SpacePollUserAnswerQueryOption::builder().bookmark(b)
            } else {
                SpacePollUserAnswerQueryOption::builder()
            };
            let (rows, next) = SpacePollUserAnswer::find_by_space_pk(cli, &gsi_sk, opt).await?;
            for row in rows {
                let user_key = row
                    .user_pk
                    .as_ref()
                    .map(|pk| pk.to_string())
                    .unwrap_or_else(|| row.pk.to_string());
                if matched_users.contains(&user_key) {
                    answers_by_user.insert(user_key, row.answers);
                }
            }
            match next {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }

        // Bucket per question.
        for (q_idx, question) in poll.questions.iter().enumerate() {
            let labels = question_option_labels(question);
            let is_text = matches!(
                question,
                crate::features::spaces::pages::actions::actions::poll::Question::ShortAnswer(_)
                    | crate::features::spaces::pages::actions::actions::poll::Question::Subjective(_)
            );

            let mut option_counts: Vec<u32> = vec![0; labels.len()];
            let mut text_answers: Vec<String> = Vec::new();
            let mut respondent_count: u32 = 0;

            for answers in answers_by_user.values() {
                let answer = match answers.get(q_idx) {
                    Some(a) => a,
                    None => continue,
                };

                if is_text {
                    if let Some(text) = answer_text(answer) {
                        if !text.trim().is_empty() {
                            text_answers.push(text);
                            respondent_count += 1;
                        }
                    }
                } else {
                    let indices = answer.to_option_indices();
                    if indices.is_empty() {
                        continue;
                    }
                    respondent_count += 1;
                    for idx in indices {
                        if (idx as usize) < option_counts.len() {
                            option_counts[idx as usize] += 1;
                        }
                    }
                }
            }

            // Skip questions nobody touched.
            if respondent_count == 0 {
                continue;
            }

            aggregates.push(PollQuestionAggregate {
                poll_id: poll_id.clone(),
                poll_title: poll_title.clone(),
                question_idx: q_idx,
                question_title: question.title().to_string(),
                options: labels
                    .into_iter()
                    .zip(option_counts.into_iter())
                    .map(|(label, count)| OptionTally { label, count })
                    .collect(),
                respondent_count,
                text_answers,
            });
        }

        let _ = SpacePoll::default(); // silence unused-import paths
    }

    Ok(aggregates)
}

async fn list_all_polls(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<Vec<crate::features::spaces::pages::actions::actions::poll::SpacePoll>> {
    use crate::features::spaces::pages::actions::actions::poll::SpacePoll;

    let mut bookmark: Option<String> = None;
    let mut polls: Vec<SpacePoll> = Vec::new();
    let prefix = EntityType::SpacePoll(String::default()).to_string();

    loop {
        let mut opt = SpacePoll::opt().sk(prefix.clone()).limit(50);
        if let Some(b) = bookmark.clone() {
            opt = opt.bookmark(b);
        }
        let (rows, next) = SpacePoll::query(cli, space_pk.clone(), opt).await?;
        polls.extend(rows);
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(polls)
}

// ── Quiz aggregation ──────────────────────────────────────────────

async fn aggregate_quizzes(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    matched_users: &HashSet<String>,
) -> Result<Vec<QuizQuestionAggregate>> {
    use crate::features::spaces::pages::actions::actions::quiz::{
        QuizCorrectAnswer, SpaceQuiz, SpaceQuizAnswer, SpaceQuizAttempt,
    };
    use crate::features::spaces::pages::actions::models::SpaceAction;

    let quizzes = list_all_quizzes(cli, space_pk).await?;
    if quizzes.is_empty() {
        return Ok(Vec::new());
    }

    // Quiz title lives on SpaceAction (CompositePartition(space_id, quiz_id)).
    // Batch-get them in one round trip — same pattern as
    // `list_analyze_quizzes`. Fall back to empty title on miss.
    let space_id: SpacePartition = match space_pk {
        Partition::Space(id) => SpacePartition(id.clone()),
        _ => SpacePartition::default(),
    };
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
        SpaceAction::batch_get(cli, action_keys).await.unwrap_or_default()
    };
    let title_by_quiz_id: HashMap<String, String> = actions
        .into_iter()
        .map(|action| (action.pk.1.clone(), action.title))
        .collect();

    let mut aggregates: Vec<QuizQuestionAggregate> = Vec::new();

    for quiz in quizzes {
        let quiz_id_str = match &quiz.sk {
            EntityType::SpaceQuiz(id) => id.clone(),
            _ => continue,
        };
        let quiz_title = title_by_quiz_id
            .get(&quiz_id_str)
            .cloned()
            .unwrap_or_default();
        let quiz_id_st: SpaceQuizEntityType = quiz_id_str.clone().into();

        // Pull correct answers from SpaceQuizAnswer (gated by quiz id).
        // Best-effort: if missing, fall back to empty correct sets.
        let correct_answer_sk = EntityType::SpaceQuizAnswer(quiz_id_str.clone());
        let correct_answers = SpaceQuizAnswer::get(cli, space_pk, Some(correct_answer_sk))
            .await
            .ok()
            .flatten()
            .map(|a| a.answers)
            .unwrap_or_default();

        // For each matched user, fetch their latest attempt for this
        // quiz. Run lookups concurrently.
        let user_partitions: Vec<Partition> = matched_users
            .iter()
            .filter_map(|s| s.parse::<Partition>().ok())
            .collect();

        let lookups = user_partitions.iter().map(|user_pk| {
            let quiz_id = quiz_id_st.clone();
            let user_pk = user_pk.clone();
            async move {
                let attempt =
                    SpaceQuizAttempt::find_latest_by_quiz_user(cli, &quiz_id, &user_pk).await;
                attempt
            }
        });
        let attempt_results = futures::future::join_all(lookups).await;

        // Materialise attempts.
        let mut attempts: Vec<SpaceQuizAttempt> = Vec::new();
        for r in attempt_results {
            if let Ok(Some(attempt)) = r {
                attempts.push(attempt);
            }
        }

        for (q_idx, question) in quiz.questions.iter().enumerate() {
            let labels = question_option_labels(question);
            let is_text = matches!(
                question,
                crate::features::spaces::pages::actions::actions::poll::Question::ShortAnswer(_)
                    | crate::features::spaces::pages::actions::actions::poll::Question::Subjective(_)
            );

            let mut option_counts: Vec<u32> = vec![0; labels.len()];
            let mut text_answers: Vec<String> = Vec::new();
            let mut respondent_count: u32 = 0;
            let mut correct_count: u32 = 0;

            let correct_indices: Vec<u32> = correct_indices_at(&correct_answers, q_idx);

            for attempt in &attempts {
                let answer = match attempt.answers.get(q_idx) {
                    Some(a) => a,
                    None => continue,
                };

                if is_text {
                    if let Some(text) = answer_text(answer) {
                        if !text.trim().is_empty() {
                            text_answers.push(text);
                            respondent_count += 1;
                        }
                    }
                } else {
                    let picked = answer.to_option_indices();
                    if picked.is_empty() {
                        continue;
                    }
                    respondent_count += 1;
                    for idx in &picked {
                        if (*idx as usize) < option_counts.len() {
                            option_counts[*idx as usize] += 1;
                        }
                    }
                    if !correct_indices.is_empty() && picked.iter().any(|i| correct_indices.contains(i)) {
                        correct_count += 1;
                    }
                }
            }

            if respondent_count == 0 {
                continue;
            }

            aggregates.push(QuizQuestionAggregate {
                quiz_id: quiz_id_str.clone(),
                quiz_title: quiz_title.clone(),
                question_idx: q_idx,
                question_title: question.title().to_string(),
                options: labels
                    .into_iter()
                    .zip(option_counts.into_iter())
                    .map(|(label, count)| OptionTally { label, count })
                    .collect(),
                correct_indices,
                correct_count,
                respondent_count,
                text_answers,
            });
        }

        let _ = QuizCorrectAnswer::Single { answer: None };
        let _ = SpaceQuiz::default();
    }

    Ok(aggregates)
}

async fn list_all_quizzes(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<Vec<crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz>> {
    use crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz;

    let mut bookmark: Option<String> = None;
    let mut quizzes: Vec<SpaceQuiz> = Vec::new();
    let prefix = EntityType::SpaceQuiz(String::default()).to_string();

    loop {
        let mut opt = SpaceQuiz::opt().sk(prefix.clone()).limit(50);
        if let Some(b) = bookmark.clone() {
            opt = opt.bookmark(b);
        }
        let (rows, next) = SpaceQuiz::query(cli, space_pk.clone(), opt).await?;
        quizzes.extend(rows);
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(quizzes)
}

fn correct_indices_at(
    answers: &[crate::features::spaces::pages::actions::actions::quiz::QuizCorrectAnswer],
    idx: usize,
) -> Vec<u32> {
    use crate::features::spaces::pages::actions::actions::quiz::QuizCorrectAnswer;
    match answers.get(idx) {
        Some(QuizCorrectAnswer::Single { answer: Some(v) }) => vec![*v as u32],
        Some(QuizCorrectAnswer::Multiple { answers }) => {
            answers.iter().map(|v| *v as u32).collect()
        }
        _ => Vec::new(),
    }
}

// ── Follow aggregation ────────────────────────────────────────────

async fn aggregate_follows(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    matched_users: &HashSet<String>,
) -> Result<Vec<FollowTargetAggregate>> {
    use crate::common::models::auth::UserFollow;
    use crate::features::spaces::pages::actions::actions::follow::SpaceFollowUser;

    // Space-defined follow targets — the same list the cross-filter
    // picker shows.
    let mut bookmark: Option<String> = None;
    let mut targets: Vec<SpaceFollowUser> = Vec::new();
    let target_sk_prefix = EntityType::SpaceSubscriptionUser(String::default()).to_string();
    loop {
        let mut opt = SpaceFollowUser::opt()
            .sk(target_sk_prefix.clone())
            .limit(50);
        if let Some(b) = bookmark.clone() {
            opt = opt.bookmark(b);
        }
        let (rows, next) = SpaceFollowUser::query(cli, space_pk.clone(), opt).await?;
        targets.extend(rows);
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }
    if targets.is_empty() {
        return Ok(Vec::new());
    }

    // For each target, walk its followers and count matched users.
    let mut aggregates: Vec<FollowTargetAggregate> = Vec::new();
    let follower_sk_prefix = EntityType::Follower(String::default()).to_string();
    for target in targets {
        if target.user_pk == Partition::None {
            continue;
        }

        let mut bookmark: Option<String> = None;
        let mut count: u32 = 0;
        loop {
            let mut opt = UserFollow::opt()
                .sk(follower_sk_prefix.clone())
                .limit(100);
            if let Some(b) = bookmark.clone() {
                opt = opt.bookmark(b);
            }
            let (rows, next) =
                UserFollow::query(cli, target.user_pk.clone(), opt).await?;
            for row in rows {
                if matched_users.contains(&row.user_pk.to_string()) {
                    count += 1;
                }
            }
            match next {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }

        if count == 0 {
            continue;
        }

        aggregates.push(FollowTargetAggregate {
            user_pk: target.user_pk.to_string(),
            display_name: target.display_name,
            username: target.username,
            profile_url: target.profile_url,
            count,
        });
    }

    aggregates.sort_by(|a, b| b.count.cmp(&a.count));
    aggregates.truncate(TOP_N_FOLLOW);
    Ok(aggregates)
}

// ── Helpers ───────────────────────────────────────────────────────

fn question_option_labels(
    question: &crate::features::spaces::pages::actions::actions::poll::Question,
) -> Vec<String> {
    use crate::features::spaces::pages::actions::actions::poll::Question;
    match question {
        Question::SingleChoice(q) | Question::MultipleChoice(q) => q.options.clone(),
        Question::Checkbox(q) => q.options.clone(),
        Question::Dropdown(q) => q.options.clone(),
        Question::LinearScale(q) => (q.min_value..=q.max_value)
            .map(|v| v.to_string())
            .collect(),
        Question::ShortAnswer(_) | Question::Subjective(_) => Vec::new(),
    }
}

fn answer_text(
    answer: &crate::features::spaces::pages::actions::actions::poll::Answer,
) -> Option<String> {
    use crate::features::spaces::pages::actions::actions::poll::Answer;
    match answer {
        Answer::ShortAnswer { answer } | Answer::Subjective { answer } => answer.clone(),
        Answer::SingleChoice { other, .. } | Answer::MultipleChoice { other, .. } => {
            other.clone().filter(|s| !s.trim().is_empty())
        }
        _ => None,
    }
}
