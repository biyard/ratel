import {
  ObjectiveQuestionUnion,
  BaseObjectiveSummary,
  SurveyAnswerType,
} from '@/features/spaces/polls/types/poll-question';

interface ObjectiveResponseProps {
  t: I18nFunction;
  question: ObjectiveQuestionUnion;
  summary: BaseObjectiveSummary;
}

const colors = [
  '#f97316',
  '#22c55e',
  '#6366f1',
  '#274c9d',
  '#2959c1',
  '#2853af',
];

function shuffle<T>(arr: T[]): T[] {
  const copy = [...arr];
  for (let i = copy.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [copy[i], copy[j]] = [copy[j], copy[i]];
  }
  return copy;
}

export default function ObjectiveQuestionSummary({
  t,
  question,
  summary,
}: ObjectiveResponseProps) {
  const shuffledColors = shuffle(colors);
  const { answers, total_count } = summary;

  const options: Record<string, number> = Object.fromEntries(
    Object.entries(answers).map(([key, count]) => {
      let label = key;
      if (question.answer_type !== SurveyAnswerType.LinearScale) {
        label = question.options[parseInt(key)];
      }

      return [label, count] as [string, number];
    }),
  );

  return (
    <div className="flex flex-col gap-5 p-5 w-full bg-transparent rounded-xl border border-neutral-500">
      <div className="flex justify-between items-center pb-2 border-b border-input-box-border">
        <div className="text-base font-semibold text-neutral-400">
          {question.title}
        </div>
        <div className="text-sm font-medium text-neutral-400">
          {total_count} {t('total_response_count_unit')}
        </div>
      </div>

      <div className="flex flex-col gap-3">
        <BarChart
          answers={options}
          total_count={total_count}
          colors={shuffledColors}
        />
        <PieChart
          answers={options}
          total_count={total_count}
          colors={shuffledColors}
        />
      </div>
    </div>
  );
}

interface BarChartProps {
  answers: Record<string, number>; // (Option Label, Count)
  total_count: number;
  colors: string[];
}
export function BarChart({ answers, total_count, colors }: BarChartProps) {
  return (
    <>
      {Object.entries(answers).map(([label, count], idx) => (
        <div key={idx} className="flex gap-3 items-center">
          <div
            className="overflow-hidden w-full text-sm font-medium whitespace-nowrap max-w-[100px] text-neutral-400 truncate"
            title={label}
          >
            {label}
          </div>
          <div className="overflow-hidden flex-1 h-4 rounded-full bg-neutral-300">
            <div
              className="h-full rounded-full transition-[width] duration-500 ease-out"
              style={{
                width: `${(count / total_count) * 100}%`,
                backgroundColor: colors[idx % colors.length],
              }}
            ></div>
          </div>
          <div className="text-sm text-left w-[80px] text-neutral-400">
            {count} ({((count / total_count) * 100).toFixed(1)}%)
          </div>
        </div>
      ))}
    </>
  );
}

import {
  PieChart as RechartsPieChart,
  Pie,
  Cell,
  ResponsiveContainer,
} from 'recharts';
import { I18nFunction } from '.';
import { useEffect, useRef, useState } from 'react';

const COLORS = ['#a1a1a1', '#737373'];

interface PieChartProps {
  answers: Record<string, number>; // (Option Label, Count)
  total_count: number;
  colors: string[];
}

function PieChart({ answers, total_count, colors }: PieChartProps) {
  const hostRef = useRef<HTMLDivElement>(null);
  const [width, setWidth] = useState(0);

  useEffect(() => {
    const el = hostRef.current;
    if (!el) return;

    const measure = () => setWidth(el.getBoundingClientRect().width || 0);
    measure();

    const ro = new ResizeObserver(measure);
    ro.observe(el);
    window.addEventListener('resize', measure);
    return () => {
      ro.disconnect();
      window.removeEventListener('resize', measure);
    };
  }, []);

  const data = Object.entries(answers).map(([label, count]) => {
    const c = Number(count);
    return {
      label,
      count: Number.isFinite(c) && c > 0 ? c : 0,
      ratio:
        total_count > 0 && Number.isFinite(c) ? (c / total_count) * 100 : 0,
    };
  });

  const sum = data.reduce((s, d) => s + d.count, 0);
  if (width <= 0 || sum <= 0) {
    return <div ref={hostRef} className="w-full min-w-0 h-[250px]" />;
  }

  const W = Math.floor(width);
  const H = 250;
  const cx = Math.floor(W / 2);
  const cy = Math.floor(H / 2);
  const outerRadius = Math.min(100, Math.floor(Math.min(W, H) / 2) - 8);

  return (
    <div ref={hostRef} className="w-full min-w-0">
      <ResponsiveContainer
        width="100%"
        height={250}
        className="focus:outline-none"
      >
        <RechartsPieChart width={W} height={H} key={`pie-${W}`}>
          <Pie
            data={data}
            dataKey="count"
            nameKey="label"
            cx={cx}
            cy={cy}
            outerRadius={outerRadius}
            startAngle={90}
            endAngle={450}
            paddingAngle={0}
            labelLine={false}
            isAnimationActive={true}
            minAngle={0}
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            label={(props: any) => {
              const { cx, cy, midAngle, innerRadius, outerRadius, index } =
                props;
              if (
                [cx, cy, midAngle, innerRadius, outerRadius, index].some(
                  (v) => v === undefined,
                )
              )
                return null;

              const RAD = Math.PI / 180;
              const r = innerRadius + (outerRadius - innerRadius) * 0.5;
              const x = cx + r * Math.cos(-midAngle * RAD);
              const y = cy + r * Math.sin(-midAngle * RAD);
              const o = data[index];
              if (!o || o.count === 0) return null;

              return (
                <text
                  x={x}
                  y={y}
                  fill="#fff"
                  textAnchor="middle"
                  dominantBaseline="central"
                  style={{ fontSize: 12 }}
                >
                  {`${o.label}: ${o.count}`}
                  <tspan x={x} dy="1.2em">{`${o.ratio.toFixed(0)}%`}</tspan>
                </text>
              );
            }}
          >
            {data.map((_, i) => (
              <Cell key={i} fill={colors[i % COLORS.length]} />
            ))}
          </Pie>
        </RechartsPieChart>
      </ResponsiveContainer>
    </div>
  );
}
