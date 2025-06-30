'use client';

import React from 'react';
import { PieChart, Pie, Cell, ResponsiveContainer } from 'recharts';
import {
  MultipleChoiceQuestion,
  SingleChoiceQuestion,
} from '@/lib/api/models/survey';

type ParsedOption = {
  label: string;
  count: number;
  ratio: number;
};

type ParsedResult = {
  question: SingleChoiceQuestion | MultipleChoiceQuestion;
  totalParticipants: number;
  options: ParsedOption[];
};

const COLORS = ['#a1a1a1', '#737373'];

export default function PieChartResponse({ parsed }: { parsed: ParsedResult }) {
  const { options } = parsed;

  const renderCustomizedLabel = ({
    cx,
    cy,
    midAngle,
    innerRadius,
    outerRadius,
    percent,
    index,
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
        <PieChart>
          <Pie
            data={options}
            dataKey="count"
            nameKey="label"
            cx="50%"
            cy="50%"
            outerRadius={100}
            labelLine={false}
            label={renderCustomizedLabel}
            isAnimationActive={true}
          >
            {options.map((_, index) => (
              <Cell
                key={`cell-${index}`}
                fill={COLORS[index % COLORS.length]}
              />
            ))}
          </Pie>
        </PieChart>
      </ResponsiveContainer>
    </div>
  );
}
