import { AnalyzePage } from '../analyze';
import { TFunction } from 'i18next';
import { MappedResponse, Poll } from '@/app/spaces/[id]/type';
import { SurveyResponseResponse } from '../../utils/deliberation.spaces.v3';

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
