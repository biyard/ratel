import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';

export type TimeRangeDisplayProps = {
  startTimestampMillis: number;
  endTimestampMillis: number;
  timezone?: string;
  className?: string;
};

export function TimeRangeDisplay({
  startTimestampMillis,
  endTimestampMillis,
  timezone,
}: TimeRangeDisplayProps) {
  const tz = timezone || Intl.DateTimeFormat().resolvedOptions().timeZone;

  const formatDateTime = (timestampMillis: number) => {
    const date = new Date(timestampMillis);
    return new Intl.DateTimeFormat('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      timeZone: tz,
    }).format(date);
  };

  const startFormatted = formatDateTime(startTimestampMillis);
  const endFormatted = formatDateTime(endTimestampMillis);

  return (
    <Col className="text-xl max-mobile:text-sm px-2 max-w-150">
      <span className="self-start text-lg max-mobile:text-xs">{tz}</span>
      <Row className="justify-between">
        <span>{startFormatted}</span>
        <span className="text-neutral-500">→</span>
        <span>{endFormatted}</span>
      </Row>
    </Col>
  );
}
