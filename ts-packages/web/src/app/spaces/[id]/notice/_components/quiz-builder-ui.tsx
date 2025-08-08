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
import Heart from '@/assets/icons/emoji/heart.svg';
import BrokenHeart from '@/assets/icons/emoji/broken-heart.svg';
import { QuizQuestion, NoticeQuizRequest } from '@/lib/api/models/notice';
import { SpaceStatus } from '@/lib/api/models/spaces';
import Image from 'next/image';
import FileUploader from '@/components/file-uploader';
import { usePopup } from '@/lib/contexts/popup-service';
import { showErrorToast } from '@/lib/toast';
import {
  useLatestQuizAttempt,
  useQuizAttempts,
  useQuizAnswers,
} from '@/lib/api/ratel_api';
import QuizSubmitForm from './modal/quiz-submit-form';

export interface Option {
  id: string;
  text: string;
  isCorrect: boolean;
  isSelected: boolean;
}

export interface Question {
  id: string;
  title: string;
  imageUrls: string[];
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

export function convertQuizQuestionToQuestion(
  quizQuestion: QuizQuestion,
  id: string,
): Question {
  return {
    id: quizQuestion.id || id,
    title: quizQuestion.title,
    imageUrls: quizQuestion.images
      .map((img) => img.url)
      .filter((url): url is string => url !== null),
    options: quizQuestion.options.map((option, index) => ({
      id: option.id || `option-${Date.now()}-${index}`,
      text: option.content,
      isCorrect: false,
      isSelected: false,
    })),
  };
}

export function convertQuestionsToNoticeQuizRequest(
  questions: Question[],
): NoticeQuizRequest {
  return {
    questions: questions.map((question) => ({
      title: question.title,
      images: question.imageUrls,
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
  const { data: attemptsData } = useQuizAttempts(spaceId || 0);

  const { data: latestAttempt } = useLatestQuizAttempt(spaceId || 0);

  const { data: quizAnswers, isError: quizAnswersIsError } = useQuizAnswers(
    spaceId || 0,
    !!(spaceId && spaceId > 0),
  );

  // Function to render heart icons based on attempt count (only for non-owners)
  const renderHeartIcons = (attemptCount: number) => {
    if (isOwner) return null;

    const heartIcons = [];

    // Add broken hearts for each attempt (max 2)
    const brokenHeartCount = Math.min(attemptCount, 2);
    for (let i = 0; i < brokenHeartCount; i++) {
      heartIcons.push(
        <BrokenHeart
          key={`broken-${i}`}
          width={25}
          height={25}
          className="text-neutral-500"
        />,
      );
    }

    // Always add one heart at the end for current attempt
    heartIcons.push(
      <Heart key="heart" width={25} height={25} className="text-red-500" />,
    );

    return <div className="flex items-center gap-1">{heartIcons}</div>;
  };

  const isActualOwner = !!quizAnswers && !quizAnswersIsError;

  const failedAttempts =
    attemptsData?.items?.filter((attempt) => !attempt.is_successful) || [];
  const nextAttemptNumber = failedAttempts.length + 1;

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

  const hasLoadedQuizAnswers = React.useRef(false);

  React.useEffect(() => {
    if (
      isActualOwner &&
      isEditMode &&
      questions.length > 0 &&
      quizAnswers?.answers?.answers &&
      !hasLoadedQuizAnswers.current
    ) {
      const correctAnswersMap = quizAnswers.answers.answers;

      const questionsWithCorrectAnswers: Question[] = questions.map(
        (question) => {
          const questionCorrectOptionIds: string[] =
            correctAnswersMap[question.id] || [];

          const updatedQuestion = {
            ...question,
            options: question.options.map((option) => {
              const isCorrect = questionCorrectOptionIds.includes(option.id);
              return { ...option, isCorrect };
            }),
          };

          return updatedQuestion;
        },
      );

      hasLoadedQuizAnswers.current = true;
      onQuestionsChange(questionsWithCorrectAnswers);
    }
  }, [quizAnswers, isActualOwner, isEditMode, questions, onQuestionsChange]);

  React.useEffect(() => {
    hasLoadedQuizAnswers.current = false;
  }, [spaceId, isActualOwner]);

  // Validation function to check if all questions are answered
  const validateAllQuestionsAnswered = (): boolean => {
    return questions.every((question) =>
      question.options.some((option) => option.isSelected),
    );
  };

  // Handler for quiz submission
  const handleQuizSubmit = async () => {
    if (!onSubmitQuiz) return;

    if (!isOwner && !validateAllQuestionsAnswered()) {
      showErrorToast('Please answer all questions before submitting.');
      return;
    }

    if (isOwner && isEditMode) {
      const hasCorrectAnswers = questions.every((question) =>
        question.options.some((option) => option.isCorrect),
      );

      if (!hasCorrectAnswers) {
        showErrorToast(
          'Please define correct answers for all questions before submission.',
        );
        return;
      }

      showErrorToast(
        'Please save your quiz before submission becomes available.',
      );
      return;
    }

    try {
      await onSubmitQuiz(questions);
      // Success notification is handled by the provider
    } catch {
      showErrorToast('Failed to submit quiz. Please try again.');
    }
  };

  const handleAddQuestion = useCallback(() => {
    if (isQuizEditingDisabled) return;

    const newQuestion: Question = {
      id: `question-${Date.now()}`,
      title: '',
      imageUrls: [],
      options: [
        {
          id: `option-${Date.now()}-1`,
          text: '',
          isCorrect: false,
          isSelected: false,
        },
        {
          id: `option-${Date.now()}-2`,
          text: '',
          isCorrect: false,
          isSelected: false,
        },
      ],
    };

    onQuestionsChange([...questions, newQuestion]);
  }, [questions, onQuestionsChange, isQuizEditingDisabled]);

  const handleRemoveQuestion = useCallback(
    (questionId: string) => {
      if (isQuizEditingDisabled) return;
      onQuestionsChange(questions.filter((q) => q.id !== questionId));
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  const handleUpdateQuestionTitle = useCallback(
    (questionId: string, title: string) => {
      if (isQuizEditingDisabled) return;
      onQuestionsChange(
        questions.map((q) => (q.id === questionId ? { ...q, title } : q)),
      );
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  const handleAddQuestionImage = useCallback(
    (questionId: string, imageUrl: string) => {
      if (isQuizEditingDisabled) return;
      onQuestionsChange(
        questions.map((q) => {
          if (q.id === questionId) {
            // Limit to maximum 2 images
            if (q.imageUrls.length >= 2) {
              showErrorToast('Maximum 2 images allowed per question');
              return q;
            }
            return { ...q, imageUrls: [...q.imageUrls, imageUrl] };
          }
          return q;
        }),
      );
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  const handleRemoveQuestionImage = useCallback(
    (questionId: string, imageIndex: number) => {
      if (isQuizEditingDisabled) return;
      onQuestionsChange(
        questions.map((q) => {
          if (q.id === questionId) {
            const newImageUrls = [...q.imageUrls];
            newImageUrls.splice(imageIndex, 1);
            return { ...q, imageUrls: newImageUrls };
          }
          return q;
        }),
      );
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  const handleAddOption = useCallback(
    (questionId: string) => {
      if (isQuizEditingDisabled) return;
      onQuestionsChange(
        questions.map((q) => {
          if (q.id === questionId) {
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

  const handleToggleSelected = useCallback(
    (questionId: string, optionId: string) => {
      if (isEditMode) return;

      onQuestionsChange(
        questions.map((q) =>
          q.id === questionId
            ? {
                ...q,
                options: q.options.map((o) => ({
                  ...o,
                  isSelected: o.id === optionId ? !o.isSelected : false,
                })),
              }
            : q,
        ),
      );
    },
    [questions, onQuestionsChange, isEditMode],
  );

  const handleRemoveOption = useCallback(
    (questionId: string, optionId: string) => {
      if (isQuizEditingDisabled) return;
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

  const handleUpdateOption = useCallback(
    (questionId: string, optionId: string, text: string) => {
      if (isQuizEditingDisabled) return;
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

  const handleToggleCorrect = useCallback(
    (questionId: string, optionId: string) => {
      if (isQuizEditingDisabled) return;
      onQuestionsChange(
        questions.map((q) => {
          if (q.id === questionId) {
            return {
              ...q,
              options: q.options.map((o) => ({
                ...o,
                isCorrect: o.id === optionId ? !o.isCorrect : false,
              })),
            };
          }
          return q;
        }),
      );
    },
    [questions, onQuestionsChange, isQuizEditingDisabled],
  );

  const handleImageUploadSuccess = (questionId: string, url: string) => {
    handleAddQuestionImage(questionId, url);
  };

  const handleDragStart = (event: DragStartEvent) => {
    const { active } = event;

    const idParts = active.id.toString().split('-option-');
    if (idParts.length > 1) {
      const questionId = idParts[0];
      const optionId = `option-${idParts[1]}`;
      setActiveOptionId({ questionId, optionId });
    }
  };

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;

    if (over && active.id !== over.id) {
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
      } else if (
        activeOptionId &&
        typeof over.id === 'string' &&
        over.id.includes('-option-')
      ) {
        const questionId = activeOptionId.questionId;
        const optionIdParts = over.id.toString().split('-option-');
        if (optionIdParts.length > 1) {
          const overQuestionId = optionIdParts[0];

          if (questionId === overQuestionId) {
            const currentQuestions = [...questions];
            const questionIndex = currentQuestions.findIndex(
              (q) => q.id === questionId,
            );
            const question = currentQuestions[questionIndex];

            const activeOptionIdFull = `${questionId}-option-${
              activeOptionId.optionId.includes('option-')
                ? activeOptionId.optionId.split('option-')[1]
                : activeOptionId.optionId
            }`;
            const overOptionIdFull = over.id.toString();

            const activeOptionIndex = question.options.findIndex((opt) => {
              const optionSuffix = opt.id.includes('option-')
                ? opt.id.split('option-')[1]
                : opt.id;
              return (
                `${questionId}-option-${optionSuffix}` === activeOptionIdFull
              );
            });
            const overOptionIndex = question.options.findIndex((opt) => {
              const optionSuffix = opt.id.includes('option-')
                ? opt.id.split('option-')[1]
                : opt.id;
              return (
                `${questionId}-option-${optionSuffix}` === overOptionIdFull
              );
            });

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

  const handleSubmitClick = () => {
    popup
      .open(
        <QuizSubmitForm
          onSubmit={() => {
            handleQuizSubmit();
            popup.close();
          }}
          onClose={() => popup.close()}
        />,
      )
      .withoutClose()
      .withoutBackdropClose();
  };

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
        {!isEditMode &&
          questions.length > 0 &&
          spaceId &&
          userId &&
          !isOwner && (
            <div className="mb-6 pt-4">
              <h3 className="text-xl font-semibold text-white flex items-center justify-between">
                <div>
                  Attempt #{nextAttemptNumber > 3 ? 3 : nextAttemptNumber}
                  {nextAttemptNumber > 3 && (
                    <span className="text-red-400 text-sm ml-2">
                      (Max reached)
                    </span>
                  )}
                </div>
                {/* Heart icons for non-owners - positioned at extreme right */}
                {attemptsData && renderHeartIcons(attemptsData.total_count)}
              </h3>
            </div>
          )}

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
                  onRemoveImage={(imageIndex) =>
                    handleRemoveQuestionImage(question.id, imageIndex)
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

        {isEditMode && !isQuizEditingDisabled && (
          <div className="rounded-[10px] p-4">
            <div className="w-full relative flex items-center justify-center">
              <div className="absolute w-full border-t border-dashed border-[var(--color-neutral-500)]"></div>

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

        {!isEditMode && questions.length > 0 && onSubmitQuiz && !isOwner && (
          <div className="flex justify-end mt-4">
            {latestAttempt && latestAttempt.is_successful ? (
              <div className="px-6 py-[14.5px] bg-green-600/20 border border-green-500/30 font-semibold text-green-400 text-base rounded-[10px]">
                ✓ Passed
              </div>
            ) : nextAttemptNumber > 3 ? (
              <div className="px-6 py-[14.5px] bg-red-600/20 border border-red-500/30 font-semibold text-red-400 text-base rounded-[10px]">
                Maximum attempts reached (3/3)
              </div>
            ) : (
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

      <DragOverlay adjustScale={true} />
    </DndContext>
  );
}

function QuestionCard({
  question,
  isEditMode,
  isQuizEditingDisabled,
  onRemove,
  onUpdateTitle,
  onRemoveImage,
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
  onRemoveImage: (imageIndex: number) => void;
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

  const optionIds = question.options.map((opt) => {
    const optionSuffix = opt.id.includes('option-')
      ? opt.id.split('option-')[1]
      : opt.id;
    return `${question.id}-option-${optionSuffix}`;
  });

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
      {isEditMode && !isQuizEditingDisabled && (
        <div className="flex justify-center mb-2">
          <div className="text-white/50 cursor-move">
            <DialPad className="w-5 h-5" />
          </div>
        </div>
      )}

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
            <div className="relative">
              <FileUploader
                onUploadSuccess={onImageUploadSuccess}
                style={{
                  border: '1px solid var(--color-neutral-700)',
                  height: '40px',
                  width: '40px',
                  opacity:
                    isQuizEditingDisabled || question.imageUrls.length >= 2
                      ? 0.5
                      : 1,
                  pointerEvents:
                    isQuizEditingDisabled || question.imageUrls.length >= 2
                      ? 'none'
                      : 'auto',
                }}
                className="rounded-md flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-white/50"
              >
                <Image2 className="w-5 h-5 stroke-[var(--color-neutral-500)]" />
              </FileUploader>
              {question.imageUrls.length > 0 && (
                <div className="absolute -top-1 -right-1 bg-primary text-black text-xs rounded-full w-5 h-5 flex items-center justify-center font-semibold">
                  {question.imageUrls.length}
                </div>
              )}
            </div>
          </>
        ) : (
          <h3 className="text-lg font-medium w-full">{question.title}</h3>
        )}
      </div>

      {question.imageUrls && question.imageUrls.length > 0 && (
        <div className="mb-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            {question.imageUrls.map((imageUrl, index) => (
              <div key={`${question.id}-image-${index}`} className="relative">
                <Image
                  src={imageUrl}
                  alt={`${question.title} - Image ${index + 1}`}
                  width={500}
                  height={300}
                  className="w-full h-auto rounded-md object-contain"
                />
                {isEditMode && !isQuizEditingDisabled && (
                  <button
                    onClick={() => onRemoveImage(index)}
                    className="absolute top-2 right-2 bg-black/60 rounded-full p-1 text-white/90 hover:bg-black/80"
                    aria-label={`Remove image ${index + 1}`}
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
            ))}
          </div>
        </div>
      )}

      <SortableContext items={optionIds} strategy={verticalListSortingStrategy}>
        <div className="space-y-2">
          {question.options.map((option) => {
            const optionSuffix = option.id.includes('option-')
              ? option.id.split('option-')[1]
              : option.id;
            const keyId = `${question.id}-option-${optionSuffix}`;

            return (
              <OptionItem
                key={keyId}
                id={keyId}
                questionId={question.id}
                option={option}
                isEditMode={isEditMode}
                isQuizEditingDisabled={isQuizEditingDisabled}
                onUpdateText={(text) => onUpdateOption(option.id, text)}
                onToggleCorrect={() => onToggleCorrect(option.id)}
                onToggleSelected={() => onToggleSelected(option.id)}
                onRemove={() => onRemoveOption(option.id)}
              />
            );
          })}
        </div>
      </SortableContext>

      {isEditMode && (
        <div className="mt-4 flex justify-between items-center">
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

  void _questionId;

  return (
    <div
      ref={isEditMode && !isQuizEditingDisabled ? setNodeRef : undefined}
      style={isEditMode && !isQuizEditingDisabled ? style : undefined}
      className={`flex items-center gap-2 p-2 rounded ${
        isEditMode && !isQuizEditingDisabled ? 'cursor-move' : 'cursor-default'
      }`}
      {...(isEditMode && !isQuizEditingDisabled ? attributes : {})}
      {...(isEditMode && !isQuizEditingDisabled ? listeners : {})}
    >
      {isEditMode && !isQuizEditingDisabled && (
        <div className="text-white/50 cursor-move mr-1">
          <DialPad2 className="w-4 h-4" />
        </div>
      )}

      <div
        className={`w-5 h-5 flex-shrink-0 border rounded flex items-center justify-center ${
          isEditMode && isQuizEditingDisabled
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
          isEditMode && isQuizEditingDisabled
            ? undefined
            : isEditMode
              ? onToggleCorrect
              : onToggleSelected
        }
        onPointerDown={(e) => {
          if (!isEditMode) {
            e.stopPropagation();
          }
        }}
        role="button"
        tabIndex={isEditMode && isQuizEditingDisabled ? -1 : 0}
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
        <span
          className="flex-1 cursor-pointer"
          onClick={onToggleSelected}
          onPointerDown={(e) => e.stopPropagation()}
        >
          {option.text}
        </span>
      )}

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
