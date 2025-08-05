'use client';

import React, { useCallback, useState } from 'react';
import {
  DndContext,
  closestCenter,
  PointerSensor,
  useSensor,
  useSensors,
  DragOverlay,
  DragStartEvent,
  DragEndEvent,
} from '@dnd-kit/core';
import {
  SortableContext,
  verticalListSortingStrategy,
  useSortable,
  arrayMove,
} from '@dnd-kit/sortable';
import {
  restrictToVerticalAxis,
  restrictToParentElement,
} from '@dnd-kit/modifiers';
import { CSS } from '@dnd-kit/utilities';
import {
  Image2,
  Add,
  Delete2,
  Remove,
  DialPad,
  DialPad2,
} from '@/components/icons';
import { QuizQuestion, NoticeQuizRequest } from '@/lib/api/models/notice';
import { SpaceStatus } from '@/lib/api/models/spaces';
import Image from 'next/image';
import FileUploader from '@/components/file-uploader';
import { usePopup } from '@/lib/contexts/popup-service';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import {
  useLatestQuizAttempt,
  useQuizAttempts,
  useQuizAnswers,
} from '@/lib/api/ratel_api';
import QuizSubmitForm from './modal/quiz-submit-form';

// Define types for our quiz data
export interface Option {
  id: string;
  text: string;
  isCorrect: boolean;
  isSelected: boolean; // For user selections in read mode
}

export interface Question {
  id: string;
  title: string;
  imageUrl: string | null;
  options: Option[];
}

interface QuizBuilderProps {
  isEditMode: boolean;
  questions: Question[];
  onQuestionsChange: (questions: Question[]) => void;
  onSubmitQuiz?: (questions: Question[]) => Promise<void>;
  spaceId?: number;
  userId?: number;
  isOwner?: boolean;
  spaceStatus?: SpaceStatus;
}

// Utility functions to convert between frontend and backend formats
export function convertQuizQuestionToQuestion(
  quizQuestion: QuizQuestion,
  id: string,
): Question {
  return {
    id,
    title: quizQuestion.title,
    imageUrl:
      quizQuestion.images.length > 0 ? quizQuestion.images[0].url : null,
    options: quizQuestion.options.map((option, index) => ({
      id: `option-${Date.now()}-${index}`,
      text: option.content,
      isCorrect: false, // Read-only version doesn't have correct answers
      isSelected: false, // Initialize user selection as false
    })),
  };
}

// Convert to the new backend format (NoticeQuizRequest)
export function convertQuestionsToNoticeQuizRequest(
  questions: Question[],
): NoticeQuizRequest {
  return {
    questions: questions.map((question) => ({
      title: question.title,
      images: question.imageUrl ? [question.imageUrl] : [],
      options: question.options.map((option) => ({
        content: option.text,
        is_correct: option.isCorrect,
      })),
    })),
  };
}

export function convertQuizQuestionsToQuestions(
  quizQuestions: QuizQuestion[],
): Question[] {
  return quizQuestions.map((qq, index) =>
    convertQuizQuestionToQuestion(qq, `question-${Date.now()}-${index}`),
  );
}

