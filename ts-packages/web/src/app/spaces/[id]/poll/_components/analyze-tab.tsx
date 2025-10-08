'use client';
import { AnalyzePage } from '../../_components/page/analyze';
import type { Space } from '@/lib/api/models/spaces';
import { mapResponses } from '../../deliberation/provider.client';
import { usePollStore } from '../store';
import { Question } from '@/lib/api/models/survey';
import { SurveyResponse } from '@/lib/api/models/response';

export function PollAnalyzePage({ space }: { space: Space }) {
  const { responses: answers } = space;
  const { survey } = usePollStore();
  const mappedResponses = mapResponses(
    survey.surveys?.[0]?.questions ?? [],
    space?.responses ?? [],
  );

  return (
    <AnalyzePage
      answers={answers}
      survey={survey}
      mappedResponses={mappedResponses}
      handleDownloadExcel={() =>
        handleDownloadExcel(
          space.id,
          survey?.surveys?.[0]?.questions || [],
          answers || [],
        )
      }
    />
  );
}

import * as XLSX from 'xlsx';

const handleDownloadExcel = (
  spaceId: number,
  questions: Question[],
  responses: SurveyResponse[],
) => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const excelRows: any[] = [];

  questions.forEach((question, questionIndex) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const row: any = {
      Index: questionIndex + 1,
      Question: question.title,
    };

    responses.forEach((response, responseIndex) => {
      const rawAnswer = response.answers?.[questionIndex]?.answer;

      let parsedAnswer;

      if (typeof rawAnswer === 'string') {
        parsedAnswer = rawAnswer;
      } else if (typeof rawAnswer === 'number') {
        parsedAnswer = rawAnswer + 1;
      } else if (Array.isArray(rawAnswer)) {
        parsedAnswer = rawAnswer.map((v) => Number(v) + 1).join(', ');
      } else {
        parsedAnswer =
          question.answer_type === 'short_answer' ||
          question.answer_type === 'subjective'
            ? ''
            : 0;
      }

      row[`Response ${responseIndex + 1}`] = parsedAnswer;
    });

    excelRows.push(row);
  });

  const worksheet = XLSX.utils.json_to_sheet(excelRows);

  worksheet['!cols'] = Object.keys(excelRows[0]).map((key) => {
    const maxLen = Math.max(
      key.length,
      ...excelRows.map((row) => String(row[key] ?? '').length),
    );
    return { wch: maxLen + 2 };
  });

  const workbook = XLSX.utils.book_new();
  XLSX.utils.book_append_sheet(workbook, worksheet, 'Survey Responses');
  XLSX.writeFile(workbook, `${spaceId}.xlsx`);
};
