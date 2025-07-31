'use client';
import React from 'react';
import { SpaceContextType } from '../../type';
import { logger } from '@/lib/logger';
import SummaryReport from '../dashboard/summary-report';
import ObjectiveResponse from '../dashboard/objective-response';
import SubjectiveResponse from '../dashboard/subjective-response';

enum AnswerType {
  SingleChoice = 'single_choice',
  MultipleChoice = 'multiple_choice',
  ShortAnswer = 'short_answer',
  Subjective = 'subjective',
  Checkbox = 'checkbox',
  Dropdown = 'dropdown',
  LinearScale = 'linear_scale',
}

export default function AnalyzePage({
  context,
}: {
  context: SpaceContextType;
}) {
  const { handleDownloadExcel, answers, survey, mappedResponses } = context;

  logger.debug('mapped responses: ', mappedResponses);

  const responseCount = answers.length;
  const startDate =
    survey.surveys.length > 0 ? survey.surveys[0].started_at : 0;
  const endDate = survey.surveys.length > 0 ? survey.surveys[0].ended_at : 0;

  return (
    <div className="flex flex-col w-full">
      <div className="flex flex-row w-full justify-end mb-[20px]">
        <div className="w-fit">
          <button
            className="w-full px-[20px] py-[10px] rounded-[10px] bg-[#fcb300] hover:bg-[#ca8f00] text-black text-bold text-[16px] hover:text-black cursor-pointer"
            disabled={false}
            onClick={() => {
              handleDownloadExcel();
            }}
          >
            {'Download Excel'}
          </button>
        </div>
      </div>

      <div className="flex flex-col w-full gap-2.5">
        {responseCount > 0 && (
          <SummaryReport
            responseCount={responseCount}
            startDate={startDate}
            endDate={endDate}
          />
        )}
        {mappedResponses.map((res, index) => {
          const type = res.question.answer_type;
          return type === AnswerType.MultipleChoice ||
            type === AnswerType.SingleChoice ||
            type === AnswerType.Checkbox ||
            type === AnswerType.Dropdown ||
            type === AnswerType.LinearScale ? (
            <ObjectiveResponse
              key={`objective-question-${index}`}
              question={res.question}
              answers={res.answers}
            />
          ) : (
            <SubjectiveResponse
              key={`subjective-question-${index}`}
              question={res.question}
              answers={res.answers}
            />
          );
        })}
      </div>
    </div>
  );
}
