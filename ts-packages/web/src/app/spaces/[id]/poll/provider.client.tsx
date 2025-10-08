/* eslint-disable @typescript-eslint/no-explicit-any */
'use client';

import * as XLSX from 'xlsx';
import { createContext, useContext, useEffect, useState } from 'react';
import { useSpaceByIdContext } from '../providers.client';
import { ratelApi, useSpaceById } from '@/lib/api/ratel_api';
import { UserType } from '@/lib/api/models/user';
import { logger } from '@/lib/logger';
import {
  postingSpaceRequest,
  Space,
  SpaceStatus,
  spaceUpdateRequest,
} from '@/lib/api/models/spaces';
import { useNavigate } from 'react-router';
import {
  Answer,
  SurveyResponse,
  surveyResponseCreateRequest,
} from '@/lib/api/models/response';
import { useApiCall } from '@/lib/api/use-send';
import { showErrorToast, showInfoToast, showSuccessToast } from '@/lib/toast';
import { Feed, FileInfo } from '@/lib/api/models/feeds';
import { DiscussionCreateRequest } from '@/lib/api/models/discussion';
import { ElearningCreateRequest } from '@/lib/api/models/elearning';
import { Question, SurveyCreateRequest } from '@/lib/api/models/survey';
import { SpaceDraftCreateRequest } from '@/lib/api/models/space_draft';
import { useQueryClient } from '@tanstack/react-query';
import { QK_GET_SPACE_BY_SPACE_ID } from '@/constants';
import { PollTab, PollTabType } from './types';
import { MappedResponse, Poll, SurveyAnswer } from '../type';
import { useTranslation } from 'react-i18next';
import useFeedById from '@/hooks/feeds/use-feed-by-id';

