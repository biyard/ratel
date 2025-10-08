'use client';
import { useDeliberationSpaceByIdContext } from '../../providers.client';
import { AnalyzePage } from '../analyze';

export default function DeliberationAnalyzePage() {
  const { answers, survey, mappedResponses, handleDownloadExcel } =
    useDeliberationSpaceByIdContext();

  return (
    <AnalyzePage
      answers={answers}
      survey={survey}
      mappedResponses={mappedResponses}
      handleDownloadExcel={handleDownloadExcel}
    />
  );
}
