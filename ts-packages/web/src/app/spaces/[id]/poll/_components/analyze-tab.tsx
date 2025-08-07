'use client';
import React from 'react';
import { usePollSpaceContext } from '../provider.client';
import { AnalyzePage } from '../../_components/page/analyze';

export function PollAnalyzePage() {
  const { answers, survey, mappedResponses, handleDownloadExcel } =
    usePollSpaceContext();

  return (
    <AnalyzePage
      answers={answers}
      survey={survey}
      mappedResponses={mappedResponses}
      handleDownloadExcel={handleDownloadExcel}
    />
  );
}
