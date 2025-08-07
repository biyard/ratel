'use client';
import React from 'react';
import { useDeliberationSpaceContext } from '../provider.client';
import { AnalyzePage } from '../../_components/page/analyze';

export default function DeliberationAnalyzePage() {
  const { answers, survey, mappedResponses, handleDownloadExcel } =
    useDeliberationSpaceContext();

  return (
    <AnalyzePage
      answers={answers}
      survey={survey}
      mappedResponses={mappedResponses}
      handleDownloadExcel={handleDownloadExcel}
    />
  );
}
