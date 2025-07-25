/* eslint-disable @typescript-eslint/no-explicit-any */
'use client';

import * as XLSX from 'xlsx';
import React, { createContext, useContext, useEffect, useState } from 'react';
import { useSpaceByIdContext } from '../providers.client';
import { ratelApi, useSpaceById } from '@/lib/api/ratel_api';
import {
  Deliberation,
  DeliberationTab,
  DeliberationTabType,
  FinalConsensus,
  Poll,
  SurveyAnswer,
  Thread,
} from './types';
import { UserType } from '@/lib/api/models/user';
import { StateSetter } from '@/types';
import { logger } from '@/lib/logger';
import {
  postingSpaceRequest,
  Space,
  SpaceStatus,
  spaceUpdateRequest,
} from '@/lib/api/models/spaces';
import { useRouter } from 'next/navigation';
import {
  Answer,
  SurveyResponse,
  surveyResponseCreateRequest,
} from '@/lib/api/models/response';
import { useApiCall } from '@/lib/api/use-send';
import { showErrorToast, showInfoToast, showSuccessToast } from '@/lib/toast';
import { checkString } from '@/lib/string-filter-utils';
import { Feed, FileInfo } from '@/lib/api/models/feeds';
import { DiscussionCreateRequest } from '@/lib/api/models/discussion';
import { ElearningCreateRequest } from '@/lib/api/models/elearning';
import { Question, SurveyCreateRequest } from '@/lib/api/models/survey';
import { SpaceDraftCreateRequest } from '@/lib/api/models/space_draft';
import { useQueryClient } from '@tanstack/react-query';
import { QK_GET_SPACE_BY_SPACE_ID } from '@/constants';
import { useFeedByID } from '@/app/(social)/_hooks/feed';

import { usePopup } from '@/lib/contexts/popup-service';
import DropdownMenu from './_components/dropdown/drop-down-menu';

export interface MappedResponse {
  question: Question;
  answers: Answer[];
}

