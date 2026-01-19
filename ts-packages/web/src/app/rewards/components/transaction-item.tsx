import { RewardsI18n } from '../rewards-page-i18n';
import { PointTransactionResponse, TransactionType } from '../types';
import { formatDistanceToNow } from 'date-fns';
import Card from '@/components/card';

interface TransactionItemProps {
  i18n: RewardsI18n;
  transaction: PointTransactionResponse;
  formatPoints: (points: number) => string;
}

export function TransactionItem({
  i18n,
  transaction,
  formatPoints,
}: TransactionItemProps) {
  const isReceived = transaction.transaction_type === TransactionType.Award;
  const timeAgo = formatDistanceToNow(new Date(transaction.created_at), {
    addSuffix: true,
  });

  return (
    <Card
      data-testid="transaction-item"
      className="rounded "
      variant="outlined"
    >
      <div className="flex items-center justify-between w-full">
        {/* Left side: Transaction info */}
        <div className="flex flex-col gap-0.5">
          <div className="flex items-center gap-2.5">
            <span
              className={`text-[15px] font-medium ${
                isReceived ? 'text-green-500' : 'text-red-500'
              }`}
            >
              {isReceived ? i18n.received : i18n.spent}
            </span>
            <div className="flex items-center">
              <div className="w-5 h-5 rounded-full bg-primary mr-1" />
              <span className="text-[15px] font-medium text-white">
                {isReceived ? '' : '-'}
                {formatPoints(transaction.amount)} P
              </span>
            </div>
          </div>
          <div className="flex items-center gap-2.5">
            <span className="text-sm font-semibold text-text-primary">
              {i18n.from}
            </span>
            <div className="flex items-center gap-1">
              <div className="w-3 h-3 rounded-full bg-bg" />
              <span className="text-sm font-semibold text-white">
                {transaction.description || 'Ratel'}
              </span>
            </div>
          </div>
        </div>

        {/* Right side: Time and chevron */}
        <div className="flex items-center gap-1">
          <span className="text-sm font-medium text-text-primary">
            {timeAgo}
          </span>
        </div>
      </div>
    </Card>
  );
}
