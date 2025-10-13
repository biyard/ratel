import { SurveyResponseResponse } from '@/lib/api/ratel/spaces/deliberation-spaces.v3';
import { AnalyzePage } from '../analyze';
import { MappedResponse, Poll } from '../../types';
import { TFunction } from 'i18next';

export type DeliberationAnalyzePageProps = {
  t: TFunction<'DeliberationSpace', undefined>;
  answers: SurveyResponseResponse[];
  survey: Poll;
  mappedResponses: MappedResponse[];
  handleDownloadExcel: () => void;
};

export default function DeliberationAnalyzePage({
  answers,
  survey,
  mappedResponses,
  handleDownloadExcel,
}: DeliberationAnalyzePageProps) {
  return (
    <AnalyzePage
      answers={answers}
      survey={survey}
      mappedResponses={mappedResponses}
      handleDownloadExcel={handleDownloadExcel}
    />
  );
}
