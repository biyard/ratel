import { logger } from '@/lib/logger';
import { useTranslation } from 'react-i18next';
import ObjectiveResponse from '@/app/spaces/[id]/_components/dashboard/objective-response';
import SubjectiveResponse from '@/app/spaces/[id]/_components/dashboard/subjective-response';
import SummaryReport from '@/app/spaces/[id]/_components/dashboard/summary-report';
import { MappedResponse, Poll } from '../types/poll-type';
import { SurveyResponseResponse } from '../utils/deliberation.spaces.v3';

const AnswerType = {
  SingleChoice: 'single_choice',
  MultipleChoice: 'multiple_choice',
  ShortAnswer: 'short_answer',
  Subjective: 'subjective',
  Checkbox: 'checkbox',
  Dropdown: 'dropdown',
  LinearScale: 'linear_scale',
} as const;

type AnswerType = (typeof AnswerType)[keyof typeof AnswerType];

export function AnalyzePage({
  answers,
  survey,
  mappedResponses,
  handleDownloadExcel,
}: {
  survey: Poll;
  answers: SurveyResponseResponse[];
  mappedResponses: MappedResponse[];
  handleDownloadExcel: () => void;
}) {
  const { t } = useTranslation('DeliberationSpace');
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
            {t('download_excel')}
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
