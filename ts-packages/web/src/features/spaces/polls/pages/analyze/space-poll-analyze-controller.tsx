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
    const {
      summaries,
      summaries_by_gender,
      summaries_by_age,
      summaries_by_school,
    } = this.summary;

    const entriesToParts = (q: PollQuestion, s?: SurveySummary) => {
      const entries = this.normalizeAnswerEntries(s);
      entries.sort(([a], [b]) => {
        const ia = Number(a);
        const ib = Number(b);
        return Number.isFinite(ia) && Number.isFinite(ib)
          ? ia - ib
          : a > b
            ? 1
            : -1;
      });
      return entries.map(([key, cnt]) => {
        const label = this.isSubjective(q.answer_type)
          ? key
          : this.keyToLabel(q, key);
        return `${label} (${cnt})`;
      });
    };

    const autoCols = (rows: (string | number)[], header: string[]) =>
      header.map((h, idx) => {
        const maxLen = (rows as unknown as (string | number)[][]).reduce(
          (m, r) => Math.max(m, String(r[idx] ?? '').length),
          h.length,
        );
        const base =
          idx === 0 ? 10 : idx === 1 ? 6 : idx === 2 ? 40 : idx === 3 ? 16 : 24;
        return { wch: Math.max(base, Math.min(maxLen + 2, 80)) };
      });

    const buildOverviewSheet = () => {
      const perQParts: string[][] = [];
      let maxRespCols = 0;
      questions.forEach((q, i) => {
        const parts = entriesToParts(q, summaries[i]);
        perQParts.push(parts);
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
        const parts = perQParts[i] ?? [];
        const padded = [
          ...parts,
          ...Array(maxRespCols - parts.length).fill(''),
        ];
        rows.push([i + 1, q.title, total, ...padded]);
      });

      const ws = XLSX.utils.aoa_to_sheet(rows);
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (ws as any)['!cols'] = autoCols(
        rows as unknown as (string | number)[],
        header,
      );
      return ws;
    };

    type GroupMap = Record<string, SurveySummary[]>;
    const buildGroupedSheet = (sheetTitle: string, groupMap?: GroupMap) => {
      if (!groupMap || Object.keys(groupMap).length === 0) return null;

      let maxRespCols = 0;
      Object.values(groupMap).forEach((arr) => {
        questions.forEach((q, i) => {
          const parts = entriesToParts(q, arr?.[i]);
          maxRespCols = Math.max(maxRespCols, parts.length);
        });
      });

      const header = [
        'Group',
        'Index',
        'Question',
        'Total Responses',
        ...Array.from({ length: maxRespCols }, (_, i) => `Answer ${i + 1}`),
      ];

      const rows: (string | number)[][] = [header];

      const groups = Object.entries(groupMap);
      groups.forEach(([groupName, arr], gi) => {
        questions.forEach((q, i) => {
          const s = arr?.[i];
          const total = s?.total_count ?? 0;
          const parts = entriesToParts(q, s);
          const padded = [
            ...parts,
            ...Array(maxRespCols - parts.length).fill(''),
          ];
          rows.push([groupName, i + 1, q.title, total, ...padded]);
        });

        if (gi < groups.length - 1) {
          rows.push(new Array(4 + maxRespCols).fill(''));
        }
      });

      const ws = XLSX.utils.aoa_to_sheet(rows);
      ws['!cols'] = header.map((h, idx) => {
        const maxLen = rows.reduce(
          (m, r) => Math.max(m, String(r[idx] ?? '').length),
          h.length,
        );
        const base =
          idx === 0 ? 10 : idx === 1 ? 6 : idx === 2 ? 40 : idx === 3 ? 16 : 24;
        return { wch: Math.max(base, Math.min(maxLen + 2, 80)) };
      });

      return ws;
    };

    const wb = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(wb, buildOverviewSheet(), 'Overview');

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const wsGender = buildGroupedSheet('Gender', summaries_by_gender as any);
    if (wsGender) XLSX.utils.book_append_sheet(wb, wsGender, 'Gender');

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const wsAge = buildGroupedSheet('Age', summaries_by_age as any);
    if (wsAge) XLSX.utils.book_append_sheet(wb, wsAge, 'Age');

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const wsSchool = buildGroupedSheet('School', summaries_by_school as any);
    if (wsSchool) XLSX.utils.book_append_sheet(wb, wsSchool, 'School');

    const filename = `${this.pollPk}.xlsx`;
    XLSX.writeFile(wb, filename);
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
