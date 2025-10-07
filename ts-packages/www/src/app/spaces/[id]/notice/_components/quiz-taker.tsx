'use client';
import * as React from 'react';

import { useState, useCallback } from 'react';
import {
  QuizQuestion,
  spaceSubmitQuizAnswersRequest,
  NoticeAnswer,
} from '@/lib/api/models/notice';

// Extended interface for user answers with selected state
interface QuizQuestionWithSelection {
  title: string;
  images: string[];
  options: Array<{
    content: string;
    is_selected: boolean;
  }>;
}

import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { showErrorToast, showInfoToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import Image from 'next/image';
import { useTranslation } from 'react-i18next';

interface QuizTakerProps {
  spaceId: number;
  questions: QuizQuestion[];
  onSubmitSuccess?: (attemptData: QuizAttemptResult) => void;
  disabled?: boolean;
}

interface QuizAttemptResult {
  is_successful: boolean;
}

export default function QuizTaker({
  spaceId,
  questions,
  onSubmitSuccess,
  disabled = false,
}: QuizTakerProps) {
  const [userAnswers, setUserAnswers] = useState<QuizQuestionWithSelection[]>(
    [],
  );
  const [isSubmitting, setIsSubmitting] = useState(false);
  const { post } = useApiCall();
  const { t } = useTranslation('NoticeSpace');

  // Initialize user answers when questions change
  React.useEffect(() => {
    if (questions.length > 0) {
      const initialAnswers = questions.map((question) => ({
        ...question,
        options: question.options.map((option) => ({
          content: option.content,
          is_selected: false,
        })),
      }));
      setUserAnswers(initialAnswers);
    }
  }, [questions]);

  const handleOptionToggle = useCallback(
    (questionIndex: number, optionIndex: number) => {
      if (disabled) return;

      setUserAnswers((prev) => {
        const newAnswers = [...prev];
        if (newAnswers[questionIndex]) {
          const newOptions = [...newAnswers[questionIndex].options];

          // Single choice only - clear all other selections when selecting one
          newOptions.forEach((opt, idx) => {
            if (idx === optionIndex) {
              opt.is_selected = !opt.is_selected; // Toggle the clicked option
            } else {
              opt.is_selected = false; // Clear all other selections
            }
          });

          newAnswers[questionIndex] = {
            ...newAnswers[questionIndex],
            options: newOptions,
          };
        }
        return newAnswers;
      });
    },
    [disabled],
  );

  const validateAnswers = useCallback(() => {
    // Check if all questions have at least one selected option
    for (let i = 0; i < userAnswers.length; i++) {
      const hasSelection = userAnswers[i].options.some(
        (opt) => opt.is_selected === true,
      );
      if (!hasSelection) {
        showErrorToast(t('quiz_answer_required_n', { n: i + 1 }));
        return false;
      }
    }
    return true;
  }, [userAnswers, t]);

  const handleSubmit = useCallback(async () => {
    if (disabled || isSubmitting) return;

    if (!validateAnswers()) return;

    setIsSubmitting(true);
    showInfoToast(t('submitting_answers'));

    try {
      // NEW: Convert user answers to HashMap format for O(1) backend validation
      const answers: { [questionId: string]: string[] } = {};

      questions.forEach((backendQuestion, questionIndex) => {
        const userQuestion = userAnswers[questionIndex];
        if (userQuestion) {
          const selectedOptionIds: string[] = [];

          userQuestion.options.forEach((userOption, optionIndex) => {
            if (userOption.is_selected) {
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

      const noticeAnswer: NoticeAnswer = { answers };

      // Create the request body using new format
      const requestBody = spaceSubmitQuizAnswersRequest(noticeAnswer);

      const response = await post(
        ratelApi.notice_quiz.submitQuizAnswers(spaceId),
        requestBody,
      );

      if (response) {
        // Success/failure notifications are handled by the provider
        onSubmitSuccess?.(response);
      }
    } catch (error: unknown) {
      logger.error('Failed to submit quiz answers:', error);

      // Handle specific error cases
      const errorMessage =
        error instanceof Error ? error.message : String(error);

      if (errorMessage.includes('maximum attempts')) {
        showErrorToast(t('maximum_attempts_reached'));
      } else if (errorMessage.includes('InvalidInputValue')) {
        showErrorToast(t('failed_to_submit_quiz'));
      } else {
        showErrorToast(t('failed_to_submit_quiz'));
      }
    } finally {
      setIsSubmitting(false);
    }
  }, [
    spaceId,
    userAnswers,
    disabled,
    isSubmitting,
    validateAnswers,
    post,
    onSubmitSuccess,
    questions,
    t,
  ]);

  if (questions.length === 0) {
    return (
      <div className="bg-[var(--color-component-bg)] rounded-[10px] p-6 text-center">
        <p className="text-white/70">{t('no_quiz_warning')}</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="bg-[var(--color-component-bg)] rounded-[10px] p-6">
        <h2 className="text-xl font-bold text-white mb-4">
          {t('interactive_quiz_title')}
        </h2>
        <p className="text-white/70 mb-6">
          {t('interactive_quiz_desc')} {t('attempts_up_to_n', { n: 3 })}
        </p>

        <div className="space-y-6">
          {userAnswers.map((question, questionIndex) => (
            <div
              key={questionIndex}
              className="border-b border-white/10 pb-6 last:border-b-0"
            >
              <h3 className="text-lg font-medium text-white mb-4">
                {questionIndex + 1}. {question.title}
              </h3>

              {/* Question Images */}
              {question.images && question.images.length > 0 && (
                <div className="mb-4">
                  {question.images.map((imageUrl, imageIndex) => (
                    <div
                      key={imageIndex}
                      className="relative w-full max-w-md mx-auto"
                    >
                      {imageUrl && (
                        <Image
                          src={imageUrl}
                          alt={`Question ${questionIndex + 1} image ${imageIndex + 1}`}
                          width={400}
                          height={300}
                          className="rounded-lg object-contain"
                        />
                      )}
                    </div>
                  ))}
                </div>
              )}

              {/* Options */}
              <div className="space-y-3">
                {question.options.map((option, optionIndex) => (
                  <div
                    key={optionIndex}
                    className={`flex items-center p-3 rounded-lg border cursor-pointer transition-colors ${
                      option.is_selected
                        ? 'border-[var(--color-primary)] bg-[var(--color-primary)]/10'
                        : 'border-white/20 hover:border-white/40'
                    } ${disabled ? 'cursor-not-allowed opacity-50' : ''}`}
                    onClick={() =>
                      handleOptionToggle(questionIndex, optionIndex)
                    }
                  >
                    <div
                      className={`w-5 h-5 rounded-full border-2 flex items-center justify-center mr-3 ${
                        option.is_selected
                          ? 'border-[var(--color-primary)] bg-[var(--color-primary)]'
                          : 'border-white/30'
                      }`}
                    >
                      {option.is_selected && (
                        <div className="w-2 h-2 rounded-full bg-black"></div>
                      )}
                    </div>
                    <span className="text-white flex-1">{option.content}</span>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>

        <div className="mt-8 flex justify-center">
          <button
            onClick={handleSubmit}
            disabled={disabled || isSubmitting}
            className={`px-8 py-3 rounded-lg font-medium transition-colors ${
              disabled || isSubmitting
                ? 'bg-gray-600 text-gray-400 cursor-not-allowed'
                : 'bg-blue-600 hover:bg-blue-700 text-white'
            }`}
          >
            {isSubmitting ? t('submitting') : t('submit')}
          </button>
        </div>
      </div>
    </div>
  );
}
