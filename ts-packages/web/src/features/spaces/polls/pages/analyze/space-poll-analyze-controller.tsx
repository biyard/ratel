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
import { PanelAttribute } from '@/features/spaces/panels/types/panel-attribute';
import { useListPanels } from '@/features/spaces/panels/hooks/use-list-panels';
import useTopic from '../../hooks/use-topic';
import { SpaceAnalyze } from '../../types/space-analyze';
import { useUpdateLdaMutation } from '../../hooks/use-update-lda-mutation';
import { useUpsertAnalyzeMutation } from '../../hooks/use-upsert-analyze-mutation';
import { useUpdateNetworkMutation } from '../../hooks/use-update-network-mutation';
import { useUpdateTfIdfMutation } from '../../hooks/use-update-tf-idf-mutation';
import { useDownloadAnalyzeMutation } from '../../hooks/use-download-analyze-mutation';
import { logger } from '@/lib/logger';
import { showSuccessToast } from '@/lib/toast';

export class SpacePollAnalyzeController {
  constructor(
    public spacePk: string,
    public pollPk: string,
    public navigate: NavigateFunction,
    public space: Space,
    public poll: Poll,
    public analyze: SpaceAnalyze,
    public summary: PollSurveySummariesResponse,
    public attributes: PanelAttribute[],

    public t: TFunction<'SpacePollAnalyze', undefined>,
    public updateLda: ReturnType<typeof useUpdateLdaMutation>,
    public updateNetwork: ReturnType<typeof useUpdateNetworkMutation>,
    public updateTfIdf: ReturnType<typeof useUpdateTfIdfMutation>,
    public upsertAnalyze: ReturnType<typeof useUpsertAnalyzeMutation>,
    public downloadAnalyze: ReturnType<typeof useDownloadAnalyzeMutation>,
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

  handleDownloadAnalyze = async () => {
    return await this.downloadAnalyze.mutateAsync({ spacePk: this.spacePk });
  };

  handleUpsertAnalyze = async (
    ldaTopics: number,
    tfIdfKeywords: number,
    networkTopNodes: number,
  ) => {
    try {
      const d = await this.upsertAnalyze.mutateAsync({
        spacePk: this.spacePk,
        ldaTopics,
        tfIdfKeywords,
        networkTopNodes,
      });

      showSuccessToast(this.t('success_analyze'));
      return d;
    } catch (e) {
      logger.error('upsert analyze failed: {}', e);
    }
  };

  handleUpdateLda = (
    topics: string[],
    keywords: string[][],
    htmlContents?: string,
  ) => {
    return this.updateLda.mutateAsync({
      spacePk: this.spacePk,
      topics: topics,
      keywords: keywords,
      htmlContents,
    });
  };

  handleUpdateNetwork = (htmlContents?: string) => {
    return this.updateNetwork.mutateAsync({
      spacePk: this.spacePk,
      htmlContents,
    });
  };

  handleUpdateTfIdf = (htmlContents?: string) => {
    return this.updateTfIdf.mutateAsync({
      spacePk: this.spacePk,
      htmlContents,
    });
  };

  handleDownloadExcel = () => {
    const questions = this.poll?.questions ?? [];
    const qCount = questions.length;

    const needGender = (this.attributes ?? []).some((a) => {
      if (!a) return false;

      if (a.type === 'verifiable_attribute') {
        if (typeof a.value === 'string') {
          return (
            a.value === 'gender' || a.value.toLowerCase().startsWith('gender')
          );
        }

        return a.value.type === 'gender';
      }

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const anyA = a as any;
      if (anyA.key === 'gender') return true;

      return false;
    });

    const needUniversity = (this.attributes ?? []).some(
      (a) => a?.type === 'collective_attribute' && a?.value === 'university',
    );

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
      const raw: any =
        ans && typeof ans === 'object' && 'answer' in ans ? ans.answer : ans;

      const otherText =
        ans &&
        typeof ans === 'object' &&
        'other' in ans &&
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        typeof (ans as any).other === 'string'
          ? // eslint-disable-next-line @typescript-eslint/no-explicit-any
            ((ans as any).other as string).trim()
          : '';

      if (
        raw === null ||
        typeof raw === 'undefined' ||
        (Array.isArray(raw) && raw.length === 0) ||
        (typeof raw === 'string' && raw.trim().length === 0)
      ) {
        return '';
      }

      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const labelOf = (v: any) => {
        const idx = typeof v === 'number' ? v : Number(v);
        if (Number.isFinite(idx) && opts && idx >= 0 && idx < opts.length) {
          return String(opts[idx] ?? `${idx}`);
        }
        return typeof v === 'string' || typeof v === 'number' ? String(v) : '';
      };

      const OTHER_LABEL = 'Others';

      if (kind === 'single_choice') {
        if (
          typeof raw === 'number' &&
          opts &&
          raw >= 0 &&
          raw < opts.length &&
          otherText.length > 0 &&
          opts[raw] === OTHER_LABEL
        ) {
          return otherText;
        }
        return labelOf(raw);
      }

      if (['single_choice', 'dropdown', 'select', 'radio'].includes(kind)) {
        return labelOf(raw);
      }

      if (['multiple_choice', 'checkbox', 'multi_select'].includes(kind)) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        let arr: any[] = [];
        if (Array.isArray(raw)) {
          arr = raw;
        } else if (typeof raw === 'string') {
          arr = raw
            .split(',')
            .map((s) => s.trim())
            .filter((s) => s.length > 0);
        }

        if (arr.length === 0) return '';

        arr = arr
          .map((v) => (typeof v === 'number' ? v : Number(v)))
          .filter((n) => Number.isFinite(n))
          .sort((a, b) => a - b);

        const otherIdx =
          opts && opts.includes(OTHER_LABEL) ? opts.indexOf(OTHER_LABEL) : -1;

        return arr
          .map((v) => {
            const idx = typeof v === 'number' ? v : Number(v);

            if (
              otherText.length > 0 &&
              otherIdx >= 0 &&
              Number.isFinite(idx) &&
              idx === otherIdx &&
              opts?.[idx] === OTHER_LABEL
            ) {
              return otherText;
            }

            return labelOf(v);
          })
          .join(', ');
      }

      if (kind === 'linear_scale') {
        return String(raw);
      }

      return String(raw);
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

    let col = 0;
    const COL_ID = col++;
    const COL_ATTR_START = col;
    const COL_GENDER = needGender ? col++ : -1;
    const COL_UNIV = needUniversity ? col++ : -1;
    const attrCols = (needGender ? 1 : 0) + (needUniversity ? 1 : 0);
    const COL_CATEGORY = col++;
    const COL_TYPE = col++;
    const COL_Q_START = col;
    const totalCols = COL_Q_START + qCount;

    const header1 = new Array(totalCols).fill('');
    header1[COL_ID] = this.t('id');
    if (attrCols > 0) header1[COL_ATTR_START] = this.t('attribute');
    header1[COL_CATEGORY] = this.t('category');
    header1[COL_TYPE] = this.t('type');
    if (qCount > 0) header1[COL_Q_START] = this.t('questionnaire');

    const header2 = new Array(totalCols).fill('');
    if (COL_GENDER >= 0) header2[COL_GENDER] = this.t('gender');
    if (COL_UNIV >= 0) header2[COL_UNIV] = this.t('university');

    const rows: (string | number)[][] = [header1, header2];

    const merges: XLSX.Range[] = [
      { s: { r: 0, c: COL_ID }, e: { r: 1, c: COL_ID } },
      { s: { r: 0, c: COL_CATEGORY }, e: { r: 1, c: COL_CATEGORY } },
      { s: { r: 0, c: COL_TYPE }, e: { r: 1, c: COL_TYPE } },
    ];
    if (attrCols > 0) {
      merges.push({
        s: { r: 0, c: COL_ATTR_START },
        e: { r: 0, c: COL_ATTR_START + attrCols - 1 },
      });
    }
    if (qCount > 0) {
      merges.push({
        s: { r: 0, c: COL_Q_START },
        e: { r: 1, c: COL_Q_START + qCount - 1 },
      });
    }

    const pushBlock = (
      roundLabel: '사전조사' | '사후조사',
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      meta: any,
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      answers: any[],
    ) => {
      const r1 = new Array(totalCols).fill('');
      r1[COL_CATEGORY] =
        roundLabel === '사전조사'
          ? this.t('sample_survey')
          : this.t('final_survey');
      r1[COL_TYPE] = this.t('question');
      for (let i = 0; i < qCount; i++)
        r1[COL_Q_START + i] = this.poll?.questions?.[i]?.title ?? `Q${i + 1}`;

      const r2 = new Array(totalCols).fill('');
      r2[COL_CATEGORY] = r1[COL_CATEGORY];
      r2[COL_TYPE] = this.t('answer');
      for (let i = 0; i < qCount; i++) {
        const ans = answers?.[i];
        r2[COL_Q_START + i] = toAnswerDisplay(
          this.poll?.questions?.[i] as PollQuestion,
          ans,
        );
      }

      const start = rows.length;
      rows.push(r1, r2);
      merges.push({
        s: { r: start, c: COL_CATEGORY },
        e: { r: start + 1, c: COL_CATEGORY },
      });
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

      merges.push({
        s: { r: startIdx, c: COL_ID },
        e: { r: endIdx, c: COL_ID },
      });
      if (COL_GENDER >= 0)
        merges.push({
          s: { r: startIdx, c: COL_GENDER },
          e: { r: endIdx, c: COL_GENDER },
        });
      if (COL_UNIV >= 0)
        merges.push({
          s: { r: startIdx, c: COL_UNIV },
          e: { r: endIdx, c: COL_UNIV },
        });

      rows[startIdx][COL_ID] = name;
      if (COL_GENDER >= 0) rows[startIdx][COL_GENDER] = gender;
      if (COL_UNIV >= 0) rows[startIdx][COL_UNIV] = school;
    }

    const ws = XLSX.utils.aoa_to_sheet(rows);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (ws as any)['!merges'] = merges;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    (ws as any)['!cols'] = Array.from({ length: totalCols }, (_, idx) => {
      const base =
        idx === COL_ID
          ? 18
          : idx === COL_GENDER
            ? 10
            : idx === COL_UNIV
              ? 16
              : idx === COL_CATEGORY
                ? 12
                : idx === COL_TYPE
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
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollPk);
  const { data: analyze } = useTopic(spacePk);
  const { data: summary } = usePollSpaceSummaries(spacePk, pollPk);
  const { data: panels } = useListPanels(spacePk);

  const updateLda = useUpdateLdaMutation();
  const updateNetwork = useUpdateNetworkMutation();
  const updateTfIdf = useUpdateTfIdfMutation();

  const upsertAnalyze = useUpsertAnalyzeMutation();
  const downloadAnalyze = useDownloadAnalyzeMutation();
  const attribute = panels?.map((p) => p.attributes).flat() ?? [];
  const { t } = useTranslation('SpacePollAnalyze');

  console.log('analyze data: ', analyze);

  const navigator = useNavigate();

  return new SpacePollAnalyzeController(
    spacePk,
    pollPk,
    navigator,
    space,
    poll,
    analyze,
    summary,
    attribute,

    t,
    updateLda,
    updateNetwork,
    updateTfIdf,
    upsertAnalyze,
    downloadAnalyze,
  );
}
