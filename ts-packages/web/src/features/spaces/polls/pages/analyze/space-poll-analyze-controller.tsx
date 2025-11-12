import { Space } from '@/features/spaces/types/space';
import { Poll } from '../../types/poll';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import usePollSpace from '../../hooks/use-poll-space';
import usePollSpaceSummaries from '../../hooks/use-poll-space-summary';
import { PollSurveySummariesResponse } from '@/lib/api/ratel/poll.spaces.v3';
import {
  PollQuestion,
  SurveyAnswerType,
  SurveySummary,
} from '../../types/poll-question';
import { logger } from '@/lib/logger';
import * as XLSX from 'xlsx';
import { route } from '@/route';
import { NavigateFunction, useNavigate } from 'react-router';

export class SpacePollAnalyzeController {
  constructor(
    public spacePk: string,
    public pollPk: string,
    public navigate: NavigateFunction,
    public space: Space,
    public poll: Poll,
    public summary: PollSurveySummariesResponse,
  ) {}

  handleBack = () => {
    this.navigate(route.spaceAnalyzePolls(this.spacePk));
  };

  isSubjective = (t: SurveyAnswerType) =>
    t === SurveyAnswerType.ShortAnswer || t === SurveyAnswerType.Subjective;

  keyToLabel = (q: PollQuestion, key: string) => {
    if (q.answer_type === SurveyAnswerType.LinearScale) return key;
    if ('options' in q && Array.isArray(q.options)) {
      const i = Number.parseInt(key, 10);
      if (Number.isFinite(i) && i >= 0 && i < q.options.length) {
        return q.options[i] ?? `${i + 1}`;
      }
    }
    return key;
  };

  normalizeAnswerEntries = (summary?: SurveySummary) => {
    if (!summary) return [];
    const raw = summary.answers as
      | Record<string, number>
      | Record<number, number>;
    const entries: Array<[string, number]> = [];
    for (const [k, v] of Object.entries(raw as Record<string, number>)) {
      entries.push([k, Number(v) || 0]);
    }
    return entries;
  };

  handleDownloadExcel = () => {
    const questions = this.poll.questions;
    const summaries = this.summary.summaries;
    logger.debug(
      'Download Excel clicked with summaries: ',
      questions,
      summaries,
    );

    const perQuestionResponses: string[][] = [];
    let maxRespCols = 0;

    questions.forEach((q, i) => {
      const s = summaries[i];
      const entries = this.normalizeAnswerEntries(s);

      entries.sort(([a], [b]) => {
        const ia = Number(a);
        const ib = Number(b);
        return Number.isFinite(ia) && Number.isFinite(ib) ? ia - ib : 0;
      });

      const parts = entries.map(([key, cnt]) => {
        const label = this.isSubjective(q.answer_type)
          ? key
          : this.keyToLabel(q, key);
        return `${label} (${cnt})`;
      });

      perQuestionResponses.push(parts);
      maxRespCols = Math.max(maxRespCols, parts.length);
    });

    const header = [
      'Index',
      'Question',
      'Total Responses',
      ...Array.from({ length: maxRespCols }, (_, i) => `Answer ${i + 1}`),
    ];

    const rows: (string | number)[][] = [header];
    questions.forEach((q, i) => {
      const total = summaries[i]?.total_count ?? 0;
      const parts = perQuestionResponses[i] ?? [];
      const padded = [...parts, ...Array(maxRespCols - parts.length).fill('')];
      rows.push([i + 1, q.title, total, ...padded]);
    });

    const ws = XLSX.utils.aoa_to_sheet(rows);
    const colInfos: XLSX.ColInfo[] = header.map((h, idx) => {
      const maxLen = rows.reduce(
        (m, r) => Math.max(m, String(r[idx] ?? '').length),
        h.length,
      );
      const base = idx === 0 ? 6 : idx === 1 ? 40 : idx === 2 ? 16 : 24;
      return { wch: Math.max(base, Math.min(maxLen + 2, 60)) };
    });
    ws['!cols'] = colInfos;

    const wb = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(wb, ws, 'Overview');
    XLSX.writeFile(wb, `${this.pollPk}.xlsx`);
  };
}

export function useSpacePollAnalyzeController(spacePk: string, pollPk: string) {
  // Fetching data from remote
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollPk);
  const { data: summary } = usePollSpaceSummaries(spacePk, pollPk);

  console.log('poll summary: ', summary);
  const navigator = useNavigate();

  return new SpacePollAnalyzeController(
    spacePk,
    pollPk,
    navigator,
    space,
    poll,
    summary,
  );
}
