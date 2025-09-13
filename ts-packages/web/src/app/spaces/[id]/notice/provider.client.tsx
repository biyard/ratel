'use client';

import React, { createContext, useContext, useEffect, useState } from 'react';
import { useSpaceByIdContext } from '../providers.client';
import { useSpaceById } from '@/lib/api/ratel_api';
import { UserType } from '@/lib/api/models/user';
import { StateSetter } from '@/types';
import { logger } from '@/lib/logger';
import { SpaceStatus, Space } from '@/lib/api/models/spaces';
import { PublishingScope } from '@/lib/api/models/notice';
import { useApiCall } from '@/lib/api/use-send';
import { showErrorToast, showInfoToast, showSuccessToast } from '@/lib/toast';
import { validateString } from '@/lib/string-filter-utils';
import { useQueryClient } from '@tanstack/react-query';
import { QK_GET_SPACE_BY_SPACE_ID, QK_LATEST_QUIZ_ATTEMPT } from '@/constants';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import { useQuizUpdates } from '@/hooks/use-quiz-updates';
import { ratelApi } from '@/lib/api/ratel_api';
import {
  postingSpaceRequest,
  spaceUpdateRequest,
} from '@/lib/api/models/spaces';
import {
  spaceSubmitQuizAnswersRequest,
  NoticeAnswer,
} from '@/lib/api/models/notice';

// Quiz validation function
const validateQuizQuestions = (
  questions: Question[],
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  t: (key: string, values?: Record<string, any>) => string,
): string | null => {
  if (questions.length === 0) return null;

  for (let i = 0; i < questions.length; i++) {
    const question = questions[i];
    const questionNumber = i + 1;

    if (!question.title.trim()) {
      return t('quiz_error.empty_title', { n: questionNumber });
    }

    if (question.options.length < 2) {
      return t('quiz_error.min_options', { n: questionNumber });
    }

    if (question.options.length > 4) {
      return t('quiz_error.max_options', { n: questionNumber });
    }

    const optionTexts = question.options.map((opt) =>
      opt.text.trim().toLowerCase(),
    );
    if (new Set(optionTexts).size !== optionTexts.length) {
      return t('quiz_error.duplicate_options', { n: questionNumber });
    }

    for (let j = 0; j < question.options.length; j++) {
      if (!question.options[j].text.trim()) {
        return t('quiz_error.empty_option', {
          n: questionNumber,
          k: j + 1,
        });
      }
    }

    const hasCorrectOption = question.options.some((opt) => opt.isCorrect);
    if (!hasCorrectOption) {
      return t('quiz_error.no_correct', { n: questionNumber });
    }
  }

  return null;
};

import {
  Question,
  convertQuestionsToNoticeQuizRequest,
  convertQuizQuestionsToQuestions,
} from './_components/quiz-builder-ui';
import { useTranslations } from 'next-intl';
import useFeedById from '@/hooks/feeds/use-feed-by-id';

