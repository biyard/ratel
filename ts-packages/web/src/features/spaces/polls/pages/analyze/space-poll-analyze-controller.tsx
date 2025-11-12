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
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';

export class SpacePollAnalyzeController {
  constructor(
    public spacePk: string,
    public pollPk: string,
    public navigate: NavigateFunction,
    public space: Space,
    public poll: Poll,
    public summary: PollSurveySummariesResponse,

    public t: TFunction<'SpacePollAnalyze', undefined>,
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
    const questions = this.poll?.questions ?? [];
    const qCount = questions.length;

    const userKeyFromPk = (pk: string | undefined) => {
      if (!pk) return '';
      const i = pk.indexOf('#USER#');
      return i >= 0 ? pk.slice(i + '#USER#'.length) : pk;
    };

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const toAnswerDisplay = (q: PollQuestion, ans: any): string => {
      const kind = String(q?.answer_type ?? '');
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const opts: string[] | undefined = Array.isArray((q as any)?.options)
        ? // eslint-disable-next-line @typescript-eslint/no-explicit-any
          (q as any).options
        : undefined;

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const labelOf = (v: any) => {
        const idx = typeof v === 'number' ? v : Number(v);
        if (Number.isFinite(idx) && opts && idx >= 0 && idx < opts.length) {
          return String(opts[idx] ?? `${idx}`);
        }
        return typeof v === 'string' || typeof v === 'number' ? String(v) : '';
      };

      if (['single_choice', 'dropdown', 'select', 'radio'].includes(kind)) {
        return labelOf(ans?.answer);
      }

      if (['multiple_choice', 'checkbox', 'multi_select'].includes(kind)) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        let arr: any[] = [];
        if (Array.isArray(ans?.answer)) arr = ans.answer;
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        else if (Array.isArray(ans)) arr = ans as any[];
        else if (typeof ans?.answer === 'string')
          arr = String(ans.answer)
            .split(',')
            .map((s) => s.trim())
            .filter((s) => s.length > 0);
        return arr.map(labelOf).join(', ');
      }

      if (kind === 'linear_scale') {
        return typeof ans?.answer !== 'undefined' ? String(ans.answer) : '';
      }

      return typeof ans?.answer !== 'undefined' ? String(ans.answer) : '';
    };

    const getGenderDisp = (g?: string) =>
      !g
        ? ''
        : g.toLowerCase() === 'male'
          ? 'Male'
          : g.toLowerCase() === 'female'
            ? 'Female'
            : g;

    const { sample_answers = [], final_answers = [] } = (this.summary ||
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      {}) as any;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const finalByUser = new Map<string, any>();
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    for (const f of final_answers as any[])
      finalByUser.set(userKeyFromPk(f?.pk), f);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const sampleByUser = new Map<string, any>();
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    for (const s of sample_answers as any[])
      sampleByUser.set(userKeyFromPk(s?.pk), s);

    const userOrder: string[] = [];
    for (const k of finalByUser.keys()) userOrder.push(k);
    for (const k of sampleByUser.keys())
      if (!finalByUser.has(k)) userOrder.push(k);

    const header1 = new Array(5 + qCount).fill('');
    header1[0] = this.t('id');
    header1[1] = this.t('attribute');
    header1[3] = this.t('category');
    header1[4] = this.t('type');
    if (qCount > 0) header1[5] = this.t('questionnaire');

    const header2 = new Array(5 + qCount).fill('');
    header2[1] = this.t('gender');
    header2[2] = this.t('university');

    const rows: (string | number)[][] = [header1, header2];

    const merges: XLSX.Range[] = [
      { s: { r: 0, c: 1 }, e: { r: 0, c: 2 } },
      { s: { r: 0, c: 0 }, e: { r: 1, c: 0 } },
      { s: { r: 0, c: 3 }, e: { r: 1, c: 3 } },
      { s: { r: 0, c: 4 }, e: { r: 1, c: 4 } },
    ];
    if (qCount > 0)
      merges.push({ s: { r: 0, c: 5 }, e: { r: 1, c: 5 + qCount - 1 } });

    const pushBlock = (
      roundLabel: '사전조사' | '사후조사',
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      meta: any,
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      answers: any[],
    ) => {
      const r1 = new Array(5 + qCount).fill('');
      r1[3] =
        roundLabel === '사전조사'
          ? this.t('sample_survey')
          : this.t('final_survey');
      r1[4] = this.t('question');
      for (let i = 0; i < qCount; i++)
        r1[5 + i] = this.poll?.questions?.[i]?.title ?? `Q${i + 1}`;

      const r2 = new Array(5 + qCount).fill('');
      r2[3] =
        roundLabel === '사전조사'
          ? this.t('sample_survey')
          : this.t('final_survey');
      r2[4] = this.t('answer');
      for (let i = 0; i < qCount; i++) {
        const ans = answers?.[i];
        r2[5 + i] = toAnswerDisplay(
          this.poll?.questions?.[i] as PollQuestion,
          ans,
        );
      }

      const start = rows.length;
      rows.push(r1, r2);
      merges.push({ s: { r: start, c: 3 }, e: { r: start + 1, c: 3 } });
      return { start, end: start + 1 };
    };

    for (const userKey of userOrder) {
      const f = finalByUser.get(userKey);
      const s = sampleByUser.get(userKey);
      const meta = f || s;
      if (!meta) continue;

      const name = meta.display_name || meta.username || userKey;
      const gender = getGenderDisp(meta?.respondent?.gender);
      const school = meta?.respondent?.school || '';

      const startIdx = rows.length;
      if (s) pushBlock('사전조사', meta, s.answers || []);
      if (f) pushBlock('사후조사', meta, f.answers || []);
      const endIdx = rows.length - 1;

      merges.push({ s: { r: startIdx, c: 0 }, e: { r: endIdx, c: 0 } });
      merges.push({ s: { r: startIdx, c: 1 }, e: { r: endIdx, c: 1 } });
      merges.push({ s: { r: startIdx, c: 2 }, e: { r: endIdx, c: 2 } });

      rows[startIdx][0] = name;
      rows[startIdx][1] = gender;
      rows[startIdx][2] = school;
    }

    const ws = XLSX.utils.aoa_to_sheet(rows);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (ws as any)['!merges'] = merges;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (ws as any)['!cols'] = Array.from({ length: 5 + qCount }, (_, idx) => {
      const base =
        idx === 0
          ? 18
          : idx === 1
            ? 10
            : idx === 2
              ? 16
              : idx === 3
                ? 12
                : idx === 4
                  ? 10
                  : 14;
      let maxLen = 0;
      for (const r of rows)
        maxLen = Math.max(maxLen, String(r[idx] ?? '').length);
      return { wch: Math.max(base, Math.min(maxLen + 2, 60)) };
    });

    const wb = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(wb, ws, 'Responses');
    XLSX.writeFile(wb, `${this.pollPk}.xlsx`);
  };
}

export function useSpacePollAnalyzeController(spacePk: string, pollPk: string) {
  // Fetching data from remote
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollPk);
  const { data: summary } = usePollSpaceSummaries(spacePk, pollPk);
  const { t } = useTranslation('SpacePollAnalyze');

  const navigator = useNavigate();

  return new SpacePollAnalyzeController(
    spacePk,
    pollPk,
    navigator,
    space,
    poll,
    summary,

    t,
  );
}
