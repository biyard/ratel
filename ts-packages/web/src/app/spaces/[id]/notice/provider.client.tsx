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
import { useFeedByID } from '@/app/(social)/_hooks/feed';
import { ratelApi } from '@/lib/api/ratel_api';
import {
  postingSpaceRequest,
  spaceUpdateRequest,
} from '@/lib/api/models/spaces';
import { spaceSubmitQuizAnswersRequest } from '@/lib/api/models/notice';

// Quiz validation function
const validateQuizQuestions = (questions: Question[]): string | null => {
  if (questions.length === 0) return null;

  for (let i = 0; i < questions.length; i++) {
    const question = questions[i];
    const questionNumber = i + 1;

    // Check if question title is not empty
    if (!question.title.trim()) {
      return `Question ${questionNumber} title cannot be empty`;
    }

    // Check if question has at least 2 options
    if (question.options.length < 2) {
      return `Question ${questionNumber} must have at least 2 options`;
    }

    // Check if question has no more than 4 options
    if (question.options.length > 4) {
      return `Question ${questionNumber} can have maximum 4 options`;
    }

    // Check for duplicate options (case-insensitive)
    const optionTexts = question.options.map((opt) =>
      opt.text.trim().toLowerCase(),
    );
    const uniqueTexts = new Set(optionTexts);

    if (optionTexts.length !== uniqueTexts.size) {
      return `Question ${questionNumber} has duplicate options`;
    }

    // Check for empty options
    for (let j = 0; j < question.options.length; j++) {
      if (!question.options[j].text.trim()) {
        return `Question ${questionNumber} option ${j + 1} cannot be empty`;
      }
    }

    // Check if at least one option is marked as correct
    const hasCorrectOption = question.options.some(
      (option) => option.isCorrect,
    );
    if (!hasCorrectOption) {
      return `Question ${questionNumber} must have at least one correct option selected`;
    }
  }

  return null; // No validation errors
};
import {
  Question,
  convertQuestionsToNoticeQuestionsWithAnswers,
  convertQuizQuestionsToQuestions,
  convertUserAnswersToNoticeQuestionsWithAnswers,
} from './_components/quiz-builder-ui';

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
  handlePublishWithScope: (scope: PublishingScope) => Promise<void>;
  handleSubmitQuiz: (questions: Question[]) => Promise<void>;
};

export const Context = createContext<ContextType | undefined>(undefined);

export default function ClientProviders({
  children,
}: {
  children: React.ReactNode;
}) {
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

  // We'll use a simpler approach for the proposer details
  const proposerImage = ''; // This would need to be properly implemented based on the actual model
  const proposerName = ''; // This would need to be properly implemented based on the actual model

  const createdAt = space?.created_at ?? 0;
  const status = space?.status ?? SpaceStatus.Draft;

  const handleGoBack = () => {
    window.history.back();
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

        showSuccessToast('Start date updated successfully');
      } catch (e) {
        showErrorToast('Failed to update start date');
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

        showSuccessToast('End date updated successfully');
      } catch (e) {
        showErrorToast('Failed to update end date');
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
      showSuccessToast('Space posted successfully');
    } catch (e) {
      showErrorToast('Failed to post space');
      logger.error(e);
    }
  };

  const handlePublishWithScope = async (scope: PublishingScope) => {
    if (!space) return;
    try {
      // First, post the space
      await callPostingSpace(
        ratelApi.spaces.getSpaceBySpaceId(space.id),
        postingSpaceRequest(),
      );

      // Then, update the publishing scope
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
          space.ended_at,
          scope,
          null, // Don't modify quiz when publishing
        ),
      );

      queryClient.invalidateQueries({
        queryKey: [QK_GET_SPACE_BY_SPACE_ID, space.id],
      });
      refetch();
      showSuccessToast(
        `Space published ${scope === PublishingScope.Public ? 'publicly' : 'privately'} successfully`,
      );
    } catch (e) {
      showErrorToast('Failed to publish space');
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
    showInfoToast('Saving changes...');

    if (!validateString(title)) {
      showErrorToast('Title contains invalid characters');
      return;
    }

    if (!title.trim()) {
      showErrorToast('Title cannot be empty');
      return;
    }

    try {
      // Validate quiz questions before saving
      if (quizQuestions.length > 0) {
        const validationError = validateQuizQuestions(quizQuestions);
        if (validationError) {
          showErrorToast(validationError);
          return;
        }
      }

      // Convert frontend quiz questions to backend format
      const quizWithAnswers =
        convertQuestionsToNoticeQuestionsWithAnswers(quizQuestions);

      // Debug log to verify is_correct field is included
      console.log('Quiz being sent:', JSON.stringify(quizWithAnswers, null, 2));

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
        quizWithAnswers, // quiz
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
      showSuccessToast('Space updated successfully');
      setIsEdit(false);
    } catch (e) {
      showErrorToast('Failed to update space');
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
      // Convert frontend quiz questions with user selections to backend format
      const quizAnswers =
        convertUserAnswersToNoticeQuestionsWithAnswers(questions);
      console.log('Converted to quiz answers:', quizAnswers);

      // Create the submit request payload
      const submitRequest = spaceSubmitQuizAnswersRequest(quizAnswers);

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

      // Wait a bit to ensure the database has been updated
      await new Promise((resolve) => setTimeout(resolve, 500));

      // Force a complete refresh of all quiz data
      await forceRefreshQuizData();

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

        // Show appropriate toast based on success status
        if (latestAttempt && latestAttempt.is_successful) {
          showSuccessToast('Coin Earned! View it in your profile.');
        } else {
          showErrorToast('Each wrong answer cuts your reward in half!');
        }

        // Trigger one final refresh to ensure all components update
        await forceRefreshQuizData();

        // Wait a short moment and refresh again to ensure UI updates
        setTimeout(async () => {
          await forceRefreshQuizData();
        }, 200);
      } catch (attemptError) {
        console.error('Failed to fetch attempt data:', attemptError);
        // Still refresh queries even if fetch failed
        await forceRefreshQuizData();
        // Fallback to generic success message
        showSuccessToast('Quiz submitted successfully!');
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
            showErrorToast('You have already passed this quiz.');
          } else {
            showErrorToast(
              'You have reached the maximum number of attempts (3) for this quiz.',
            );
          }
        } catch {
          // Fallback error message if we can't determine the specific reason
          showErrorToast(
            'Cannot submit: either you have already passed or reached maximum attempts.',
          );
        }
      } else {
        showErrorToast('Failed to submit quiz');
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
  const { data: feed } = useFeedByID(feedId);
  if (!feed) {
    throw new Error('Feed not found');
  }
  return feed;
}
