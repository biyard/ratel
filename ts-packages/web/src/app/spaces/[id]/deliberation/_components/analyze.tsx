'use client';
import React from 'react';
import { useDeliberationSpaceContext } from '../provider.client';
import ObjectiveResponse from './dashboard/objective_response';
import SubjectiveResponse from './dashboard/subjective_response';
import { logger } from '@/lib/logger';
import SummaryReport from './dashboard/summary_report';

export default function AnalyzePage() {
  const { handleDownloadExcel, answers, survey, mappedResponses } =
    useDeliberationSpaceContext();

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
          return res.question.answer_type === 'multiple_choice' ||
            res.question.answer_type === 'single_choice' ||
            res.question.answer_type === 'checkbox' ||
            res.question.answer_type === 'dropdown' ? (
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