type ContextType = {
  spaceId: number;
  isEdit: boolean;
  setIsEdit: StateSetter<boolean>;
  title: string;
  setTitle: StateSetter<string>;
  startedAt: number;
  setStartedAt: StateSetter<number>;
  endedAt: number;
  setEndedAt: StateSetter<number>;
  htmlContent: string;
  setHtmlContent: StateSetter<string>;
  isPrivatelyPublished: boolean;
  setIsPrivatelyPublished: StateSetter<boolean>;

  // Quiz state
  quizQuestions: Question[];
  setQuizQuestions: StateSetter<Question[]>;

  // Space data
  space: Space | null;

  handleGoBack: () => void;

  userType: UserType;
  proposerImage: string;
  proposerName: string;
  createdAt: number;
  status: SpaceStatus;

  handleLike: () => void;
  handleShare: () => void;
  handleSetStartDate: (startDate: number) => void;
  handleSetEndDate: (endDate: number) => void;
  handlePostingSpace: () => Promise<void>;
  handleEdit: () => void;
  handleSave: () => Promise<void>;
  handleSaveWithoutExitingEditMode: () => Promise<void>;
  handleSaveAndPublish: (scope: PublishingScope) => Promise<void>;
  handlePublishWithScope: (scope: PublishingScope) => Promise<void>;
  handleSubmitQuiz: (questions: Question[]) => Promise<void>;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
}: {
  children: React.ReactNode;
}) {
  const t = useTranslations('NoticeSpace');
  const { spaceId } = useSpaceByIdContext();
  const { data: space, refetch } = useSpaceById(spaceId);
  // We can ignore the feed variable since we're not using it here
  // No longer using tab selection
  const [isEdit, setIsEdit] = useState(false);
  const [title, setTitle] = useState<string>('');
  const [startedAt, setStartedAt] = useState<number>(0);
  const [endedAt, setEndedAt] = useState<number>(0);
  const [htmlContent, setHtmlContent] = useState<string>('');
  // Track if the space is published privately or publicly
  const [isPrivatelyPublished, setIsPrivatelyPublished] =
    useState<boolean>(false);

  // Quiz state
  const [quizQuestions, setQuizQuestions] = useState<Question[]>([]);

  // Track if quiz has been initialized for owners to prevent overwriting
  const quizInitialized = React.useRef(false);

  const queryClient = useQueryClient();
  const { data: userInfo } = useUserInfo();
  const { invalidateQuizData, forceRefreshQuizData } = useQuizUpdates(
    space?.id || 0,
  );

  const { post: callPostingSpace } = useApiCall();
  const { post: callUpdateSpace } = useApiCall();
  const { post: callSubmitQuiz } = useApiCall();
  const { get: getLatestAttempt } = useApiCall();

  const userType = space?.author[0]?.user_type || UserType.Individual;

  const proposerImage = space?.author[0]?.profile_url || '';
  const proposerName = space?.author[0]?.nickname || '';

  const createdAt = space?.created_at ?? 0;
  const status = space?.status ?? SpaceStatus.Draft;

  const handleGoBack = () => {
    if (isEdit) {
      setIsEdit(false);
    } else {
      window.history.back();
    }
  };

  const handleLike = () => {
    // Implement the like functionality for notice spaces
  };

  const handleShare = () => {
    // Implement the share functionality for notice spaces
  };

  const handleSetStartDate = async (startDate: number) => {
    setStartedAt(startDate);

    // If we're in an existing space, update the start date in the backend
    if (space?.id && startDate !== space.started_at) {
      try {
        await callUpdateSpace(
          ratelApi.spaces.getSpaceBySpaceId(space.id),
          spaceUpdateRequest(
            space.html_contents,
            space.files,
            [], // discussions
            [], // elearnings
            [], // surveys
            [], // drafts
            space.title,
            startDate,
            space.ended_at,
            space.publishing_scope, // preserve current publishing scope
            null, // Don't modify quiz when updating dates
          ),
        );

        await queryClient.invalidateQueries({
          queryKey: [QK_GET_SPACE_BY_SPACE_ID, space.id],
        });

        showSuccessToast(t('success_update_start_date'));
      } catch (e) {
        showErrorToast(t('failed_update_start_date'));
        logger.error(e);
        // Revert to original value on error
        setStartedAt(space.started_at || 0);
      }
    }
  };

  const handleSetEndDate = async (endDate: number) => {
    setEndedAt(endDate);

    // If we're in an existing space, update the end date in the backend
    if (space?.id && endDate !== space.ended_at) {
      try {
        await callUpdateSpace(
          ratelApi.spaces.getSpaceBySpaceId(space.id),
          spaceUpdateRequest(
            space.html_contents,
            space.files,
            [], // discussions
            [], // elearnings
            [], // surveys
            [], // drafts
            space.title,
            space.started_at,
            endDate,
            space.publishing_scope, // preserve current publishing scope
            null, // Don't modify quiz when updating dates
          ),
        );

        await queryClient.invalidateQueries({
          queryKey: [QK_GET_SPACE_BY_SPACE_ID, space.id],
        });

        showSuccessToast(t('success_update_end_date'));
      } catch (e) {
        showErrorToast(t('failed_update_end_date'));
        logger.error(e);
        // Revert to original value on error
        setEndedAt(space.ended_at || 0);
      }
    }
  };

  const handlePostingSpace = async () => {
    if (!space) return;
    try {
      await callPostingSpace(
        ratelApi.spaces.getSpaceBySpaceId(space.id),
        postingSpaceRequest(),
      );
      queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, space.id],
      });
      refetch();
      showSuccessToast(t('success_post_space'));
    } catch (e) {
      showErrorToast(t('failed_post_space'));
      logger.error(e);
    }
  };

  const handlePublishWithScope = async (scope: PublishingScope) => {
    if (!space) return;
    try {
      await callPostingSpace(
        ratelApi.spaces.getSpaceBySpaceId(space.id),
        postingSpaceRequest(),
      );

      await callUpdateSpace(
        ratelApi.spaces.getSpaceBySpaceId(space.id),
        spaceUpdateRequest(
          space.html_contents,
          space.files,
          [],
          [],
          [],
          [],
          space.title,
          space.started_at,
          space.ended_at,
          scope,
          null,
        ),
      );

      queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, space.id],
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

  const handleEdit = () => {
    setIsEdit(true);
  };

  const handleSave = async () => {
    if (!space) return;
    console.log('handleSave called');
    // Show temporary saving message
    showInfoToast(t('saving_change'));

    if (!validateString(title)) {
      showErrorToast(t('invalid_title_character'));
      return;
    }

    if (!title.trim()) {
      showErrorToast(t('title_empty'));
      return;
    }

    try {
      // Validate quiz questions before saving
      if (quizQuestions.length > 0) {
        const validationError = validateQuizQuestions(quizQuestions, t);
        if (validationError) {
          showErrorToast(validationError);
          return;
        }
      }

      // Convert frontend quiz questions to backend format
      // When space is InProgress, pass null to prevent quiz modifications
      const quizRequest =
        space.status === SpaceStatus.InProgress
          ? null
          : convertQuestionsToNoticeQuizRequest(quizQuestions);

      // Debug log to verify the correct structure is being sent
      console.log('Quiz being sent:', JSON.stringify(quizRequest, null, 2));

      const updateRequest = spaceUpdateRequest(
        htmlContent,
        space.files,
        [], // discussions
        [], // elearnings
        [], // surveys
        [], // drafts
        title,
        space.started_at,
        space.ended_at,
        space.publishing_scope, // preserve current publishing scope
        quizRequest, // quiz
      );

      // Debug log the full request payload
      console.log(
        'Full update request payload:',
        JSON.stringify(updateRequest, null, 2),
      );

      await callUpdateSpace(
        ratelApi.spaces.getSpaceBySpaceId(space.id),
        updateRequest,
      );

      await queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, space.id],
      });
      showSuccessToast(t('success_update_space'));
      setIsEdit(false);
    } catch (e) {
      showErrorToast(t('failed_update_space'));
      logger.error(e);
    }
  };

  const handleSaveWithoutExitingEditMode = async () => {
    if (!space) return;
    console.log('handleSaveWithoutExitingEditMode called');
    // Show temporary saving message
    showInfoToast(t('saving_change'));

    if (!validateString(title)) {
      showErrorToast(t('invalid_title_character'));
      return;
    }

    if (!title.trim()) {
      showErrorToast(t('title_empty'));
      return;
    }

    try {
      // Validate quiz questions before saving
      if (quizQuestions.length > 0) {
        const validationError = validateQuizQuestions(quizQuestions, t);
        if (validationError) {
          showErrorToast(validationError);
          return;
        }
      }

      // Convert frontend quiz questions to backend format
      // When space is InProgress, pass null to prevent quiz modifications
      const quizRequest =
        space.status === SpaceStatus.InProgress
          ? null
          : convertQuestionsToNoticeQuizRequest(quizQuestions);

      // Debug log to verify the correct structure is being sent
      console.log('Quiz being sent:', JSON.stringify(quizRequest, null, 2));

      const updateRequest = spaceUpdateRequest(
        htmlContent,
        space.files,
        [], // discussions
        [], // elearnings
        [], // surveys
        [], // drafts
        title,
        space.started_at,
        space.ended_at,
        space.publishing_scope, // preserve current publishing scope
        quizRequest, // quiz
      );

      // Debug log the full request payload
      console.log(
        'Full update request payload:',
        JSON.stringify(updateRequest, null, 2),
      );

      await callUpdateSpace(
        ratelApi.spaces.getSpaceBySpaceId(space.id),
        updateRequest,
      );

      await queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, space.id],
      });
      showSuccessToast(t('success_save_space'));
      // Note: Do NOT call setIsEdit(false) here - keep in edit mode
    } catch (e) {
      showErrorToast(t('failed_save_space'));
      logger.error(e);
    }
  };

  const handleSaveAndPublish = async (scope: PublishingScope) => {
    if (!space) return;
    console.log('handleSaveAndPublish called with scope:', scope);

    // Show temporary saving message
    showInfoToast(t('saving_change'));

    if (!validateString(title)) {
      showErrorToast(t('invalid_title_character'));
      return;
    }

    if (!title.trim()) {
      showErrorToast(t('title_empty'));
      return;
    }

    try {
      // Validate quiz questions before saving
      if (quizQuestions.length > 0) {
        const validationError = validateQuizQuestions(quizQuestions, t);
        if (validationError) {
          showErrorToast(validationError);
          return;
        }
      }

      // Convert frontend quiz questions to backend format
      // When space is InProgress, pass null to prevent quiz modifications
      const quizRequest =
        space.status === SpaceStatus.InProgress
          ? null
          : convertQuestionsToNoticeQuizRequest(quizQuestions);

      // Create update request with BOTH the content changes AND the publishing scope
      const updateRequest = spaceUpdateRequest(
        htmlContent,
        space.files,
        [], // discussions
        [], // elearnings
        [], // surveys
        [], // drafts
        title,
        space.started_at,
        space.ended_at,
        scope, // Use the new publishing scope
        quizRequest, // quiz
      );

      // Debug log the full request payload
      console.log(
        'Save and publish request payload:',
        JSON.stringify(updateRequest, null, 2),
      );

      await callUpdateSpace(
        ratelApi.spaces.getSpaceBySpaceId(space.id),
        updateRequest,
      );

      await queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, space.id],
      });

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

  const handleSubmitQuiz = async (questions: Question[]) => {
    console.log('Provider handleSubmitQuiz called with questions:', questions);
    console.log('Space:', space);
    console.log('UserInfo:', userInfo);
    if (!space || !userInfo) {
      console.log('No space or user info available');
      return;
    }

    try {
      // Convert user answers to NoticeAnswer format (same as QuizTaker does)
      const answers: { [questionId: string]: string[] } = {};

      // Map frontend questions with selections to backend format
      if (space.notice_quiz) {
        space.notice_quiz.forEach((backendQuestion, questionIndex) => {
          const frontendQuestion = questions[questionIndex];
          if (frontendQuestion) {
            const selectedOptionIds: string[] = [];

            frontendQuestion.options.forEach((frontendOption, optionIndex) => {
              if (frontendOption.isSelected) {
                // Map to backend option ID using the same index
                const backendOption = backendQuestion.options[optionIndex];
                if (backendOption) {
                  selectedOptionIds.push(backendOption.id);
                }
              }
            });

            if (selectedOptionIds.length > 0) {
              answers[backendQuestion.id] = selectedOptionIds;
            }
          }
        });
      }

      const noticeAnswer: NoticeAnswer = { answers };

      // Create the submit request payload using correct format
      const submitRequest = spaceSubmitQuizAnswersRequest(noticeAnswer);

      // Debug log to verify the payload
      console.log(
        'Submitting quiz answers:',
        JSON.stringify(submitRequest, null, 2),
      );

      // Call the submit API
      console.log('About to call API...');
      await callSubmitQuiz(
        ratelApi.notice_quiz.submitQuizAnswers(space.id),
        submitRequest,
      );

      console.log('API call successful!');

      // First, invalidate all quiz-related queries
      await invalidateQuizData();

      // Poll for updated data with exponential backoff
      let retries = 0;
      const maxRetries = 5;
      while (retries < maxRetries) {
        await forceRefreshQuizData();
        // Check if data is updated, if so break
        // Otherwise wait with exponential backoff
        await new Promise((resolve) =>
          setTimeout(resolve, Math.pow(2, retries) * 200),
        );
        retries++;
      }

      // Get the latest attempt to show appropriate feedback
      try {
        const latestAttempt = await queryClient.fetchQuery({
          queryKey: [QK_LATEST_QUIZ_ATTEMPT, space.id],
          queryFn: async () => {
            const response = await getLatestAttempt(
              ratelApi.notice_quiz.getLatestQuizAttempt(space.id),
            );
            // Return the latest attempt (first item since it's ordered by created_at desc)
            return response.items.length > 0 ? response.items[0] : null;
          },
          staleTime: 0, // Force fresh data
          retry: 3, // Retry up to 3 times if it fails
        });

        console.log('Latest attempt after submission:', latestAttempt);

        // Trigger refresh to update quiz data and UI state
        await forceRefreshQuizData();

        // Wait a short moment and refresh again to ensure UI updates
        setTimeout(async () => {
          await forceRefreshQuizData();
        }, 200);
      } catch (attemptError) {
        console.error('Failed to fetch attempt data:', attemptError);
        // Still refresh queries even if fetch failed
        await forceRefreshQuizData();
      }
    } catch (e) {
      console.error('Submit quiz error:', e);

      // Handle specific error cases
      const errorMessage = e instanceof Error ? e.message : String(e);

      if (
        errorMessage.includes('Unauthorized') ||
        errorMessage.includes('401')
      ) {
        // This could be either max attempts reached or already passed
        // Try to get the latest attempt to determine the specific error
        try {
          const currentLatestAttempt = await queryClient.fetchQuery({
            queryKey: [QK_LATEST_QUIZ_ATTEMPT, space.id],
            queryFn: async () => {
              const response = await getLatestAttempt(
                ratelApi.notice_quiz.getLatestQuizAttempt(space.id),
              );
              return response.items.length > 0 ? response.items[0] : null;
            },
            staleTime: 0,
          });

          if (currentLatestAttempt?.is_successful) {
            showErrorToast(t('already_passed_quiz'));
          } else {
            showErrorToast(t('max_attempts_for_this_quiz'));
          }
        } catch {
          showErrorToast(t('cannot_submit_already_or_max'));
        }
      } else {
        showErrorToast(t('failed_to_submit_quiz'));
      }

      logger.error(e);
    }
  };

  useEffect(() => {
    if (space) {
      setTitle(space.title || '');
      setStartedAt(space.started_at ?? 0);
      setEndedAt(space.ended_at ?? 0);
      // Initialize with space description or empty HTML
      setHtmlContent(
        space.html_contents ||
          '<p>This is the notice content. Edit to add details.</p>',
      );
      // Set the publishing state based on space's publishing_scope
      setIsPrivatelyPublished(
        space.status === SpaceStatus.InProgress &&
          space.publishing_scope === PublishingScope.Private,
      );

      // Initialize quiz questions
      // For owners, let the quiz builder handle initialization with correct answers
      // For non-owners, use the read-only format
      const isOwner = userInfo?.id === space?.owner_id;
      if (!isOwner) {
        setQuizQuestions(
          convertQuizQuestionsToQuestions(space.notice_quiz) || [],
        );
        quizInitialized.current = true;
      } else {
        // For owners, only initialize if not already initialized
        // This allows the quiz builder to load with correct answers
        if (!quizInitialized.current) {
          setQuizQuestions(
            convertQuizQuestionsToQuestions(space.notice_quiz) || [],
          );
          quizInitialized.current = true;
        }
      }
    }
  }, [space, userInfo]);

  const contextValue = {
    spaceId,
    // Tab selection removed
    isEdit,
    setIsEdit,
    title,
    setTitle,
    startedAt,
    setStartedAt,
    endedAt,
    setEndedAt,
    htmlContent,
    setHtmlContent,
    isPrivatelyPublished,
    setIsPrivatelyPublished,
    quizQuestions,
    setQuizQuestions,
    space,
    handleGoBack,
    userType,
    proposerImage,
    proposerName,
    createdAt,
    status,
    handleLike,
    handleShare,
    handleSetStartDate,
    handleSetEndDate,
    handlePostingSpace,
    handleEdit,
    handleSave,
    handleSaveWithoutExitingEditMode,
    handleSaveAndPublish,
    handlePublishWithScope,
    handleSubmitQuiz,
  };

  if (!space) {
    return null;
  }

  return <Context.Provider value={contextValue}>{children}</Context.Provider>;
}

export function useNoticeSpaceContext() {
  const context = useContext(Context);
  if (context === undefined) {
    throw new Error(
      'useNoticeSpaceContext must be used within a ClientProviders',
    );
  }
  return context;
}

export function useNoticeSpace() {
  const { spaceId } = useSpaceByIdContext();
  const { data: space } = useSpaceById(spaceId);
  if (!space) {
    throw new Error('Space not found');
  }
  return space;
}

export function useNoticeFeed(feedId: number) {
  const { data: feed } = useFeedById(feedId);
  if (!feed) {
    throw new Error('Feed not found');
  }
  return feed;
}
