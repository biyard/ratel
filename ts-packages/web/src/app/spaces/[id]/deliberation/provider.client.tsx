'use client';

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
import { Answer, surveyResponseCreateRequest } from '@/lib/api/models/response';
import { useApiCall } from '@/lib/api/use-send';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { checkString } from '@/lib/string-filter-utils';
import { FileInfo } from '@/lib/api/models/feeds';
import { DiscussionCreateRequest } from '@/lib/api/models/discussion';
import { ElearningCreateRequest } from '@/lib/api/models/elearning';
import { SurveyCreateRequest } from '@/lib/api/models/survey';
import { SpaceDraftCreateRequest } from '@/lib/api/models/space_draft';

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
  answer: SurveyAnswer;
  setAnswer: StateSetter<SurveyAnswer>;
  draft: FinalConsensus;
  setDraft: StateSetter<FinalConsensus>;
  handleGoBack: () => void;

  userType: UserType;
  proposerImage: string;
  proposerName: string;
  createdAt: number;
  status: SpaceStatus;

  handleSetAnswers: (answers: Answer[]) => void;
  handleSetStartDate: (startDate: number) => void;
  handleSetEndDate: (endDate: number) => void;
  handleSend: () => Promise<void>;
  handlePostingSpace: () => Promise<void>;
  handleEdit: () => void;
  handleSave: () => Promise<void>;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
}: {
  children: React.ReactNode;
}) {
  const { spaceId } = useSpaceByIdContext();
  const data = useSpaceById(spaceId);
  const space = data.data;

  const [selectedType, setSelectedType] = useState<DeliberationTabType>(
    DeliberationTab.SUMMARY,
  );
  const [isEdit, setIsEdit] = useState(false);
  const [title, setTitle] = useState(space.title ?? '');
  const [startedAt, setStartedAt] = useState(
    changeStartedAt(Math.floor(space.started_at ?? Date.now() / 1000)),
  );
  const [endedAt, setEndedAt] = useState(
    changeEndedAt(Math.floor(space.ended_at ?? Date.now() / 1000)),
  );
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

  const router = useRouter();

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
      showSuccessToast('Your response has been saved successfully.');
      data.refetch();
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
  };

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
        survey.surveys,
        draft.drafts,
      );
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
        answer,
        setAnswer,
        draft,
        setDraft,
        handleGoBack,
        userType,
        proposerImage,
        proposerName,
        createdAt,
        status,
        handleSetAnswers,
        handleSetStartDate,
        handleSetEndDate,
        handleSend,
        handlePostingSpace,
        handleEdit,
        handleSave,
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

function changeStartedAt(timestamp: number) {
  const date = new Date(timestamp * 1000);
  date.setUTCHours(0, 0, 0, 0);
  const newDate = Math.floor(date.getTime() / 1000);
  return newDate;
}

function changeEndedAt(timestamp: number) {
  const date = new Date(timestamp * 1000);
  date.setUTCHours(23, 59, 59, 0);
  const newDate = Math.floor(date.getTime() / 1000);
  return newDate;
}
