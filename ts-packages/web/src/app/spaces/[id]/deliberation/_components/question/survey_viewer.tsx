import React from 'react';
import BlackBox from '@/app/(social)/_components/black-box';
import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { SurveyAnswer } from '../../types';
import { Answer } from '@/lib/api/models/response';
import { usePopup } from '@/lib/contexts/popup-service';
import CheckPopup from './check_popup';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { logger } from '@/lib/logger';
import Image from 'next/image';

interface Question {
  title: string;
  answer_type: Answer['answer_type'];
  image_url?: string;
  is_multi?: boolean;
  options?: string[];
}

interface SurveyViewerProps {
  isEdit: boolean;
  status: SpaceStatus;
  startDate: number;
  endDate: number;
  questions: Question[];
  answer: SurveyAnswer;
  setAnswers: (answer: Answer[]) => void;
  onConfirm: () => void;
}

export default function SurveyViewer({
  isEdit,
  status,
  startDate,
  endDate,
  questions,
  answer,
  setAnswers,
  onConfirm,
}: SurveyViewerProps) {
  const now = Math.floor(Date.now() / 1000);
  const isLive = startDate <= now && now <= endDate;
  const popup = usePopup();
  const is_completed = answer.is_completed;
  const answers: Answer[] = answer.answers;

  logger.debug(
    'is completed:',
    is_completed,
    ' status:',
    status,
    'isLive:',
    isLive,
  );

  const handleSelect = (
    qIdx: number,
    optionIdx: number,
    type: Question['answer_type'],
  ) => {
    if (is_completed) return;

    const updated = [...answers];

    if (type === 'single_choice') {
      updated[qIdx] = {
        answer_type: 'single_choice',
        answer: optionIdx,
      } satisfies Answer;
    } else if (type === 'checkbox') {
      if (questions[qIdx].is_multi) {
        const existing =
          answers[qIdx]?.answer_type === 'checkbox'
            ? [
                ...(
                  answers[qIdx] as Extract<Answer, { answer_type: 'checkbox' }>
                ).answer,
              ]
            : [];
        const exists = existing.includes(optionIdx);
        const newAnswer = exists
          ? existing.filter((v) => v !== optionIdx)
          : [...existing, optionIdx];

        updated[qIdx] = {
          answer_type: 'checkbox',
          answer: newAnswer,
        } satisfies Answer;
      } else {
        updated[qIdx] = {
          answer_type: 'checkbox',
          answer: [optionIdx],
        } satisfies Answer;
      }
    } else if (type === 'multiple_choice') {
      const existing =
        answers[qIdx]?.answer_type === 'multiple_choice'
          ? [
              ...(
                answers[qIdx] as Extract<
                  Answer,
                  { answer_type: 'multiple_choice' }
                >
              ).answer,
            ]
          : [];
      const exists = existing.includes(optionIdx);
      const newAnswer = exists
        ? existing.filter((v) => v !== optionIdx)
        : [...existing, optionIdx];

      updated[qIdx] = {
        answer_type: 'multiple_choice',
        answer: newAnswer,
      } satisfies Answer;
    }

    setAnswers(updated);
  };

  const handleInput = (
    qIdx: number,
    value: string,
    type: 'short_answer' | 'subjective',
  ) => {
    if (is_completed) return;

    const updated = [...answers];
    updated[qIdx] = {
      answer_type: type,
      answer: value,
    } satisfies Answer;

    setAnswers(updated);
  };

  return (
    <div className="flex flex-col gap-2.5 w-full">
      {questions.map((q, index) => {
        const selected = answers[index];

        let selectedIndexes =
          q.answer_type === 'checkbox' && selected?.answer_type === 'checkbox'
            ? selected.answer
            : [];

        if (selectedIndexes.length === 0) {
          selectedIndexes =
            q.answer_type === 'multiple_choice' &&
            selected?.answer_type === 'multiple_choice'
              ? selected.answer
              : [];
        }

        return (
          <BlackBox key={index}>
            <div className="flex flex-col w-full gap-2.5">
              {(q.answer_type === 'single_choice' ||
                q.answer_type === 'multiple_choice' ||
                q.answer_type == 'checkbox') && (
                <>
                  <div className="flex flex-row w-full mt-[7px] mb-[15px] font-semibold text-base/[22.5px] text-white gap-1">
                    <div className="text-blue-500">
                      {q.answer_type === 'single_choice' ||
                      (q.answer_type === 'checkbox' && !q.is_multi)
                        ? '[Single Choice]'
                        : '[Multiple Choice]'}
                    </div>
                    <div>{q.title}</div>
                  </div>
                  {q.image_url ? (
                    <Image
                      width={700}
                      height={280}
                      className="object-contain max-h-70 w-fit rounded-lg"
                      src={q.image_url}
                      alt={q.title || 'Question Title'}
                    />
                  ) : (
                    <></>
                  )}
                  <div className="flex flex-col gap-2">
                    {q.options?.map((opt, idx) => {
                      let isChecked = selectedIndexes.includes(idx);

                      if (!isChecked) {
                        isChecked =
                          q.answer_type === 'single_choice'
                            ? selected?.answer === idx
                            : selectedIndexes.includes(idx);
                      }

                      return (
                        <div
                          key={`${q.answer_type}-${index}-${idx}`}
                          className="flex flex-row w-full h-fit justify-start items-center gap-3"
                        >
                          <div className="w-4.5 h-4.5">
                            <CustomCheckbox
                              checked={isChecked}
                              onChange={() =>
                                handleSelect(index, idx, q.answer_type)
                              }
                              disabled={is_completed}
                            />
                          </div>
                          <div className="font-normal text-neutral-300 text-[15px]/[22.5px]">
                            {opt}
                          </div>
                        </div>
                      );
                    })}
                  </div>
                </>
              )}

              {(q.answer_type === 'short_answer' ||
                q.answer_type === 'subjective') && (
                <div className="flex flex-col w-full gap-[10px]">
                  <div className="flex flex-row w-full mt-[7px] mb-[15px] font-semibold text-base/[22.5px] text-white gap-[4px]">
                    <div className="text-[#ff6467]">[Required]</div>
                    <div>{q.title}</div>
                  </div>
                  {q.answer_type === 'short_answer' ? (
                    <Input
                      type="text"
                      placeholder="Please share your opinion."
                      className="bg-neutral-800 border border-neutral-700 text-base text-white placeholder:text-neutral-600 px-4 py-3 rounded-lg focus:outline-none focus:border-yellow-500"
                      value={
                        selected?.answer_type === 'short_answer'
                          ? selected.answer
                          : ''
                      }
                      onChange={(e) =>
                        handleInput(index, e.target.value, 'short_answer')
                      }
                      disabled={is_completed}
                    />
                  ) : (
                    <Textarea
                      rows={7}
                      placeholder="Please share your opinion."
                      className="bg-neutral-800 min-h-[185px] border border-neutral-700 text-base text-white placeholder:text-neutral-600 px-4 py-3 rounded-lg focus:outline-none focus:border-yellow-500"
                      value={
                        selected?.answer_type === 'subjective'
                          ? selected.answer
                          : ''
                      }
                      onChange={(e) =>
                        handleInput(index, e.target.value, 'subjective')
                      }
                      disabled={is_completed}
                    />
                  )}
                </div>
              )}
            </div>
          </BlackBox>
        );
      })}

      <div
        className={`flex flex-row w-full justify-end ${is_completed || status != SpaceStatus.InProgress || isEdit || !isLive || questions.length == 0 ? 'hidden' : ''}`}
      >
        <div
          className="cursor-pointer flex flex-row w-[180px] h-fit py-[14px] px-[40px] justify-center items-center bg-primary hover:opacity-70 rounded-lg font-bold text-[15px] text-[#000203]"
          onClick={() => {
            popup
              .open(
                <CheckPopup
                  onContinue={() => {
                    onConfirm();
                    popup.close();
                  }}
                  onClose={() => {
                    popup.close();
                  }}
                />,
              )
              .withTitle('Please check again before voting.');
          }}
        >
          Save
        </div>
      </div>
    </div>
  );
}
