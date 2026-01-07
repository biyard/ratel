import React, { useMemo } from 'react';
import { TFunction } from 'i18next';
import { TfIdf } from '../../polls/types/tf-idf';
import {
  ResponsiveContainer,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  LabelList,
} from 'recharts';

export type TfIdfChartProps = {
  t: TFunction<'SpacePollAnalyze', undefined>;
  tf_idf?: TfIdf[];
  limit?: number;
};

export function TfIdfChart({ t, tf_idf, limit = 10 }: TfIdfChartProps) {
  const data = useMemo(() => {
    const list = Array.isArray(tf_idf) ? tf_idf : [];
    return list
      .map((r) => ({
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        keyword: String((r as any)?.keyword ?? '').trim(),
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        tf_idf: Number((r as any)?.tf_idf ?? 0),
      }))
      .filter((r) => r.keyword.length > 0 && Number.isFinite(r.tf_idf))
      .sort((a, b) => b.tf_idf - a.tf_idf)
      .slice(0, limit);
  }, [tf_idf, limit]);

  const { niceMax, ticks } = useMemo(() => {
    const max = data.reduce((m, r) => Math.max(m, r.tf_idf), 0);
    const step = 0.5;

    const nm = Math.max(step, Math.ceil(max / step) * step);
    const count = Math.round(nm / step);

    return {
      niceMax: nm,
      ticks: Array.from({ length: count + 1 }, (_, i) => i * step),
    };
  }, [data]);

  const chartHeight = Math.max(280, data.length * 30 + 70);

  if (data.length === 0) {
    return (
      <div className="w-full rounded-xl border border-input-box-border p-6 text-center text-sm text-text-secondary">
        {t('no_tfidf')}
      </div>
    );
  }

  return (
    <div className="w-full">
      <div className="mb-3 text-center text-base font-semibold text-text-primary">
        TF-IDF
      </div>

      <div className="w-full" style={{ height: chartHeight }}>
        <ResponsiveContainer width="100%" height="100%">
          <BarChart
            data={data}
            layout="vertical"
            margin={{ top: 6, right: 44, left: 28, bottom: 10 }}
            barCategoryGap={2}
          >
            <XAxis
              type="number"
              domain={[0, niceMax]}
              ticks={ticks}
              tickMargin={10}
              tickFormatter={(v) => Number(v).toFixed(1)}
              axisLine={false}
              tickLine={false}
            />

            <YAxis
              type="category"
              dataKey="keyword"
              width={90}
              axisLine={false}
              tickLine={false}
              tickMargin={14}
              interval={0}
              tick={{ dy: 2 }}
            />

            <Bar
              dataKey="tf_idf"
              fill="#2E5068"
              barSize={20}
              isAnimationActive
              animationDuration={650}
              animationEasing="ease-out"
            >
              <LabelList
                dataKey="tf_idf"
                position="right"
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                formatter={(v: any) => {
                  const n = Number(v);
                  return Number.isFinite(n) ? n.toFixed(2) : '';
                }}
                style={{ fill: '#111827', fontSize: 12 }}
              />
            </Bar>
          </BarChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
