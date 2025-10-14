import {
  ObjectiveQuestionUnion,
  BaseObjectiveSummary,
  SurveyAnswerType,
} from '@/types/survey-type';

interface ObjectiveResponseProps {
  t: I18nFunction;
  question: ObjectiveQuestionUnion;
  summary: BaseObjectiveSummary;
}
export default function ObjectiveQuestionSummary({
  t,
  question,
  summary,
}: ObjectiveResponseProps) {
  console.log('Rendering ObjectiveQuestionSummary', summary);
  const { answers, total_count } = summary;
  console.log('answers', answers);

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
    <div className="w-full p-5 bg-transparent rounded-xl flex flex-col gap-5 border border-neutral-500">
      <div className="flex items-center justify-between border-b border-input-box-border pb-2">
        <div className="text-base font-semibold text-neutral-400">
          {question.title}
        </div>
        <div className="text-sm font-medium text-neutral-400">
          {total_count} {t('total_response_count_unit')}
        </div>
      </div>

      <div className="flex flex-col gap-3">
        <BarChart answers={options} total_count={total_count} />
        <PieChart answers={options} total_count={total_count} />
      </div>
    </div>
  );
}

interface BarChartProps {
  answers: Record<string, number>; // (Option Label, Count)
  total_count: number;
}
export function BarChart({ answers, total_count }: BarChartProps) {
  return (
    <>
      {Object.entries(answers).map(([label, count], idx) => (
        <div key={idx} className="flex items-center gap-3">
          <div
            className="max-w-[100px] w-full text-sm font-medium text-neutral-400 truncate overflow-hidden whitespace-nowrap"
            title={label}
          >
            {label}
          </div>
          <div className="flex-1 h-4 bg-neutral-700 rounded-full overflow-hidden">
            <div
              className="h-full rounded-full bg-neutral-400 transition-[width] duration-500 ease-out"
              style={{ width: `${(count / total_count) * 100}%` }}
            ></div>
          </div>
          <div className="w-[80px] text-sm text-left text-neutral-400">
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

const COLORS = ['#a1a1a1', '#737373'];

interface PieChartProps {
  answers: Record<string, number>; // (Option Label, Count)
  total_count: number;
}

function PieChart({ answers, total_count }: PieChartProps) {
  const options = Object.entries(answers).map(([label, count]) => ({
    label,
    count,
    ratio: (count / total_count) * 100,
  }));
  const renderCustomizedLabel = ({
    cx,
    cy,
    midAngle,
    innerRadius,
    outerRadius,
    percent,
    index,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  }: any) => {
    if (
      cx === undefined ||
      cy === undefined ||
      midAngle === undefined ||
      innerRadius === undefined ||
      outerRadius === undefined ||
      percent === undefined ||
      index === undefined
    ) {
      return null;
    }

    const RADIAN = Math.PI / 180;
    const radius = innerRadius + (outerRadius - innerRadius) * 0.5;
    const x = cx + radius * Math.cos(-midAngle * RADIAN);
    const y = cy + radius * Math.sin(-midAngle * RADIAN);
    const option = options[index];

    if (!option || option.count === 0) return null;

    return (
      <text
        x={x}
        y={y}
        fill="#fff"
        textAnchor="middle"
        dominantBaseline="central"
        style={{ fontSize: 12 }}
      >
        {`${option.label}: ${option.count}`}
        <tspan x={x} dy="1.2em">
          {`${option.ratio.toFixed(0)}%`}
        </tspan>
      </text>
    );
  };

  return (
    <div className="w-full flex flex-col items-start justify-start">
      <ResponsiveContainer
        width="100%"
        height={250}
        className="focus:outline-none"
      >
        <RechartsPieChart>
          <Pie
            data={options}
            dataKey="count"
            nameKey="label"
            cx="50%"
            cy="50%"
            outerRadius={100}
            labelLine={false}
            label={renderCustomizedLabel}
            fill="none"
            strokeWidth={0}
            isAnimationActive={true}
          >
            {options.map((_, index) => (
              <Cell
                key={`cell-${index}`}
                fill={COLORS[index % COLORS.length]}
              />
            ))}
          </Pie>
        </RechartsPieChart>
      </ResponsiveContainer>
    </div>
  );
}
