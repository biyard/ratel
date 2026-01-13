import React, { useEffect, useMemo, useState } from 'react';
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
import { Edit1, Save } from '@/components/icons';
import { PostEditor } from '@/features/posts/components/post-editor';

export type TfIdfChartProps = {
  t: TFunction<'SpacePollAnalyze', undefined>;
  tf_idf?: TfIdf[];
  limit?: number;
  htmlContents?: string;
  handleUpdateTfIdf?: (htmlContents?: string) => void;
};

export function TfIdfChart({
  t,
  tf_idf,
  htmlContents,
  handleUpdateTfIdf,
  limit = 10,
}: TfIdfChartProps) {
  const [content, setContent] = useState<string>(htmlContents ?? '');
  const [editing, setEditing] = useState(false);

  const startEdit = () => {
    setContent(htmlContents ?? '');
    setEditing(true);
  };

  const save = () => {
    handleUpdateTfIdf?.(content);
    setEditing(false);
  };

  useEffect(() => {
    if (editing) return;
    setContent(htmlContents ?? '');
  }, [htmlContents, editing]);

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

    const base = Math.max(step, Math.ceil(max / step) * step);
    const padded = base + step;
    const count = Math.round(padded / step);

    return {
      niceMax: padded,
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

  const LABEL_TEXT_COLOR = 'var(--color-text-secondary)';

  return (
    <div className="w-full">
      <div className="mb-2 flex items-center justify-end gap-2">
        {!editing ? (
          <Edit1
            className="cursor-pointer w-5 h-5 [&>path]:stroke-1"
            onClick={startEdit}
          />
        ) : (
          <Save
            className="cursor-pointer w-5 h-5 [&>path]:stroke-1"
            onClick={save}
          />
        )}
      </div>
      <div className="mb-3 text-center text-base font-semibold text-text-primary">
        TF-IDF
      </div>

      <div className="w-full" style={{ height: chartHeight }}>
        <ResponsiveContainer width="100%" height="100%">
          <BarChart
            data={data}
            layout="vertical"
            margin={{ top: 6, right: 72, left: 28, bottom: 10 }}
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
              tick={{ fill: LABEL_TEXT_COLOR, fontSize: 12 }}
            />

            <YAxis
              type="category"
              dataKey="keyword"
              width={90}
              axisLine={false}
              tickLine={false}
              tickMargin={14}
              interval={0}
              tick={{ dy: 2, fill: LABEL_TEXT_COLOR, fontSize: 12 }}
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
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                content={(p: any) => {
                  const n = Number(p?.value);
                  if (!Number.isFinite(n)) return null;

                  const x = Number(p?.x ?? 0);
                  const y = Number(p?.y ?? 0);
                  const w = Number(p?.width ?? 0);
                  const h = Number(p?.height ?? 0);

                  return (
                    <text
                      x={x + w + 10}
                      y={y + h / 2}
                      dominantBaseline="middle"
                      textAnchor="start"
                      style={{ fill: LABEL_TEXT_COLOR, fontSize: 12 }}
                    >
                      {n.toFixed(2)}
                    </text>
                  );
                }}
              />
            </Bar>
          </BarChart>
        </ResponsiveContainer>
      </div>

      <PostEditor
        url={''}
        content={content}
        onUpdate={(next) => setContent(next)}
        disabledFileUpload={true}
        disabledImageUpload={true}
        editable={editing}
        showToolbar={editing}
      />
    </div>
  );
}