type ContextType = {
  spaceId: number;
  selectedType: PollTabType;
  isEdit: boolean;
  title: string;
  startedAt: number;
  endedAt: number;
  survey: Poll;
  answers: SurveyResponse[];
  mappedResponses: MappedResponse[];
  answer: SurveyAnswer;

  userType: UserType;
  proposerImage: string;
  proposerName: string;
  createdAt: number;
  status: SpaceStatus;

  handleGoBack: () => void;
  handleDownloadExcel: () => void;
  handleUpdateSelectedType: (type: PollTabType) => void;
  handleUpdateStartDate: (startDate: number) => void;
  handleUpdateEndDate: (endDate: number) => void;
  handleUpdateTitle: (title: string) => void;
  handleUpdateSurvey: (survey: Poll) => void;
  handleLike: () => void;
  handleShare: () => void;
  handleSetAnswers: (answers: Answer[]) => void;
  handleSetStartDate: (startDate: number) => void;
  handleSetEndDate: (endDate: number) => void;
  handleSend: () => Promise<void>;
  handlePostingSpace: () => Promise<void>;
  handleEdit: () => void;
  handleSave: () => Promise<void>;
  handleDelete: () => Promise<void>;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
}: {
  children: React.ReactNode;
}) {
  const { t } = useTranslation('PollSpace');
  const queryClient = useQueryClient();
  const { spaceId } = useSpaceByIdContext();
  const data = useSpaceById(spaceId);
  const space = data.data;

  logger.debug('spaces: ', space);

  const [selectedType, setSelectedType] = useState<PollTabType>(PollTab.POLL);
  const [isEdit, setIsEdit] = useState(false);
  const [title, setTitle] = useState(space.title ?? '');
  const [startedAt, setStartedAt] = useState(
    changeStartedAt(space.started_at ?? Date.now() / 1000),
  );
  const [endedAt, setEndedAt] = useState(
    changeEndedAt(space.ended_at ?? Date.now() / 1000),
  );

  useEffect(() => {
    if (space.started_at) {
      setStartedAt(changeStartedAt(space.started_at));
    }
    if (space.ended_at) {
      setEndedAt(changeEndedAt(space.ended_at));
    }
  }, [space.started_at, space.ended_at]);

  const [survey, setSurvey] = useState<Poll>({
    surveys: space.surveys.map((survey) => ({
      started_at: changeStartedAt(survey.started_at),
      ended_at: changeEndedAt(survey.ended_at),
      questions: survey.questions,
    })),
  });

  const [answers] = useState<SurveyResponse[]>(space.responses ?? []);
  const [answer, setAnswer] = useState<SurveyAnswer>({
    answers:
      space.user_responses.length != 0 ? space.user_responses[0].answers : [],
    is_completed:
      space.user_responses.length !== 0
        ? space.user_responses[0].survey_type === 1
          ? false
          : true
        : false,
  });

  const mappedResponses = mapResponses(
    survey.surveys?.[0]?.questions ?? [],
    space?.responses ?? [],
  );

  const navigate = useNavigate();

  const handleShare = async () => {
    const space_id = space.id;
    navigator.clipboard.writeText(window.location.href).then(async () => {
      try {
        const res = await post(ratelApi.spaces.shareSpace(space_id), {
          share: {},
        });
        if (res) {
          showInfoToast(t('success_share_info'));
          data.refetch();
        }
      } catch (error) {
        logger.error('Failed to share space with error: ', error);
        showErrorToast(t('failed_share_info'));
      }
    });
  };

  const handleLike = async () => {
    const space_id = space.id;
    const value = !space.is_liked;
    try {
      const res = await post(ratelApi.spaces.likeSpace(space_id), {
        like: {
          value,
        },
      });
      if (res) {
        data.refetch();
      }
    } catch (error) {
      logger.error('Failed to like user with error: ', error);
      showErrorToast(t('failed_like_info'));
    }
  };

  const handleGoBack = () => {
    if (isEdit) {
      setIsEdit(false);
      data.refetch();
    } else {
      navigate(-1);
    }
  };

  const handleUpdateTitle = (title: string) => {
    setTitle(title);
  };

  const handleSetStartDate = (startDate: number) => {
    setStartedAt(Math.floor(startDate));
  };

  const handleSetEndDate = (endDate: number) => {
    setEndedAt(Math.floor(endDate));
  };
  const { post } = useApiCall();

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

    try {
      await post(
        ratelApi.responses.respond_answer(spaceId),
        surveyResponseCreateRequest(answers, 1),
      );
      data.refetch();
      queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, spaceId],
      });
      window.location.reload();
      showSuccessToast(t('success_save_response'));
    } catch (err) {
      showErrorToast(t('failed_save_response'));
      logger.error('failed to create response with error: ', err);
    }
  };

  const handlePostingSpace = async () => {
    try {
      await post(
        ratelApi.spaces.getSpaceBySpaceId(spaceId),
        postingSpaceRequest(),
      );
      queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, spaceId],
      });
      window.location.reload();
      data.refetch();

      showSuccessToast(t('success_post_space'));
    } catch (err) {
      showErrorToast(t('failed_post_space'));
      logger.error('failed to posting space with error: ', err);
    }
  };

  const handleEdit = () => {
    setIsEdit(true);
  };

  const handleDownloadExcel = () => {
    const questions = survey?.surveys?.[0]?.questions || [];
    const responses = space?.responses || [];

    const excelRows: any[] = [];

    questions.forEach((question, questionIndex) => {
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
    XLSX.writeFile(workbook, `${space.id}.xlsx`);
  };

  const userType = space.author[0].user_type;
  const proposerImage = space.author[0].profile_url;
  const proposerName = space.author[0].nickname;
  const createdAt = space.created_at;
  const status = space.status;

  logger.debug('startedAt: ', startedAt, 'endedAt: ', endedAt);

  useEffect(() => {
    if (space.user_responses && space.user_responses.length > 0) {
      setAnswer({
        answers: space.user_responses[0].answers,
        is_completed: space.user_responses[0].survey_type === 1 ? false : true,
      });
    }
  }, [space.user_responses]);

  const handleUpdate = async (
    title: string,
    started_at: number,
    ended_at: number,
    html_contents: string,
    files: FileInfo[],
    discussions: DiscussionCreateRequest[],
    elearnings: ElearningCreateRequest[],
    surveys: SurveyCreateRequest[],
    drafts: SpaceDraftCreateRequest[],
  ) => {
    await post(
      ratelApi.spaces.getSpaceBySpaceId(spaceId),
      spaceUpdateRequest(
        html_contents,
        files,
        discussions,
        elearnings,
        surveys,
        drafts,
        title,
        started_at,
        ended_at,
      ),
    );
    queryClient.invalidateQueries({
      queryKey: [QK_GET_SPACE_BY_SPACE_ID, spaceId],
    });
    window.location.reload();
  };

  const handleUpdateSurvey = (survey: Poll) => {
    setSurvey(survey);
  };

  const handleSetAnswers = (answers: Answer[]) => {
    setAnswer((prev) => ({
      ...prev,
      answers,
    }));
  };

  const handleUpdateStartDate = (startDate: number) => {
    setStartedAt(Math.floor(startDate));
  };

  const handleUpdateEndDate = (endDate: number) => {
    setEndedAt(Math.floor(endDate));
  };

  const handleUpdateSelectedType = (type: PollTabType) => {
    setSelectedType(type);
  };

  const handleDelete = async () => {
    try {
      await post(ratelApi.spaces.deleteSpaceV2(spaceId), {
        confirmation: true,
        space_name: space.title,
      });
      showSuccessToast(t('success_delete_space'));
      navigate('/');
    } catch (error) {
      logger.debug('Failed to delete space:', error);
      logger.error('Error deleting space:', error);
      showErrorToast(t('failed_delete_space'));
    }
  };

  const handleSave = async () => {
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

    logger.debug('surveys', survey.surveys);

    const surveys = survey.surveys.map((survey) => ({
      started_at: startedAt,
      ended_at: endedAt,
      questions: survey.questions,
    }));

    try {
      await handleUpdate(
        title,
        startedAt,
        endedAt,
        '',
        [],
        [],
        [],
        surveys,
        [],
      );

      queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, spaceId],
      });
      window.location.reload();
      data.refetch();

      showSuccessToast(t('success_update_space'));
      setIsEdit(false);
    } catch (err) {
      showErrorToast(t('failed_update_space'));
      logger.error('failed to update space with error: ', err);
      setIsEdit(false);
    }
  };

  return (
    <Context.Provider
      value={{
        spaceId,
        selectedType,
        isEdit,
        title,
        startedAt,
        endedAt,
        survey,
        answers,
        answer,
        handleGoBack,
        handleDownloadExcel,
        userType,
        proposerImage,
        proposerName,
        createdAt,
        status,
        mappedResponses,
        handleUpdateSelectedType,
        handleUpdateStartDate,
        handleUpdateEndDate,
        handleUpdateTitle,
        handleUpdateSurvey,
        handleSetAnswers,
        handleSetStartDate,
        handleSetEndDate,
        handleSend,
        handlePostingSpace,
        handleEdit,
        handleSave,
        handleLike,
        handleShare,
        handleDelete,
      }}
    >
      {children}
    </Context.Provider>
  );
}

export function usePollSpaceContext() {
  const context = useContext(Context);
  if (!context)
    throw new Error(
      'Context does not be provided. Please wrap your component with ClientProviders.',
    );
  return context;
}

export function useDeliberationSpace(): Space {
  const { spaceId } = useSpaceByIdContext();
  const { data: space } = useSpaceById(spaceId);

  if (!space) {
    throw new Error('Space data is not available');
  }

  return space;
}

export function useDeliberationFeed(feedId: number): Feed {
  // TODO: Update to use v3 feed API with string IDs
  const { data: feedDetail } = useFeedById(feedId.toString());

  if (!feedDetail) {
    throw new Error('Feed data is not available');
  }

  // TODO: Update to properly type conversion after v3 migration
  return feedDetail as unknown as Feed;
}

function mapResponses(
  questions: Question[],
  responses: SurveyResponse[],
): MappedResponse[] {
  return questions.map((question, index) => {
    const answersForQuestion = responses.map(
      (response) => response.answers[index],
    );

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