export default function QuizBuilderUI({
  isEditMode,
  questions,
  onQuestionsChange,
  onSubmitQuiz,
  spaceId,
  userId,
  isOwner,
  spaceStatus,
}: QuizBuilderProps) {
  const popup = usePopup();

  // Check if quiz editing should be disabled (when space is InProgress)
  const isQuizEditingDisabled = spaceStatus === SpaceStatus.InProgress;

  // Drag state
  const [activeOptionId, setActiveOptionId] = useState<{
    questionId: string;
    optionId: string;
  } | null>(null);

  // Initialize sensors for drag and drop
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8, // 8px of movement required before drag starts
      },
    }),
  );

  // Fetch quiz attempts for non-edit mode
  const {
    data: attemptsData,
    isLoading: attemptsLoading,
    isFetching: attemptsFetching,
  } = useQuizAttempts(spaceId || 0);

  // Fetch latest quiz attempt for non-edit mode (for backward compatibility)
  const { data: latestAttempt } = useLatestQuizAttempt(spaceId || 0);

  // Fetch quiz answers to check ownership and get correct answers
  // The API returns 200 if user is owner, 401/403 if not
  const {
    data: quizAnswers,
    error: quizAnswersError,
    isLoading: quizAnswersLoading,
    isError: quizAnswersIsError,
  } = useQuizAnswers(
    spaceId || 0,
    !!(spaceId && spaceId > 0), // Only fetch if we have a valid spaceId
  );

  // Determine if user is owner based on API response
  // If we successfully got data, user is owner. If there's an error, user is not owner.
  const isActualOwner = !!quizAnswers && !quizAnswersIsError;

  // Debug logging for API call
  React.useEffect(() => {
    console.log('Quiz Answers API Debug:', {
      spaceId,
      enabled: !!(spaceId && spaceId > 0),
      quizAnswers,
      quizAnswersError,
      quizAnswersLoading,
      quizAnswersIsError,
      isActualOwner,
      timestamp: new Date().toISOString(),
    });
  }, [
    spaceId,
    quizAnswers,
    quizAnswersError,
    quizAnswersLoading,
    quizAnswersIsError,
    isActualOwner,
  ]);

  // Track attempts data changes for debugging
  React.useEffect(() => {
    const failedCount =
      attemptsData?.items?.filter((attempt) => !attempt.is_successful)
        ?.length || 0;
    console.log('Attempts data updated:', {
      attemptsData,
      totalCount: attemptsData?.total_count,
      itemsLength: attemptsData?.items?.length,
      failedAttemptsCount: failedCount,
      isLoading: attemptsLoading,
      isFetching: attemptsFetching,
      timestamp: new Date().toISOString(),
    });
  }, [attemptsData, attemptsLoading, attemptsFetching]);

  // Calculate next attempt number using failed attempts count only
  // Only unsuccessful attempts count towards the attempt number
  const failedAttempts =
    attemptsData?.items?.filter((attempt) => !attempt.is_successful) || [];
  const nextAttemptNumber = failedAttempts.length + 1;

  // Debug logging for submit button conditions
  console.log('Quiz Builder Debug:', {
    isEditMode,
    questionsLength: questions.length,
    hasOnSubmitQuiz: !!onSubmitQuiz,
    isOwner,
    latestAttempt,
    nextAttemptNumber,
    failedAttemptsCount: failedAttempts.length,
    totalAttempts: attemptsData?.total_count || 0,
    showSubmitButton:
      !isEditMode && questions.length > 0 && onSubmitQuiz && !isOwner,
  });

  // Add a style tag for placeholder color
  React.useEffect(() => {
    const styleTag = document.createElement('style');
    styleTag.innerHTML = `
      .title-input::placeholder {
        color: var(--color-neutral-600);
      }
    `;
    document.head.appendChild(styleTag);

    return () => {
      document.head.removeChild(styleTag);
    };
  }, []);

  // Track if we've already loaded quiz answers to prevent infinite loops
  const hasLoadedQuizAnswers = React.useRef(false);

  // Load quiz questions with correct answers for owners in edit mode
  // This merges space.notice_quiz (questions) with quizAnswers.answers (correct answers)
  // Show correct answers for owners in edit mode regardless of space status
  React.useEffect(() => {
    console.log('Quiz answer loading effect triggered:', {
      isActualOwner,
      isEditMode,
      questionsLength: questions.length,
      hasQuizAnswers: !!quizAnswers,
      quizAnswersStructure: quizAnswers ? Object.keys(quizAnswers) : [],
      answersStructure: quizAnswers?.answers
        ? Object.keys(quizAnswers.answers)
        : [],
      hasLoadedQuizAnswers: hasLoadedQuizAnswers.current,
    });

    if (
      isActualOwner &&
      isEditMode &&
      questions.length > 0 &&
      quizAnswers?.answers?.answers &&
      !hasLoadedQuizAnswers.current
    ) {
      console.log('Loading quiz with correct answers for owner in edit mode');
      console.log('Questions:', questions);
      console.log('Quiz answers raw:', quizAnswers);
      console.log('Quiz answers data:', quizAnswers.answers.answers);

      // The backend stores answers as { questionId: [optionIds] }
      // But we need to match by question/option content since IDs might not match
      const correctAnswersMap = quizAnswers.answers.answers;

      console.log('Correct answers map:', correctAnswersMap);
      console.log(
        'Number of questions with answers:',
        Object.keys(correctAnswersMap).length,
      );

      // For now, let's try a simpler approach - mark the first option of each question as correct
      // This is a temporary solution to test if the API call and ownership detection work
      const questionsWithCorrectAnswers: Question[] = questions.map(
        (question, questionIndex) => {
          console.log(`Processing question ${questionIndex}:`, question.title);

          // Get all answer entries and try to match by index for now
          const answerEntries = Object.entries(correctAnswersMap);
          let correctOptionIds: string[] = [];

          if (answerEntries[questionIndex]) {
            correctOptionIds = answerEntries[questionIndex][1];
            console.log(
              `Found correct option IDs for question ${questionIndex}:`,
              correctOptionIds,
            );
          }

          const updatedQuestion = {
            ...question,
            options: question.options.map((option, optionIndex) => {
              // For testing: mark first option as correct if we have any answers
              const isCorrect = answerEntries.length > 0 && optionIndex === 0;
              console.log(
                `Option ${optionIndex} "${option.text}" isCorrect:`,
                isCorrect,
              );

              return {
                ...option,
                isCorrect,
              };
            }),
          };

          console.log('Updated question:', updatedQuestion);
          return updatedQuestion;
        },
      );

      console.log(
        'Final questions with correct answers:',
        questionsWithCorrectAnswers,
      );
      hasLoadedQuizAnswers.current = true;
      onQuestionsChange(questionsWithCorrectAnswers);
    }
  }, [quizAnswers, isActualOwner, isEditMode, questions, onQuestionsChange]);

  // Reset the loaded flag when switching modes or spaces
  React.useEffect(() => {
    hasLoadedQuizAnswers.current = false;
  }, [spaceId, isActualOwner]); // Reset when space or ownership changes

  // Validation function to check if all questions are answered
  const validateAllQuestionsAnswered = (): boolean => {
    return questions.every((question) =>
      question.options.some((option) => option.isSelected),
    );
  };

  // Handler for quiz submission
  const handleQuizSubmit = async () => {
    console.log('handleQuizSubmit called!');
    if (!onSubmitQuiz) {
      console.warn('No submit handler provided');
      return;
    }

    // For users taking the quiz, validate that all questions are answered
    if (!isOwner && !validateAllQuestionsAnswered()) {
      console.log('Validation failed - not all questions answered');
      showErrorToast('Please answer all questions before submitting.');
      return;
    }

    // Validate that the quiz has correct answers defined (for any user attempting to submit)
    const hasCorrectAnswers = questions.every((question) =>
      question.options.some((option) => option.isCorrect),
    );

    if (!hasCorrectAnswers) {
      showErrorToast(
        'Quiz is not ready for submission. Please contact the space owner.',
      );
      return;
    }

    // For owners, this should not happen as submit should be disabled in edit mode
    if (isOwner && isEditMode) {
      showErrorToast(
        'Please save your quiz before submission becomes available.',
      );
      return;
    }

    console.log('Validation passed, calling onSubmitQuiz...');
    try {
      await onSubmitQuiz(questions);
      showSuccessToast('Quiz submitted successfully!');
      // Manually refresh quiz data after successful submission
      // You may need to import and use the forceRefreshQuizData or refetchQuizData hook here
      // Example (if available):
      // forceRefreshQuizData();
    } catch (error) {
      console.error('Failed to submit quiz:', error);
      showErrorToast('Failed to submit quiz. Please try again.');
    }
  };

  // Handler for adding a new question
  const handleAddQuestion = useCallback(() => {
    if (isQuizEditingDisabled) return; // Block editing when space is InProgress

    const newQuestion: Question = {
      id: `question-${Date.now()}`,
      title: '',
      imageUrl: null,
      options: [
        {
          id: `option-${Date.now()}-1`,
          text: 'Option 1',
          isCorrect: false,
          isSelected: false,
        },
        {
          id: `option-${Date.now()}-2`,
          text: 'Option 2',
          isCorrect: false,
          isSelected: false,
        },
      ],
    };

    onQuestionsChange([...questions, newQuestion]);
  }, [questions, onQuestionsChange, isQuizEditingDisabled]);

  // Handler for removing a question
  const handleRemoveQuestion = useCallback(
    (questionId: string) => {
      if (isQuizEditingDisabled) return; // Block editing when space is InProgress
      onQuestionsChange(questions.filter((q) => q.id !== questionId));
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  // Handler for updating a question title
  const handleUpdateQuestionTitle = useCallback(
    (questionId: string, title: string) => {
      if (isQuizEditingDisabled) return; // Block editing when space is InProgress
      onQuestionsChange(
        questions.map((q) => (q.id === questionId ? { ...q, title } : q)),
      );
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  // Handler for updating a question image
  const handleUpdateQuestionImage = useCallback(
    (questionId: string, imageUrl: string | null) => {
      if (isQuizEditingDisabled) return; // Block editing when space is InProgress
      onQuestionsChange(
        questions.map((q) => (q.id === questionId ? { ...q, imageUrl } : q)),
      );
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  // Handler for adding an option to a question, limited to 4 options
  const handleAddOption = useCallback(
    (questionId: string) => {
      if (isQuizEditingDisabled) return; // Block editing when space is InProgress
      onQuestionsChange(
        questions.map((q) => {
          if (q.id === questionId) {
            // Only add new option if less than 4 options exist
            if (q.options.length >= 4) return q;

            return {
              ...q,
              options: [
                ...q.options,
                {
                  id: `option-${Date.now()}`,
                  text: 'New Option',
                  isCorrect: false,
                  isSelected: false,
                },
              ],
            };
          }
          return q;
        }),
      );
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  // Handler for toggling option selection in read mode (for user answers)
  const handleToggleSelected = useCallback(
    (questionId: string, optionId: string) => {
      if (isEditMode) return; // Only allow selection in read mode

      onQuestionsChange(
        questions.map((q) =>
          q.id === questionId
            ? {
                ...q,
                options: q.options.map((o) => ({
                  ...o,
                  isSelected: o.id === optionId ? !o.isSelected : false, // Single selection per question
                })),
              }
            : q,
        ),
      );
    },
    [questions, onQuestionsChange, isEditMode],
  );

  // Handler for removing an option from a question
  const handleRemoveOption = useCallback(
    (questionId: string, optionId: string) => {
      if (isQuizEditingDisabled) return; // Block editing when space is InProgress
      onQuestionsChange(
        questions.map((q) => {
          if (q.id === questionId) {
            return {
              ...q,
              options: q.options.filter((o) => o.id !== optionId),
            };
          }
          return q;
        }),
      );
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  // Handler for updating an option
  const handleUpdateOption = useCallback(
    (questionId: string, optionId: string, text: string) => {
      if (isQuizEditingDisabled) return; // Block editing when space is InProgress
      onQuestionsChange(
        questions.map((q) => {
          if (q.id === questionId) {
            return {
              ...q,
              options: q.options.map((o) =>
                o.id === optionId ? { ...o, text } : o,
              ),
            };
          }
          return q;
        }),
      );
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  // Handler for toggling an option's correctness (only one option can be correct per question)
  const handleToggleCorrect = useCallback(
    (questionId: string, optionId: string) => {
      if (isQuizEditingDisabled) return; // Block editing when space is InProgress
      onQuestionsChange(
        questions.map((q) => {
          if (q.id === questionId) {
            return {
              ...q,
              options: q.options.map((o) => ({
                ...o,
                isCorrect: o.id === optionId ? !o.isCorrect : false, // Only one option can be correct
              })),
            };
          }
          return q;
        }),
      );
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  // Handler for image upload success from FileUploader
  const handleImageUploadSuccess = (questionId: string, url: string) => {
    handleUpdateQuestionImage(questionId, url);
  };

  // Handle drag start event
  const handleDragStart = (event: DragStartEvent) => {
    const { active } = event;

    // Check if this is an option being dragged
    const idParts = active.id.toString().split('-option-');
    if (idParts.length > 1) {
      const questionId = idParts[0];
      const optionId = `option-${idParts[1]}`;
      setActiveOptionId({ questionId, optionId });
    }
  };

  // Handle drag end event for questions and options
  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;

    if (over && active.id !== over.id) {
      // Handle question reordering
      if (
        typeof active.id === 'string' &&
        typeof over.id === 'string' &&
        !active.id.includes('-option-') &&
        !over.id.includes('-option-')
      ) {
        const currentQuestions = [...questions];
        const oldIndex = currentQuestions.findIndex(
          (item) => item.id === active.id,
        );
        const newIndex = currentQuestions.findIndex(
          (item) => item.id === over.id,
        );
        const reorderedQuestions = arrayMove(
          currentQuestions,
          oldIndex,
          newIndex,
        );
        onQuestionsChange(reorderedQuestions);
      }
      // Handle option reordering within questions
      else if (
        activeOptionId &&
        typeof over.id === 'string' &&
        over.id.includes('-option-')
      ) {
        const questionId = activeOptionId.questionId;
        const optionIdParts = over.id.toString().split('-option-');
        if (optionIdParts.length > 1) {
          const overQuestionId = optionIdParts[0];

          // Only reorder if within the same question
          if (questionId === overQuestionId) {
            const currentQuestions = [...questions];
            const questionIndex = currentQuestions.findIndex(
              (q) => q.id === questionId,
            );
            const question = currentQuestions[questionIndex];

            const activeOptionIdFull = `${questionId}-option-${activeOptionId.optionId.split('option-')[1]}`;
            const overOptionIdFull = over.id.toString();

            const activeOptionIndex = question.options.findIndex(
              (opt) =>
                `${questionId}-option-${opt.id.split('option-')[1]}` ===
                activeOptionIdFull,
            );
            const overOptionIndex = question.options.findIndex(
              (opt) =>
                `${questionId}-option-${opt.id.split('option-')[1]}` ===
                overOptionIdFull,
            );

            const reorderedOptions = arrayMove(
              question.options,
              activeOptionIndex,
              overOptionIndex,
            );

            const updatedQuestions = [...currentQuestions];
            updatedQuestions[questionIndex] = {
              ...question,
              options: reorderedOptions,
            };

            onQuestionsChange(updatedQuestions);
          }
        }
      }
    }

    setActiveOptionId(null);
  };

  // Handler for opening the submit confirmation
  const handleSubmitClick = () => {
    console.log('Submit button clicked!');
    console.log('Questions when submitting:', questions);
    console.log('All questions answered?', validateAllQuestionsAnswered());
    popup
      .open(
        <QuizSubmitForm
          onSubmit={() => {
            console.log('Quiz submit form confirmed!');
            handleQuizSubmit();
            popup.close();
          }}
          onClose={() => popup.close()}
        />,
      )
      .withoutClose()
      .withoutBackdropClose();
  };

  // Get the items we're working with (questions)
  const itemIds = questions.map((item) => item.id);

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={closestCenter}
      modifiers={[restrictToVerticalAxis, restrictToParentElement]}
      onDragStart={handleDragStart}
      onDragEnd={handleDragEnd}
    >
      <div className="text-white w-full flex flex-col">
        {/* Attempt Header - Show in read mode when there are questions, user info is available, and user is not the owner */}
        {!isEditMode &&
          questions.length > 0 &&
          spaceId &&
          userId &&
          !isOwner && (
            <div className="mb-6 pt-4">
              <h3 className="text-xl font-semibold text-white">
                Attempt #{nextAttemptNumber > 3 ? 3 : nextAttemptNumber}
                {nextAttemptNumber > 3 && (
                  <span className="text-red-400 text-sm ml-2">
                    (Max reached)
                  </span>
                )}
              </h3>
            </div>
          )}

        {/* Questions List */}
        {questions.length > 0 ? (
          <SortableContext
            items={itemIds}
            strategy={verticalListSortingStrategy}
          >
            <div>
              {questions.map((question) => (
                <QuestionCard
                  key={question.id}
                  question={question}
                  isEditMode={isEditMode}
                  isQuizEditingDisabled={isQuizEditingDisabled}
                  onRemove={() => handleRemoveQuestion(question.id)}
                  onUpdateTitle={(title) =>
                    handleUpdateQuestionTitle(question.id, title)
                  }
                  onUpdateImage={(url) =>
                    handleUpdateQuestionImage(question.id, url)
                  }
                  onImageUploadSuccess={(url) =>
                    handleImageUploadSuccess(question.id, url)
                  }
                  onAddOption={() => handleAddOption(question.id)}
                  onRemoveOption={(optionId) =>
                    handleRemoveOption(question.id, optionId)
                  }
                  onUpdateOption={(optionId, text) =>
                    handleUpdateOption(question.id, optionId, text)
                  }
                  onToggleCorrect={(optionId) =>
                    handleToggleCorrect(question.id, optionId)
                  }
                  onToggleSelected={(optionId) =>
                    handleToggleSelected(question.id, optionId)
                  }
                />
              ))}
            </div>
          </SortableContext>
        ) : (
          <div className="bg-[var(--color-component-bg)] rounded-[10px] p-4 mb-4 text-center py-8 text-white/70">
            No quiz questions yet. Click the button below to add your first
            question.
          </div>
        )}

        {/* Add Question Button */}
        {isEditMode && !isQuizEditingDisabled && (
          <div className="rounded-[10px] p-4">
            <div className="w-full relative flex items-center justify-center">
              {/* Dotted line across the full width */}
              <div className="absolute w-full border-t border-dashed border-[var(--color-neutral-500)]"></div>

              {/* Circular add button in the middle */}
              <button
                onClick={handleAddQuestion}
                className="relative z-10 flex items-center justify-center h-12 w-12 rounded-full border border-[var(--color-neutral-500)] bg-[var(--color-background)] hover:bg-[var(--color-btn-hover)] transition-colors"
                aria-label="Add question"
              >
                <div style={{ color: 'var(--color-neutral-500)' }}>
                  <Add className="w-10 h-10 stroke-[currentColor]" />
                </div>
              </button>
            </div>
          </div>
        )}

        {/* Submit Button or Max Attempts Message - Show in read mode when there are questions and submit handler is available */}
        {!isEditMode && questions.length > 0 && onSubmitQuiz && !isOwner && (
          <div className="flex justify-end mt-4">
            {latestAttempt && latestAttempt.is_successful ? (
              // Show "Passed" when last attempt was successful - this takes priority over everything else
              <div className="px-6 py-[14.5px] bg-green-600/20 border border-green-500/30 font-semibold text-green-400 text-base rounded-[10px]">
                ✓ Passed
              </div>
            ) : nextAttemptNumber > 3 ? (
              // Show max attempts reached only if not passed
              <div className="px-6 py-[14.5px] bg-red-600/20 border border-red-500/30 font-semibold text-red-400 text-base rounded-[10px]">
                Maximum attempts reached (3/3)
              </div>
            ) : (
              // Show submit button for new attempts
              <button
                onClick={handleSubmitClick}
                className="px-6 py-[14.5px] bg-primary font-bold text-black text-base rounded-[10px] hover:bg-primary/90 transition-colors"
              >
                Submit
              </button>
            )}
          </div>
        )}
      </div>

      {/* Drag overlay for showing the item being dragged */}
      <DragOverlay adjustScale={true} />
    </DndContext>
  );
}

// Question Card Component
function QuestionCard({
  question,
  isEditMode,
  isQuizEditingDisabled,
  onRemove,
  onUpdateTitle,
  onUpdateImage,
  onImageUploadSuccess,
  onAddOption,
  onRemoveOption,
  onUpdateOption,
  onToggleCorrect,
  onToggleSelected,
}: {
  question: Question;
  isEditMode: boolean;
  isQuizEditingDisabled: boolean;
  onRemove: () => void;
  onUpdateTitle: (title: string) => void;
  onUpdateImage: (url: string | null) => void;
  onImageUploadSuccess: (url: string) => void;
  onAddOption: () => void;
  onRemoveOption: (optionId: string) => void;
  onUpdateOption: (optionId: string, text: string) => void;
  onToggleCorrect: (optionId: string) => void;
  onToggleSelected: (optionId: string) => void;
}) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({
    id: question.id,
    disabled: !isEditMode || isQuizEditingDisabled,
  });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  // Create sortable context for options
  const optionIds = question.options.map(
    (opt) => `${question.id}-option-${opt.id.split('option-')[1]}`,
  );

  return (
    <div
      ref={setNodeRef}
      style={{
        ...style,
        backgroundColor: 'var(--color-component-bg)',
      }}
      className={`rounded-[10px] p-4 mb-4 ${isEditMode && !isQuizEditingDisabled ? 'cursor-move' : ''}`}
      {...(isEditMode && !isQuizEditingDisabled ? attributes : {})}
      {...(isEditMode && !isQuizEditingDisabled ? listeners : {})}
    >
      {/* Drag handle for question - centered above title */}
      {isEditMode && !isQuizEditingDisabled && (
        <div className="flex justify-center mb-2">
          <div className="text-white/50 cursor-move">
            <DialPad className="w-5 h-5" />
          </div>
        </div>
      )}

      {/* Question Header */}
      <div className="flex items-center gap-2 mb-4">
        {isEditMode ? (
          <>
            <input
              type="text"
              value={question.title}
              onChange={(e) => onUpdateTitle(e.target.value)}
              disabled={isQuizEditingDisabled}
              style={{
                backgroundColor: 'var(--color-neutral-800)',
                border: '1px solid var(--color-neutral-700)',
                height: '40px',
                opacity: isQuizEditingDisabled ? 0.5 : 1,
              }}
              className="title-input flex-1 px-3 rounded-md text-white focus:outline-none focus:ring-1 focus:ring-white/50"
              placeholder="Title"
              spellCheck={false}
            />
            <FileUploader
              onUploadSuccess={onImageUploadSuccess}
              style={{
                border: '1px solid var(--color-neutral-700)',
                height: '40px',
                width: '40px',
                opacity: isQuizEditingDisabled ? 0.5 : 1,
                pointerEvents: isQuizEditingDisabled ? 'none' : 'auto',
              }}
              className="rounded-md flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-white/50"
            >
              <Image2 className="w-5 h-5 stroke-[var(--color-neutral-500)]" />
            </FileUploader>
          </>
        ) : (
          <h3 className="text-lg font-medium w-full">{question.title}</h3>
        )}
      </div>

      {/* Image display */}
      {question.imageUrl && (
        <div className="mb-4">
          <div className="relative">
            <Image
              src={question.imageUrl}
              alt={question.title}
              width={500}
              height={300}
              className="w-full h-auto rounded-md object-contain"
            />
            {isEditMode && !isQuizEditingDisabled && (
              <button
                onClick={() => onUpdateImage(null)}
                className="absolute top-2 right-2 bg-black/60 rounded-full p-1 text-white/90 hover:bg-black/80"
                aria-label="Remove image"
              >
                <svg
                  className="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              </button>
            )}
          </div>
        </div>
      )}

      {/* Options List */}
      <SortableContext items={optionIds} strategy={verticalListSortingStrategy}>
        <div className="space-y-2">
          {question.options.map((option) => (
            <OptionItem
              key={`${question.id}-option-${option.id.split('option-')[1]}`}
              id={`${question.id}-option-${option.id.split('option-')[1]}`}
              questionId={question.id}
              option={option}
              isEditMode={isEditMode}
              isQuizEditingDisabled={isQuizEditingDisabled}
              onUpdateText={(text) => onUpdateOption(option.id, text)}
              onToggleCorrect={() => onToggleCorrect(option.id)}
              onToggleSelected={() => onToggleSelected(option.id)}
              onRemove={() => onRemoveOption(option.id)}
            />
          ))}
        </div>
      </SortableContext>

      {/* Add Option and Delete Buttons on the same line */}
      {isEditMode && (
        <div className="mt-4 flex justify-between items-center">
          {/* Only show Add Option button if less than 4 options */}
          {question.options.length < 4 ? (
            <button
              onClick={onAddOption}
              disabled={isQuizEditingDisabled}
              className="flex items-center text-sm"
              style={{
                color: 'var(--color-neutral-500)',
                opacity: isQuizEditingDisabled ? 0.3 : 1,
                cursor: isQuizEditingDisabled ? 'not-allowed' : 'pointer',
              }}
            >
              <div className="mr-1.5">
                <Add className="w-5 h-5 stroke-[currentColor]" />
              </div>
              Add Option
            </button>
          ) : (
            <span className="text-sm text-[var(--color-neutral-500)]">
              Maximum 4 options
            </span>
          )}

          <button
            onClick={onRemove}
            disabled={isQuizEditingDisabled}
            className="flex items-center text-sm"
            style={{
              color: 'var(--color-neutral-500)',
              opacity: isQuizEditingDisabled ? 0.3 : 1,
              cursor: isQuizEditingDisabled ? 'not-allowed' : 'pointer',
            }}
          >
            Delete
            <div className="ml-1.5">
              <Delete2 className="w-5 h-5 stroke-[currentColor]" />
            </div>
          </button>
        </div>
      )}
    </div>
  );
}

// Option Item Component
function OptionItem({
  id,
  questionId: _questionId,
  option,
  isEditMode,
  isQuizEditingDisabled,
  onUpdateText,
  onToggleCorrect,
  onToggleSelected,
  onRemove,
}: {
  id: string;
  questionId: string;
  option: Option;
  isEditMode: boolean;
  isQuizEditingDisabled: boolean;
  onUpdateText: (text: string) => void;
  onToggleCorrect: () => void;
  onToggleSelected: () => void;
  onRemove: () => void;
}) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({
    id,
    disabled: !isEditMode || isQuizEditingDisabled,
  });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  // Acknowledge unused parameter to satisfy linting rules
  void _questionId;

  return (
    <div
      ref={setNodeRef}
      style={style}
      className={`flex items-center gap-2 p-2 rounded ${
        isEditMode && !isQuizEditingDisabled ? 'cursor-move' : ''
      }`}
      {...(isEditMode && !isQuizEditingDisabled ? attributes : {})}
      {...(isEditMode && !isQuizEditingDisabled ? listeners : {})}
    >
      {/* Drag handle for edit mode */}
      {isEditMode && !isQuizEditingDisabled && (
        <div className="text-white/50 cursor-move mr-1">
          <DialPad2 className="w-4 h-4" />
        </div>
      )}

      {/* Checkbox for correct answer in edit mode, or user selection in read mode */}
      <div
        className={`w-5 h-5 flex-shrink-0 border rounded flex items-center justify-center ${
          isQuizEditingDisabled
            ? 'cursor-not-allowed opacity-50'
            : 'cursor-pointer'
        } ${
          isEditMode
            ? option.isCorrect
              ? 'bg-[var(--color-primary)] border-[var(--color-primary)]'
              : 'border-white/30'
            : option.isSelected
              ? 'bg-[var(--color-primary)] border-[var(--color-primary)]'
              : 'border-white/30 hover:border-white/50'
        }`}
        onClick={
          isQuizEditingDisabled
            ? undefined
            : isEditMode
              ? onToggleCorrect
              : onToggleSelected
        }
        role="button"
        tabIndex={isQuizEditingDisabled ? -1 : 0}
        aria-label={
          isEditMode ? 'Mark as correct answer' : 'Select this option'
        }
      >
        {(isEditMode ? option.isCorrect : option.isSelected) && (
          <svg
            className="w-3 h-3 text-black"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={3}
              d="M5 13l4 4L19 7"
            />
          </svg>
        )}
      </div>

      {/* Option text */}
      {isEditMode ? (
        <input
          type="text"
          value={option.text}
          onChange={(e) => onUpdateText(e.target.value)}
          disabled={isQuizEditingDisabled}
          className="flex-1 bg-transparent border-b-0 px-2 py-1 text-white focus:outline-none focus:border-b-2 focus:border-white"
          style={{ opacity: isQuizEditingDisabled ? 0.5 : 1 }}
          placeholder="Option text"
        />
      ) : (
        <span className="flex-1">{option.text}</span>
      )}

      {/* Delete option button */}
      {isEditMode && !isQuizEditingDisabled && (
        <button
          onClick={onRemove}
          aria-label="Remove option"
          style={{ color: 'var(--color-neutral-500)' }}
        >
          <Remove className="w-5 h-5 stroke-[currentColor]" />
        </button>
      )}
    </div>
  );
}
