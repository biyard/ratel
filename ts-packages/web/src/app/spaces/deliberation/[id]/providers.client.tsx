'use client';

import * as XLSX from 'xlsx';
import {
  File,
  responseCreateRequest,
} from '@/lib/api/models/spaces/deliberation-spaces';
import { ratelApi, useDeliberationSpaceById } from '@/lib/api/ratel_api';
import { useTranslations } from 'next-intl';
import React, { createContext, useContext, useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
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
import {
  SurveyResponseResponse,
  updateSpaceRequest,
  Visibility,
} from '@/lib/api/models/spaces/deliberation-spaces';
import { SurveyAnswer } from '../../[id]/type';
import { NewSurveyCreateRequest, Question } from '@/lib/api/models/survey';
import { PublishingScope } from '@/lib/api/models/notice';
import { useApiCall } from '@/lib/api/use-send';
import { NewDiscussionCreateRequest } from '@/lib/api/models/discussion';
import { NewElearningCreateRequest } from '@/lib/api/models/elearning';
import { useQueryClient } from '@tanstack/react-query';
import { QK_GET_DELIBERATION_SPACE_BY_SPACE_ID } from '@/constants';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { checkString } from '@/lib/string-filter-utils';

type ContextType = {
  spaceId: string;
  selectedType: DeliberationTabType;
  isPrivatelyPublished: boolean;
  isEdit: boolean;
  title: string;
  startedAt: number;
  endedAt: number;
  thread: Thread;
  deliberation: Deliberation;
  survey: Poll;
  answers: SurveyResponseResponse[];
  mappedResponses: MappedResponse[];
  answer: SurveyAnswer;
  draft: FinalConsensus;

  proposerImage: string;
  proposerName: string;
  createdAt: number;
  visibility: Visibility;

  handleGoBack: () => void;
  handleDownloadExcel: () => void;
  handleViewRecord: (discussionId: string, record: string) => Promise<void>;
  handleUpdateSelectedType: (type: DeliberationTabType) => void;
  handleUpdateStartDate: (startDate: number) => void;
  handleUpdateEndDate: (endDate: number) => void;
  handleUpdateTitle: (title: string) => void;
  handleUpdateThread: (thread: Thread) => void;
  handleUpdateDeliberation: (deliberation: Deliberation) => void;
  handleUpdateSurvey: (survey: Poll) => void;
  handleUpdateDraft: (draft: FinalConsensus) => void;
  handleSetAnswers: (answers: Answer[]) => void;
  handleSetStartDate: (startDate: number) => void;
  handleSetEndDate: (endDate: number) => void;
  handleSend: () => Promise<void>;
  handlePostingSpace: () => Promise<void>;
  handleEdit: () => void;
  handleSave: () => Promise<void>;
  handleDelete: () => Promise<void>;
  handlePublishWithScope: (scope: PublishingScope) => Promise<void>;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
  spaceId,
}: {
  children: React.ReactNode;
  spaceId: string;
}) {
  const queryClient = useQueryClient();
  const router = useRouter();
  const { post } = useApiCall();
  const t = useTranslations('DeliberationSpace');

  const { data: space, refetch } = useDeliberationSpaceById(spaceId);

  const [selectedType, setSelectedType] = useState<DeliberationTabType>(
    DeliberationTab.SUMMARY,
  );
  const [isPrivatelyPublished] = useState<boolean>(false);
  const [isEdit, setIsEdit] = useState(false);
  const [title, setTitle] = useState(space.title ?? '');
  const [startedAt, setStartedAt] = useState(
    changeStartedAt(
      space.surveys.started_at && space.surveys.started_at != 0
        ? space.surveys.started_at
        : Date.now() / 1000,
    ),
  );
  const [endedAt, setEndedAt] = useState(
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

  //FIXME: fix to publish when api is implemented
  //   useEffect(() => {
  //     if (space) {
  //       setIsPrivatelyPublished(
  //         // space.status !== SpaceStatus.Draft &&
  //         //   space.publishing_scope === PublishingScope.Private,
  //         false,
  //       );
  //     }

  //     console.log('deliberation space: ', space);
  //   }, [space]);

  useEffect(() => {
    if (space.surveys.started_at) {
      setStartedAt(changeStartedAt(space.surveys.started_at));
    }
    if (space.surveys.ended_at) {
      setEndedAt(changeEndedAt(space.surveys.ended_at));
    }
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
      participants: disc.members.map((member) => ({
        user_pk: member.user_pk,
        display_name: member.author_display_name,
        profile_url: member.author_profile_url,
        username: member.author_username,
      })),
    })),
    elearnings: [{ files: space.elearnings.files }],
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

  const [answer, setAnswer] = useState<SurveyAnswer>({
    answers:
      space.surveys.user_responses.length != 0
        ? (space.surveys.user_responses[0].answers ?? [])
        : [],
    is_completed:
      space.surveys.user_responses.length !== 0
        ? space.surveys.user_responses[0].survey_type === 1
          ? false
          : true
        : false,
  });

  const [draft, setDraft] = useState<FinalConsensus>({
    drafts: {
      html_contents: space.recommendation.html_contents ?? '',
      files: space.recommendation.files ?? [],
    },
  });

  const mappedResponses = mapResponses(
    survey.surveys?.[0]?.questions ?? [],
    space?.surveys.responses ?? [],
  );

  const handleEdit = () => {
    setIsEdit(true);
  };

  const handleUpdateTitle = (title: string) => {
    setTitle(title);
  };

  const handleUpdateThread = (thread: Thread) => {
    setThread(thread);
  };

  const handleUpdateDeliberation = (deliberation: Deliberation) => {
    setDeliberation(deliberation);
  };

  const handleUpdateSurvey = (survey: Poll) => {
    setSurvey(survey);
  };

  const handleUpdateDraft = (draft: FinalConsensus) => {
    setDraft(draft);
  };

  const handleUpdateStartDate = (startDate: number) => {
    setStartedAt(Math.floor(startDate));
  };

  const handleUpdateEndDate = (endDate: number) => {
    setEndedAt(Math.floor(endDate));
  };

  const handleUpdateSelectedType = (type: DeliberationTabType) => {
    setSelectedType(type);
  };

  const handleSetAnswers = (answers: Answer[]) => {
    setAnswer((prev) => ({
      ...prev,
      answers,
    }));
  };

  const handleSetStartDate = (startDate: number) => {
    setStartedAt(Math.floor(startDate));
  };

  const handleSetEndDate = (endDate: number) => {
    setEndedAt(Math.floor(endDate));
  };

  const handleSend = async () => {
    const questions =
      survey.surveys.length != 0 ? survey.surveys[0].questions : [];

    let answers = answer.answers;

    let isCheck = true;

    answers = [
      ...answers,
      ...Array(questions.length - answers.length).fill(undefined),
    ];

    for (let i = 0; i < questions.length; i++) {
      const required = questions[i].is_required;
      const ans = answers[i];

      if (!required) {
        if (!ans) {
          answers[i] = {
            answer_type: questions[i].answer_type,
            answer: null,
          };
        }
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

    const spacePk = 'DELIBERATION_SPACE%23' + spaceId;

    try {
      await post(
        ratelApi.responses.deliberation_response_answer(spacePk),
        responseCreateRequest(space.surveys.pk, answers),
      );
      refetch();
      queryClient.invalidateQueries({
        queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, spacePk],
      });
      router.refresh();
      showSuccessToast(t('success_save_response'));
    } catch (err) {
      showErrorToast(t('failed_save_response'));
      logger.error('failed to create response with error: ', err);
    }
  };

  const handlePublishWithScope = async (scope: PublishingScope) => {
    if (!space) return;
    try {
      const spacePk = 'DELIBERATION_SPACE%23' + spaceId;

      await post(
        ratelApi.spaces.postingDeliberationSpaceBySpaceId(spacePk),
        {},
      );

      const discussions = deliberation.discussions.map((disc) => ({
        discussion_pk: disc.discussion_pk,
        started_at: disc.started_at,
        ended_at: disc.ended_at,

        name: disc.name,
        description: disc.description,
        user_ids: disc.participants.map((v) => v.user_pk),
      }));

      const surveys: NewSurveyCreateRequest = {
        survey_pk: space.surveys.pk,
        started_at: survey.surveys[0]?.started_at ?? startedAt,
        ended_at: survey.surveys[0]?.ended_at ?? endedAt,
        status: 1,
        questions: survey.surveys.flatMap((s) => s.questions),
      };

      const elearningFiles: NewElearningCreateRequest = {
        files: deliberation.elearnings.flatMap((e) => e.files),
      };

      await handleUpdate(
        title,
        startedAt,
        endedAt,
        thread.html_contents,
        thread.files,
        discussions,
        elearningFiles,
        surveys,
        draft.drafts.html_contents,
        draft.drafts.files,
        'Public',
      );

      queryClient.invalidateQueries({
        queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, spacePk],
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

  const handleUpdate = async (
    title: string,
    started_at: number,
    ended_at: number,
    html_contents: string,
    files: File[],
    discussions: NewDiscussionCreateRequest[],
    elearning_files: NewElearningCreateRequest,
    surveys: NewSurveyCreateRequest,
    recommendation_html_contents: string,
    recommendation_files: File[],
    visibility: Visibility,
  ) => {
    const spacePk = 'DELIBERATION_SPACE%23' + spaceId;

    await post(
      ratelApi.spaces.updateDeliberationSpaceBySpaceId(spacePk),
      updateSpaceRequest(
        html_contents,
        files,

        discussions,
        elearning_files.files,

        [surveys],

        recommendation_files,

        visibility,
        started_at,
        ended_at,

        title,
        recommendation_html_contents,
      ),
    );

    queryClient.invalidateQueries({
      queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, spacePk],
    });
    router.refresh();
  };

  const handleGoBack = () => {
    if (isEdit) {
      setIsEdit(false);
      refetch();
    } else {
      router.back();
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

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const excelRows: any[] = [];

    questions.forEach((question, questionIndex) => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const row: any = {
        Index: questionIndex + 1,
        Question: question.title,
      };

      responses.forEach((response, responseIndex) => {
        const rawAnswer = response.answers?.[questionIndex]?.answer;

        let parsedAnswer;

        if (typeof rawAnswer === 'string') {
          parsedAnswer = rawAnswer;
        } else if (typeof rawAnswer === 'number') {
          parsedAnswer = rawAnswer + 1;
        } else if (Array.isArray(rawAnswer)) {
          parsedAnswer = rawAnswer.map((v) => Number(v) + 1).join(', ');
        } else {
          parsedAnswer =
            question.answer_type === 'short_answer' ||
            question.answer_type === 'subjective'
              ? ''
              : 0;
        }

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

  const handlePostingSpace = async () => {
    try {
      const spacePk = 'DELIBERATION_SPACE%23' + spaceId;

      await post(
        ratelApi.spaces.postingDeliberationSpaceBySpaceId(spacePk),
        {},
      );
      queryClient.invalidateQueries({
        queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, spacePk],
      });
      router.refresh();
      refetch();

      showSuccessToast(t('success_post_space'));
    } catch (err) {
      showErrorToast(t('failed_post_space'));
      logger.error('failed to posting space with error: ', err);
    }
  };

  const handleSave = async () => {
    const spacePk = 'DELIBERATION_SPACE%23' + spaceId;

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

    logger.debug('discussions: ', discussions);
    logger.debug('surveys', survey.surveys);

    const surveys: NewSurveyCreateRequest = {
      survey_pk: space.surveys.pk,
      started_at: survey.surveys[0]?.started_at ?? startedAt,
      ended_at: survey.surveys[0]?.ended_at ?? endedAt,
      status: 1,
      questions: survey.surveys.flatMap((s) => s.questions),
    };

    const elearningFiles: NewElearningCreateRequest = {
      files: deliberation.elearnings.flatMap((e) => e.files),
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
        surveys,
        draft.drafts.html_contents,
        draft.drafts.files,
        'Public',
      );

      queryClient.invalidateQueries({
        queryKey: [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, spacePk],
      });
      router.refresh();
      refetch();

      showSuccessToast(t('success_update_space'));
      setIsEdit(false);
    } catch (err) {
      showErrorToast(t('failed_update_space'));
      logger.error('failed to update space with error: ', err);
      setIsEdit(false);
    }
  };

  const handleDelete = async () => {
    const spacePk = 'DELIBERATION_SPACE%23' + spaceId;

    try {
      await post(ratelApi.spaces.deleteDeliberationSpaceBySpaceId(spacePk), {});
      showSuccessToast(t('success_delete_space'));
      router.push('/');
    } catch (error) {
      logger.debug('Failed to delete space:', error);
      logger.error('Error deleting space:', error);
      showErrorToast(t('failed_delete_space'));
    }
  };

  return (
    <Context.Provider
      value={{
        spaceId,
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

        handleGoBack,
        handleDownloadExcel,
        handleViewRecord,
        handleUpdateSelectedType,
        handleUpdateStartDate,
        handleUpdateEndDate,
        handleUpdateDeliberation,
        handleUpdateThread,
        handleUpdateTitle,
        handleUpdateDraft,
        handleUpdateSurvey,
        handleSetAnswers,
        handleSetEndDate,
        handleSetStartDate,
        handleSend,
        handlePostingSpace,
        handlePublishWithScope,
        handleEdit,
        handleSave,
        handleDelete,
      }}
    >
      {children}
    </Context.Provider>
  );
}

export function useDeliberationSpaceByIdContext() {
  const context = useContext(Context);

  if (!context)
    throw new Error(
      'Context has not been provided. Please wrap your component with ClientProviders.',
    );

  return context;
}

function mapResponses(
  questions: Question[],
  responses: SurveyResponseResponse[],
): MappedResponse[] {
  return questions.map((question, index) => {
    const answersForQuestion = (responses ?? [])
      .map((r) => r.answers?.[index])
      .filter((a): a is Answer => a !== undefined);

    return {
      question,
      answers: answersForQuestion,
    };
  });
}

function changeStartedAt(timestamp: number) {
  const date = new Date(timestamp * 1000);
  // date.setUTCHours(0, 0, 0, 0);
  const newDate = Math.floor(date.getTime() / 1000);
  return newDate;
}

function changeEndedAt(timestamp: number) {
  const date = new Date(timestamp * 1000);
  // date.setUTCHours(23, 59, 59, 0);
  const newDate = Math.floor(date.getTime() / 1000);
  return newDate;
}
