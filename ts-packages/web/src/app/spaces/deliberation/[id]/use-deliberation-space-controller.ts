import * as XLSX from 'xlsx';
import { useEffect, useMemo, useState } from 'react';
import { useNavigate } from 'react-router';
import { useTranslation } from 'react-i18next';
import { useQueryClient } from '@tanstack/react-query';

import useDeliberationSpace from '@/features/deliberation-space/hooks/use-deliberation-space';
import useFeedById from '@/hooks/feeds/use-feed-by-id';

import {
  File,
  SpacePublishState,
  toBackendFile,
  SurveyResponseResponse,
  SpaceVisibility,
} from '@/lib/api/ratel/spaces/deliberation-spaces.v3';

import { ratelApi } from '@/lib/api/ratel_api';
import { NewSurveyCreateRequest, Question } from '@/lib/api/models/survey';
import { PublishingScope } from '@/lib/api/models/notice';
import { useApiCall } from '@/lib/api/use-send';
import { NewDiscussionCreateRequest } from '@/lib/api/models/discussion';
import { NewElearningCreateRequest } from '@/lib/api/models/elearning';
import { QK_GET_DELIBERATION_SPACE_BY_SPACE_ID } from '@/constants';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { checkString } from '@/lib/string-filter-utils';

import {
  Deliberation,
  DeliberationTab,
  DeliberationTabType,
  Thread,
  Poll,
  MappedResponse,
  FinalConsensus,
} from './types';
import { Answer } from '@/lib/api/models/response';
import { SpaceVisibilityValue } from '@/types/space-common';

import { SpaceVisibility as UiVisibility } from '@/types/space-common';
import { SpaceVisibility as ApiVisibility } from '@/lib/api/ratel/spaces/deliberation-spaces.v3';
import { useUpdateDeliberationMutation } from '@/hooks/spaces/deliberation/use-update-deliberation-mutation';
import { useSendDeliberationResponseMutation } from '@/hooks/spaces/deliberation/use-send-response-mutation';

export class DeliberationSpaceController {
  private deps: ReturnType<typeof buildDeps>;
  constructor(deps: ReturnType<typeof buildDeps>) {
    this.deps = deps;
  }

  get threadProps() {
    return {
      t: this.t,
      thread: this.thread,
      setThread: this.setThread,
    };
  }

  get deliberationProps() {
    return {
      t: this.t,
      space: this.space,
      deliberation: this.deliberation,
      setDeliberation: this.setDeliberation,
      handleViewRecord: this.handleViewRecord,
    };
  }

  get deliberationSurveyProps() {
    return {
      t: this.t,
      space: this.space,
      startedAt: this.startedAt,
      endedAt: this.endedAt,
      survey: this.survey,
      answer: this.answer,
      setStartDate: this.setStartDate,
      setEndDate: this.setEndDate,
      setSurvey: this.setSurvey,
      setAnswers: this.setAnswersForSubmit,
      handleSend: this.handleSend,
    };
  }

  get finalConsensusProps() {
    return {
      t: this.t,
      draft: this.draft,
      setDraft: this.setDraft,
    };
  }

  get deliberationAnalyzeProps() {
    return {
      t: this.t,
      answers: this.answers,
      survey: this.survey,
      mappedResponses: this.mappedResponses,
      handleDownloadExcel: this.handleDownloadExcel,
    };
  }

  get spaceSidemenuProps() {
    return {
      t: this.t,
      space: this.space,
      deliberation: this.deliberation,
      selectedType: this.selectedType,
      handleUpdateSelectedType: this.setSelectedType,
      startedAt: this.startedAt,
      endedAt: this.endedAt,
      handleUpdateStartDate: this.setStartDate,
      handleUpdateEndDate: this.setEndDate,
    };
  }

  get t() {
    return this.deps.t;
  }

  get post() {
    return this.deps.post;
  }
  get space() {
    return this.deps.space;
  }

