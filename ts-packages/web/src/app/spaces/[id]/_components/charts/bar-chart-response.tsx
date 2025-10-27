'use client';

import React from 'react';
import { ParsedResult } from './models/parsed';

export default function BarChartResponse({
  parsed,
  colors,
}: {
  parsed: ParsedResult;
  colors: string[];
}) {
  const { options } = parsed;

  return (
    <>
      {options.map((opt, idx) => (
        <div key={idx} className="flex items-center gap-3">
          <div
            className="max-w-[100px] w-full text-sm font-medium text-neutral-400 truncate overflow-hidden whitespace-nowrap"
            title={opt.label}
          >
            {opt.label}
          </div>
          <div className="flex-1 h-4 bg-neutral-300 rounded-full overflow-hidden">
            <div
              className="h-full rounded-full transition-[width] duration-500 ease-out"
              style={{
                width: `${opt.ratio}%`,
                backgroundColor: colors[idx % colors.length],
              }}
            ></div>
          </div>
          <div className="w-[80px] text-sm text-left text-neutral-400">
            {opt.count} ({opt.ratio.toFixed(1)}%)
          </div>
        </div>
      ))}
    </>
  );
}
