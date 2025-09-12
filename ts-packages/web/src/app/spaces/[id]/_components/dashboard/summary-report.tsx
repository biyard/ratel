'use client';

import { useTranslations } from 'next-intl';
import React from 'react';

export default function SummaryReport({
  responseCount,
  startDate,
  endDate,
}: {
  responseCount: number;
  startDate: number;
  endDate: number;
}) {
  const t = useTranslations('PollSpace');
  const now = Math.floor(Date.now() / 1000);
  const timeLeft = endDate - now;

  let dueDate = '';
  if (timeLeft <= 0) {
    dueDate = '0 Day';
  } else {
    const daysLeft = Math.ceil(timeLeft / (60 * 60 * 24));
    dueDate = `${daysLeft} Day${daysLeft > 1 ? 's' : ''}`;
  }

  const dateRange = formatDateRange(startDate, endDate);

  return (
    <div className="flex flex-row w-full justify-start items-center gap-[10px]">
      <SummaryBox label={t('participants')} value={responseCount.toString()} />
      <SummaryBox label={t('remainings')} value={dueDate} />
      {(startDate > 0 || endDate > 0) && (
        <SummaryBox label={t('date')} value={dateRange} />
      )}
    </div>
  );
}

function SummaryBox({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex flex-col w-fit h-fit px-[24px] py-[18px] bg-transparent border border-neutral-500 justify-center items-center gap-2.5 rounded-lg">
      <div className="text-sm font-semibold text-neutral-400">{label}</div>
      <div className="text-base font-bold text-text-primary">{value}</div>
    </div>
  );
}

function formatDateRange(start: number, end: number): string {
  const format = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}.${month}.${day}`;
  };

  return `${format(start)} - ${format(end)}`;
}