  get selectedType() {
    return this.deps.state.selectedType;
  }
  get isPrivatelyPublished() {
    return this.deps.state.isPrivatelyPublished;
  }
  get isEdit() {
    return this.deps.state.isEdit;
  }
  get title() {
    return this.deps.state.title;
  }
  get startedAt() {
    return this.deps.state.startedAt;
  }
  get endedAt() {
    return this.deps.state.endedAt;
  }
  get thread() {
    return this.deps.state.thread;
  }
  get deliberation() {
    return this.deps.state.deliberation;
  }
  get survey() {
    return this.deps.state.survey;
  }
  get answers() {
    return this.deps.state.answers;
  }
  get mappedResponses() {
    return this.deps.state.mappedResponses;
  }
  get answer() {
    return this.deps.state.answer;
  }
  get draft() {
    return this.deps.state.draft;
  }

  get proposerImage() {
    return this.deps.state.proposerImage;
  }
  get proposerName() {
    return this.deps.state.proposerName;
  }
  get createdAt() {
    return this.deps.state.createdAt;
  }
  get visibility() {
    return this.deps.state.visibility;
  }

  setSelectedType = (type: DeliberationTabType) =>
    this.deps.setters.setSelectedType(type);
  setTitle = (title: string) => this.deps.setters.setTitle(title);
  setThread = (thread: Thread) => this.deps.setters.setThread(thread);
  setDeliberation = (d: Deliberation) => this.deps.setters.setDeliberation(d);
  setSurvey = (s: Poll) => {
    this.deps.setters.setSurvey(s);
  };
  setDraft = (d: FinalConsensus) => this.deps.setters.setDraft(d);
  setAnswersForSubmit = (answers: Answer[]) =>
    this.deps.setters.setAnswersForSubmit(answers);
  setStartDate = (ts: number) => this.deps.setters.setStartDate(ts);
  setEndDate = (ts: number) => this.deps.setters.setEndDate(ts);

  handleGoBack = () => this.deps.handlers.handleGoBack();
  handleDownloadExcel = () => this.deps.handlers.handleDownloadExcel();
  handleViewRecord = (discussionId: string, record: string) =>
    this.deps.handlers.handleViewRecord(discussionId, record);

  handleSend = () => this.deps.handlers.handleSend();
  handlePostingSpace = () => this.deps.handlers.handlePostingSpace();
  handlePublishWithScope = (scope: PublishingScope) =>
    this.deps.handlers.handlePublishWithScope(scope);

  handleEdit = () => this.deps.handlers.handleEdit();
  handleSave = () => this.deps.handlers.handleSave();
  handleDelete = () => this.deps.handlers.handleDelete();

  onSave = async (title: string, html_content: string) => {
    logger.debug('html contents: ', html_content);
    await this.deps.handlers.handleUpdateQuick(
      title,
      this.thread.html_contents,
    );
  };
}

export function useDeliberationSpaceController(
  spacePk: string,
): DeliberationSpaceController {
  const deps = buildDeps(spacePk);
  return new DeliberationSpaceController(deps);
}

