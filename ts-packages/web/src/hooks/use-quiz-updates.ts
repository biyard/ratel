'use client';

import { useEffect } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import {
  QK_LATEST_QUIZ_ATTEMPT,
  QK_QUIZ_ATTEMPTS,
  QK_QUIZ_ATTEMPT,
  QK_QUIZ,
} from '@/constants';

/**
 * Custom hook to handle quiz-related data updates
 * This ensures that all components that depend on quiz data are properly updated
 * when a quiz is submitted or when quiz attempt data changes
 */
export function useQuizUpdates(spaceId: number) {
  const queryClient = useQueryClient();

  const invalidateQuizData = async () => {
    // Invalidate all quiz-related queries
    await Promise.all([
      // Latest quiz attempt data
      queryClient.invalidateQueries({
        queryKey: [QK_LATEST_QUIZ_ATTEMPT, spaceId],
      }),
      // Quiz attempts data (for attempt counting)
      queryClient.invalidateQueries({
        queryKey: [QK_QUIZ_ATTEMPTS, spaceId],
      }),
      // Space data (may contain quiz configuration)
      // queryClient.invalidateQueries({
      //   queryKey: [QK_GET_SPACE_BY_SPACE_ID, spaceId],
      // }),
      // Any other quiz-related queries
      queryClient.invalidateQueries({
        queryKey: [QK_QUIZ_ATTEMPT],
        exact: false,
      }),
      queryClient.invalidateQueries({
        queryKey: [QK_QUIZ],
        exact: false,
      }),
    ]);
  };

  const forceRefreshQuizData = async () => {
    // First invalidate all queries
    await invalidateQuizData();

    // Then force refetch the critical ones
    await Promise.all([
      queryClient.refetchQueries({
        queryKey: [QK_LATEST_QUIZ_ATTEMPT, spaceId],
        type: 'active',
      }),
      queryClient.refetchQueries({
        queryKey: [QK_QUIZ_ATTEMPTS, spaceId],
        type: 'active',
      }),
      // queryClient.refetchQueries({
      //   queryKey: [QK_GET_SPACE_BY_SPACE_ID, spaceId],
      //   type: 'active',
      // }),
    ]);
  };

  const refetchQuizData = async () => {
    // Force refetch of critical quiz data
    await Promise.all([
      queryClient.refetchQueries({
        queryKey: [QK_LATEST_QUIZ_ATTEMPT, spaceId],
      }),
      queryClient.refetchQueries({
        queryKey: [QK_QUIZ_ATTEMPTS, spaceId],
      }),
      // queryClient.refetchQueries({
      //   queryKey: [QK_GET_SPACE_BY_SPACE_ID, spaceId],
      // }),
    ]);
  };

  return {
    invalidateQuizData,
    refetchQuizData,
    forceRefreshQuizData,
  };
}

/**
 * Hook to automatically refresh quiz data when the component mounts
 * Useful for components that display quiz-related information
 */
export function useQuizDataRefresh(spaceId: number, enabled = true) {
  const { refetchQuizData } = useQuizUpdates(spaceId);

  useEffect(() => {
    if (enabled && spaceId > 0) {
      // Refresh data when component mounts
      refetchQuizData();
    }
  }, [spaceId, enabled, refetchQuizData]);
}