type ContextType = {
  spaceId: number;
  selectedType: DeliberationTabType;
  setSelectedType: StateSetter<DeliberationTabType>;
  isEdit: boolean;
  setIsEdit: StateSetter<boolean>;
  title: string;
  setTitle: StateSetter<string>;
  startedAt: number;
  setStartedAt: StateSetter<number>;
  endedAt: number;
  setEndedAt: StateSetter<number>;
  thread: Thread;
  setThread: StateSetter<Thread>;
  deliberation: Deliberation;
  setDeliberation: StateSetter<Deliberation>;
  survey: Poll;
  setSurvey: StateSetter<Poll>;
  answers: SurveyResponse[];
  mappedResponses: MappedResponse[];
  answer: SurveyAnswer;
  setAnswer: StateSetter<SurveyAnswer>;
  draft: FinalConsensus;
  setDraft: StateSetter<FinalConsensus>;
  handleGoBack: () => void;
  handleDownloadExcel: () => void;
  handleViewRecord: (discussionId: number, record: string) => Promise<void>;

  userType: UserType;
  proposerImage: string;
  proposerName: string;
  createdAt: number;
  status: SpaceStatus;

  handleLike: () => void;
  handleShare: () => void;
  handleSetAnswers: (answers: Answer[]) => void;
  handleSetStartDate: (startDate: number) => void;
  handleSetEndDate: (endDate: number) => void;
  handleSend: () => Promise<void>;
  handlePostingSpace: () => Promise<void>;
  handleEdit: () => void;
  handleSave: () => Promise<void>;
  handleDelete: () => Promise<void>
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
}: {
  children: React.ReactNode;
}) {
  const queryClient = useQueryClient();
  const { spaceId } = useSpaceByIdContext();
  const data = useSpaceById(spaceId);
  const space = data.data;
  const {popup} = usePopup()

  logger.debug('spaces: ', space);

  const [selectedType, setSelectedType] = useState<DeliberationTabType>(
    DeliberationTab.SUMMARY,
  );
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

  const [thread, setThread] = useState<Thread>({
    html_contents: space.html_contents ?? '',
    files: space.files ?? [],
  });
  const [deliberation, setDeliberation] = useState<Deliberation>({
    discussions: space.discussions.map((disc) => ({
      started_at: disc.started_at,
      ended_at: disc.ended_at,
      name: disc.name,
      description: disc.description,
      discussion_id: disc.id,
      participants: disc.members.map((member) => ({
        id: member.id,
        created_at: member.created_at,
        updated_at: member.updated_at,
        nickname: member.nickname,
        username: member.username,
        email: member.email,
        profile_url: member.profile_url ?? '',
        user_type: UserType.Individual,
      })),
    })),

    elearnings: space.elearnings.map((elearning) => ({
      files: elearning.files,
    })),
  });
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
    is_completed: space.user_responses.length != 0,
  });

  const [draft, setDraft] = useState<FinalConsensus>({
    drafts: space.drafts.map((draft) => ({
      title: draft.title,
      html_contents: draft.html_contents,
      files: draft.files,
    })),
  });

  const mappedResponses = mapResponses(
    survey.surveys?.[0]?.questions ?? [],
    space?.responses ?? [],
  );

  const router = useRouter();

  const handleShare = async () => {
    const space_id = space.id;
    navigator.clipboard.writeText(window.location.href).then(async () => {
      try {
        const res = await post(ratelApi.spaces.shareSpace(space_id), {
          share: {},
        });
        if (res) {
          showInfoToast('The space URL has been copied to your clipboard.');
          data.refetch();
        }
      } catch (error) {
        logger.error('Failed to share space with error: ', error);
        showErrorToast(
          'Unable to share space at this time. Please try again later.',
        );
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
      showErrorToast(
        'Unable to register your like at this time. Please try again later.',
      );
    }
  };

  const handleGoBack = () => {
    if (isEdit) {
      setIsEdit(false);
      data.refetch();
    } else {
      router.back();
    }
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
  const { post } = useApiCall();

  const handleSend = async () => {
    try {
      await post(
        ratelApi.responses.respond_answer(spaceId),
        surveyResponseCreateRequest(answer.answers),
      );
      data.refetch();
      queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, spaceId],
      });
      router.refresh();
      showSuccessToast('Your response has been saved successfully.');
    } catch (err) {
      showErrorToast(
        'There was a problem saving your response. Please try again later.',
      );
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
      router.refresh();
      data.refetch();

      showSuccessToast('Your space has been posted successfully.');
    } catch (err) {
      showErrorToast('Failed to post the space. Please try again.');
      logger.error('failed to posting space with error: ', err);
    }
  };

  const handleEdit = () => {
    setIsEdit(true);
  };

  const handleViewRecord = async (discussionId: number, record: string) => {
    const response = await fetch(record);
    if (!response.ok) throw new Error('failed to download files');

    const blob = await response.blob();
    const blobUrl = URL.createObjectURL(blob);

    const a = document.createElement('a');
    a.href = blobUrl;
    a.download = `recording-${discussionId}.mp4`;
    document.body.appendChild(a);
    a.click();

    document.body.removeChild(a);
    URL.revokeObjectURL(blobUrl);
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
  logger.debug('deliberation: ', deliberation);

  useEffect(() => {
    if (space.user_responses && space.user_responses.length > 0) {
      setAnswer({
        answers: space.user_responses[0].answers,
        is_completed: true,
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
    router.refresh();
  };

  const handleDelete = async () => {
    try {

      showSuccessToast("Space deleted successful")
      
    } catch (error) {


      
    }
  }

  const handleSave = async () => {
    if (checkString(title) || checkString(thread.html_contents)) {
      showErrorToast('Please remove any test-related keywords before saving.');
      setIsEdit(false);
      return;
    }

    const discussions = deliberation.discussions.map((disc) => ({
      started_at: disc.started_at,
      ended_at: disc.ended_at,
      name: disc.name,
      description: disc.description,
      participants: disc.participants.map((member) => member.id),
      discussion_id: disc.discussion_id,
    }));

    logger.debug('discussions: ', discussions);
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
        thread.html_contents,
        thread.files,
        discussions,
        deliberation.elearnings,
        surveys,
        draft.drafts,
      );

      queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, spaceId],
      });
      router.refresh();
      data.refetch();

      showSuccessToast('Space has been updated successfully.');
      setIsEdit(false);
    } catch (err) {
      showErrorToast('Failed to update the space. Please try again.');
      logger.error('failed to update space with error: ', err);
      setIsEdit(false);
    }
  };

  return (
    <Context.Provider
      value={{
        spaceId,
        selectedType,
        setSelectedType,
        isEdit,
        setIsEdit,
        title,
        setTitle,
        startedAt,
        setStartedAt,
        endedAt,
        setEndedAt,
        thread,
        setThread,
        deliberation,
        setDeliberation,
        survey,
        setSurvey,
        answers,
        answer,
        setAnswer,
        draft,
        setDraft,
        handleGoBack,
        handleDownloadExcel,
        userType,
        proposerImage,
        proposerName,
        createdAt,
        status,
        mappedResponses,
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
        handleViewRecord,
      }}
    >
      {children}
    </Context.Provider>
  );
}

export function useDeliberationSpaceContext() {
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
  const { data: feed } = useFeedByID(feedId);

  if (!feed) {
    throw new Error('Feed data is not available');
  }

  return feed;
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
