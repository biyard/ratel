'use client';
import { ArrowLeft } from 'lucide-react';
import React from 'react';
import { useDeliberationSpaceContext } from '../provider.client';
import ObjectiveResponse from './dashboard/objective_response';
import SubjectiveResponse from './dashboard/subjective_response';
import { logger } from '@/lib/logger';

export default function AnalyzePage() {
  const { handleGoBack, handleDownloadExcel, mappedResponses } =
    useDeliberationSpaceContext();

  logger.debug('mapped responses: ', mappedResponses);

  return (
    <div className="flex flex-col w-full">
      <div
        className="cursor-pointer w-fit h-fit mb-[20px]"
        onClick={() => {
          handleGoBack();
        }}
      >
        <ArrowLeft width={24} height={24} />
      </div>

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
        {mappedResponses.map((res) => {
          return res.question.answer_type === 'multiple_choice' ||
            res.question.answer_type === 'single_choice' ? (
            <ObjectiveResponse />
          ) : (
            <SubjectiveResponse question={res.question} answers={res.answers} />
          );
        })}
      </div>
    </div>
  );
}