function buildDeps(spacePk: string) {
  const mutation = useUpdateDeliberationMutation();
  const resMutation = useSendDeliberationResponseMutation();
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const { post } = useApiCall();
  const { t } = useTranslation('DeliberationSpace');

  const { data: space, refetch } = useDeliberationSpace(spacePk);
  const { data: feed } = useFeedById(space.post_pk);

  const [selectedType, setSelectedType] = useState<DeliberationTabType>(
    DeliberationTab.SUMMARY,
  );
  const [isPrivatelyPublished, setIsPrivatelyPublished] = useState(false);
  const [isEdit, setIsEdit] = useState(false);

  const [title, setTitle] = useState(feed.post.title ?? '');
  const [startedAt, _setStartedAt] = useState(
    changeStartedAt(
      space.surveys.started_at && space.surveys.started_at != 0
        ? space.surveys.started_at
        : Date.now() / 1000,
    ),
  );
  const [endedAt, _setEndedAt] = useState(
    changeEndedAt(
      space.surveys.ended_at && space.surveys.ended_at != 0
        ? space.surveys.ended_at
        : Date.now() / 1000,
    ),
  );

  const proposerImage = space.author_profile_url;
  const proposerName = space.author_username;
  const createdAt = space.created_at;
  const visibility = space.visibility;

  useEffect(() => {
    if (space) {
      setIsPrivatelyPublished(
        space.publish_state !== SpacePublishState.Draft.toUpperCase() &&
          space.visibility == SpaceVisibilityValue.Private,
      );
    }
  }, [space]);

  useEffect(() => {
    if (space.surveys.started_at)
      _setStartedAt(changeStartedAt(space.surveys.started_at));
    if (space.surveys.ended_at)
      _setEndedAt(changeEndedAt(space.surveys.ended_at));
  }, [space.surveys.started_at, space.surveys.ended_at]);

  const [thread, setThread] = useState<Thread>({
    html_contents: space.summary.html_contents ?? '',
    files: space.summary.files ?? [],
  });

  const [deliberation, setDeliberation] = useState<Deliberation>({
    discussions: space.discussions.map((disc) => ({
      started_at: disc.started_at,
      ended_at: disc.ended_at,
      name: disc.name,
      description: disc.description,
      discussion_pk: disc.pk,
      participants: disc.members.map((m) => ({
        user_pk: m.user_pk,
        display_name: m.author_display_name,
        profile_url: m.author_profile_url,
        username: m.author_username,
      })),
    })),
    elearnings: { files: space.elearnings.files },
  });

  const surveyStartedAt = space.surveys.started_at;
  const surveyEndedAt = space.surveys.ended_at;

  const [survey, setSurvey] = useState<Poll>({
    surveys: [
      {
        started_at: changeStartedAt(surveyStartedAt),
        ended_at: changeEndedAt(surveyEndedAt),
        questions: space.surveys.questions,
      },
    ],
  });

  const [answers] = useState<SurveyResponseResponse[]>(
    space.surveys.responses ?? [],
  );

  const [answer, setAnswer] = useState<{
    answers: Answer[];
    is_completed: boolean;
  }>({
    answers:
      space.surveys.user_responses.length != 0
        ? space.surveys.user_responses[0].answers ?? []
        : [],
    is_completed:
      space.surveys.user_responses.length !== 0
        ? space.surveys.user_responses[0].survey_type === 'SURVEY'
        : false,
  });

  const [draft, setDraft] = useState<FinalConsensus>({
    drafts: {
      html_contents: space.recommendation.html_contents ?? '',
      files: space.recommendation.files ?? [],
    },
  });

  const mappedResponses = useMemo(
    () =>
      mapResponses(
        survey.surveys?.[0]?.questions ?? [],
        space?.surveys.responses ?? [],
      ),
    [survey.surveys, space?.surveys.responses],
  );

  const recalSpacePk = useMemo(() => {
    const id = decodeURIComponent(spacePk).replace(/^.*#/, '');
    return 'DELIBERATION_SPACE%23' + id;
  }, [spacePk]);

  // const invalidateAndRefresh = () => {
  //   queryClient.invalidateQueries({
  //     queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, recalSpacePk],
  //   });
  //   window.location.reload();
  //   refetch();
  // };

  const handleUpdate = async (
    titleArg: string,
    started_at: number,
    ended_at: number,
    html_contents: string,
    files: File[],
    discussions: NewDiscussionCreateRequest[],
    elearning_files: NewElearningCreateRequest,
    surveysReq: NewSurveyCreateRequest,
    recommendation_html_contents: string,
    recommendation_files: File[],
    visibilityArg: SpaceVisibility,
  ) => {
    const payloadFiles = files.map(toBackendFile);
    const payloadElearningFiles = elearning_files.files.map(toBackendFile);
    const payloadRecommendationFiles = recommendation_files.map(toBackendFile);

    await mutation.mutateAsync({
      spacePk,
      html_contents,
      files: payloadFiles,
      discussions,
      elearning_files: payloadElearningFiles,
      surveys: [surveysReq],
      recommendation_files: payloadRecommendationFiles,
      visibility: visibilityArg,
      started_at,
      ended_at,
      title: titleArg,
      recommendation_html_contents,
    });

    window.location.reload();
  };

  const handleGoBack = () => {
    if (isEdit) {
      setIsEdit(false);
      refetch();
    } else {
      navigate(-1);
    }
  };

  const handleViewRecord = async (discussionPk: string, record: string) => {
    const response = await fetch(record);
    if (!response.ok) throw new Error(t('failed_download_files'));

    const blob = await response.blob();
    const blobUrl = URL.createObjectURL(blob);

    const a = document.createElement('a');
    a.href = blobUrl;
    a.download = `recording-${discussionPk}.mp4`;
    document.body.appendChild(a);
    a.click();

    document.body.removeChild(a);
    URL.revokeObjectURL(blobUrl);
  };

  const handleDownloadExcel = () => {
    const questions = survey?.surveys?.[0]?.questions || [];
    const responses = space?.surveys.responses || [];

    const excelRows: any[] = [];

    questions.forEach((question, questionIndex) => {
      const row: any = { Index: questionIndex + 1, Question: question.title };

      responses.forEach((response, responseIndex) => {
        const rawAnswer = response.answers?.[questionIndex]?.answer;

        let parsedAnswer;
        if (typeof rawAnswer === 'string') parsedAnswer = rawAnswer;
        else if (typeof rawAnswer === 'number') parsedAnswer = rawAnswer + 1;
        else if (Array.isArray(rawAnswer))
          parsedAnswer = rawAnswer.map((v) => Number(v) + 1).join(', ');
        else
          parsedAnswer =
            question.answer_type === 'short_answer' ||
            question.answer_type === 'subjective'
              ? ''
              : 0;

        row[`Response ${responseIndex + 1}`] = parsedAnswer;
      });

      excelRows.push(row);
    });

    const worksheet = XLSX.utils.json_to_sheet(excelRows);
    worksheet['!cols'] = Object.keys(excelRows[0]).map((key) => {
      const maxLen = Math.max(
        key.length,
        ...excelRows.map((row) => String(row[key] ?? '').length),
      );
      return { wch: maxLen + 2 };
    });

    const workbook = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(workbook, worksheet, 'Survey Responses');
    XLSX.writeFile(workbook, `${space.pk}.xlsx`);
  };

  const handleSend = async () => {
    const questions =
      survey.surveys.length != 0 ? survey.surveys[0].questions : [];
    let a = answer.answers;

    let isCheck = true;
    a = [...a, ...Array(questions.length - a.length).fill(undefined)];

    for (let i = 0; i < questions.length; i++) {
      const required = questions[i].is_required;
      const ans = a[i];
      if (!required) {
        if (!ans)
          a[i] = { answer_type: questions[i].answer_type, answer: null };
        continue;
      }
      if (!ans) {
        isCheck = false;
        break;
      }
    }

    if (!isCheck) {
      showErrorToast(t('all_input_required'));
      return;
    }

    try {
      await resMutation.mutateAsync({
        spacePk: recalSpacePk,
        survey_pk: space.surveys.pk,
        answers: a,
      });
      queryClient.invalidateQueries({
        queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, recalSpacePk],
      });
      window.location.reload();
      showSuccessToast(t('success_save_response'));
    } catch (err) {
      logger.error('failed to create response with error: ', err);
    }
  };

  const handlePublishWithScope = async (scope: PublishingScope) => {
    if (!space) return;
    try {
      await post(
        ratelApi.spaces.postingDeliberationSpaceBySpaceId(recalSpacePk),
        {
          visibility: scope === PublishingScope.Private ? 'PRIVATE' : 'PUBLIC',
        },
      );

      const discussions = deliberation.discussions.map((disc) => ({
        discussion_pk: disc.discussion_pk,
        started_at: disc.started_at,
        ended_at: disc.ended_at,
        name: disc.name,
        description: disc.description,
        user_ids: disc.participants.map((v) => v.user_pk),
      }));

      const surveysReq: NewSurveyCreateRequest = {
        survey_pk: space.surveys.pk,
        started_at: survey.surveys[0]?.started_at ?? startedAt,
        ended_at: survey.surveys[0]?.ended_at ?? endedAt,
        status: 'READY',
        questions: survey.surveys.flatMap((s) => s.questions),
      };

      const elearningFiles: NewElearningCreateRequest = {
        files: deliberation.elearnings.files,
      };

      await handleUpdate(
        title,
        startedAt,
        endedAt,
        thread.html_contents,
        thread.files,
        discussions,
        elearningFiles,
        surveysReq,
        draft.drafts.html_contents,
        draft.drafts.files,
        scope === PublishingScope.Private ? 'PRIVATE' : 'PUBLIC',
      );

      queryClient.invalidateQueries({
        queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, recalSpacePk],
      });
      refetch();

      showSuccessToast(
        scope === PublishingScope.Public
          ? t('success_publish_space_public')
          : t('success_publish_space_private'),
      );
    } catch (e) {
      showErrorToast(t('failed_publish_space'));
      logger.error(e);
    }
  };

  const handlePostingSpace = async () => {
    try {
      await post(
        ratelApi.spaces.postingDeliberationSpaceBySpaceId(recalSpacePk),
        {},
      );
      queryClient.invalidateQueries({
        queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, recalSpacePk],
      });
      window.location.reload();
      refetch();
      showSuccessToast(t('success_post_space'));
    } catch (err) {
      showErrorToast(t('failed_post_space'));
      logger.error('failed to posting space with error: ', err);
    }
  };

  const handleEdit = () => setIsEdit(true);

  const handleSave = async () => {
    if (checkString(title) || checkString(thread.html_contents)) {
      showErrorToast(t('remove_test_keyword'));
      setIsEdit(false);
      return;
    }

    for (let i = 0; i < survey.surveys.length; i++) {
      const question = survey.surveys[i].questions;
      for (let j = 0; j < question.length; j++) {
        const q = question[j];
        if (q.title === '') {
          showErrorToast(t('question_title_required'));
          return;
        }
        if (q.answer_type === 'checkbox' || q.answer_type === 'dropdown') {
          if (q.options.length < 2) {
            showErrorToast(t('more_option_required'));
            return;
          }
        }
        if (q.answer_type === 'linear_scale') {
          if (q.max_label === '' || q.min_label === '') {
            showErrorToast(t('fill_label_required'));
            return;
          }
        }
      }
    }

    const discussions = deliberation.discussions.map((disc) => ({
      discussion_pk: disc.discussion_pk,
      started_at: disc.started_at,
      ended_at: disc.ended_at,
      name: disc.name,
      description: disc.description,
      user_ids: disc.participants.map((v) => v.user_pk),
    }));

    const surveysReq: NewSurveyCreateRequest = {
      survey_pk: space.surveys.pk,
      started_at: startedAt,
      ended_at: endedAt,
      status: 'READY',
      questions: survey.surveys.flatMap((s) => s.questions),
    };

    const elearningFiles: NewElearningCreateRequest = {
      files: deliberation.elearnings.files,
    };

    try {
      await handleUpdate(
        title,
        startedAt,
        endedAt,
        thread.html_contents,
        thread.files,
        discussions,
        elearningFiles,
        surveysReq,
        draft.drafts.html_contents,
        draft.drafts.files,
        toApiVisibility(space.visibility),
      );

      queryClient.invalidateQueries({
        queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, recalSpacePk],
      });
      window.location.reload();
      refetch();

      showSuccessToast(t('success_update_space'));
      setIsEdit(false);
    } catch (err) {
      showErrorToast(t('failed_update_space'));
      logger.error('failed to update space with error: ', err);
      setIsEdit(false);
    }
  };

  const handleUpdateQuick = async (
    overrideTitle: string,
    overrideHtml: string,
  ) => {
    const discussions = deliberation.discussions.map((disc) => ({
      discussion_pk: disc.discussion_pk,
      started_at: disc.started_at,
      ended_at: disc.ended_at,
      name: disc.name,
      description: disc.description,
      user_ids: disc.participants.map((v) => v.user_pk),
    }));

    const surveysReq: NewSurveyCreateRequest = {
      survey_pk: space.surveys.pk,
      started_at: startedAt,
      ended_at: endedAt,
      status: 'READY',
      questions: survey.surveys.flatMap((s) => s.questions),
    };

    const elearningFiles: NewElearningCreateRequest = {
      files: deliberation.elearnings.files,
    };

    await handleUpdate(
      overrideTitle,
      startedAt,
      endedAt,
      overrideHtml,
      thread.files,
      discussions,
      elearningFiles,
      surveysReq,
      draft.drafts.html_contents,
      draft.drafts.files,
      toApiVisibility(space.visibility),
    );

    queryClient.invalidateQueries({
      queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, recalSpacePk],
    });
    window.location.reload();
    refetch();
  };

  const handleDelete = async () => {
    try {
      await post(
        ratelApi.spaces.deleteDeliberationSpaceBySpaceId(recalSpacePk),
        {},
      );
      showSuccessToast(t('success_delete_space'));
      navigate('/');
    } catch (error) {
      logger.debug('Failed to delete space:', error);
      logger.error('Error deleting space:', error);
      showErrorToast(t('failed_delete_space'));
    }
  };

  const setters = {
    setSelectedType,
    setTitle,
    setThread,
    setDeliberation,
    setSurvey,
    setDraft,
    setAnswersForSubmit: (answers: Answer[]) =>
      setAnswer((prev) => ({ ...prev, answers })),
    setStartDate: (ts: number) => _setStartedAt(Math.floor(ts)),
    setEndDate: (ts: number) => _setEndedAt(Math.floor(ts)),
  };

  const state = {
    selectedType,
    isPrivatelyPublished,
    isEdit,
    title,
    startedAt,
    endedAt,
    thread,
    deliberation,
    survey,
    answers,
    mappedResponses,
    answer,
    draft,
    proposerImage,
    proposerName,
    createdAt,
    visibility,
  };

  const handlers = {
    handleGoBack,
    handleDownloadExcel,
    handleViewRecord,
    handleSend,
    handlePostingSpace,
    handlePublishWithScope,
    handleEdit,
    handleSave,
    handleDelete,
    handleUpdateQuick,
  };

  const token = [
    space.pk,
    selectedType,
    isPrivatelyPublished,
    isEdit,
    title,
    startedAt,
    endedAt,
    thread.html_contents,
    thread.files.length,
    deliberation.discussions.length,
    deliberation.elearnings.files.length,
    survey.surveys.length,
    survey.surveys[0]?.questions.length ?? 0,
    answers.length,
    draft.drafts.html_contents,
    draft.drafts.files.length,
  ].join('|');

  return {
    post: feed,
    space,
    setters,
    state,
    handlers,
    token,
    t,
  };
}

const toApiVisibility = (v: UiVisibility): ApiVisibility => {
  switch (v.type) {
    case 'Private':
      return 'PRIVATE';
    case 'Public':
      return 'PUBLIC';
    // case 'Team':
    //   return `TEAM:${v.team_pk}`;
  }
};
function mapResponses(
  questions: Question[],
  responses: SurveyResponseResponse[],
): MappedResponse[] {
  return questions.map((question, index) => {
    const answersForQuestion = (responses ?? [])
      .map((r) => r.answers?.[index])
      .filter((a): a is Answer => a !== undefined);

    return { question, answers: answersForQuestion };
  });
}
function changeStartedAt(timestamp: number) {
  const date = new Date(timestamp * 1000);
  return Math.floor(date.getTime() / 1000);
}
function changeEndedAt(timestamp: number) {
  const date = new Date(timestamp * 1000);
  return Math.floor(date.getTime() / 1000);
}
