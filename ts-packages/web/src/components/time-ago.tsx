import { getTimeAgo } from '@/lib/time-utils';

export default function TimeAgo({ timestamp }: { timestamp: number }) {
  return (
    <p className="text-sm align-middle font-light text-text-primary">
      {getTimeAgo(timestamp)}
    </p>
  );
}
