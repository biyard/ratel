import { I18nFunction } from '.';

export interface SummaryReportProps {
  t: I18nFunction;
  totalResponses: number;
  startedAt: number;
  endedAt: number;
}

export default function SummaryReport({
  t,
  totalResponses,
  startedAt,
  endedAt,
}: SummaryReportProps) {
  const now = Date.now();
  const timeLeft = endedAt - now;

  let dueDate = '';
  if (timeLeft <= 0) {
    dueDate = '0 Day';
  } else {
    const daysLeft = Math.ceil(timeLeft / (60 * 60 * 24));
    dueDate = `${daysLeft} Day${daysLeft > 1 ? 's' : ''}`;
  }

  const dateRange = formatDateRange(startedAt, endedAt);

  return (
    <div className="flex flex-row w-full justify-start items-center gap-[10px] max-tablet:hidden">
      <SummaryBox label={t('participants')} value={totalResponses.toString()} />
      <SummaryBox label={t('remainings')} value={dueDate} />
      {(startedAt > 0 || endedAt > 0) && (
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

function formatDateRange(startedAt: number, endedAt: number): string {
  //Milliseconds
  const format = (timestamp: number) => {
    const date = new Date(timestamp);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}.${month}.${day}`;
  };

  return `${format(startedAt)} - ${format(endedAt)}`;
}
