import { formatDistanceToNow } from 'date-fns';
import SpaceArtworkTrade, { TradeType } from '../types/space-artwork-trade';
import { LinksShareLink1 } from '@/components/icons';

interface SpaceArtworkTradeTableProps {
  items: SpaceArtworkTrade[];
}

const formatAddress = (address: string) => {
  if (address === '0x0' || address === 'Oracle') {
    return address;
  }
  if (address.length > 10) {
    return `${address.slice(0, 6)}..${address.slice(-4)}`;
  }
  return address;
};

const getActionText = (trade: SpaceArtworkTrade): string => {
  if (trade.tradeType === TradeType.Mint) {
    return 'Created';
  }
  return 'Transfer';
};

const getActionColor = (trade: SpaceArtworkTrade): string => {
  if (trade.tradeType === TradeType.Mint) {
    return 'text-text-primary';
  }
  return 'text-gray-700';
};

export default function SpaceArtworkTradeTable({
  items,
}: SpaceArtworkTradeTableProps) {
  return (
    <div className="w-full border border-gray-700 rounded-lg overflow-hidden">
      {/* Header */}
      <div className="flex bg-transparent border-b border-gray-700">
        <div className="flex-1 px-5 py-3">
          <span className="text-sm font-semibold text-gray-500">Action</span>
        </div>
        <div className="flex-1 px-5 py-3">
          <span className="text-sm font-semibold text-gray-500">Value</span>
        </div>
        <div className="flex-1 px-5 py-3">
          <span className="text-sm font-semibold text-gray-500">From</span>
        </div>
        <div className="flex-1 px-5 py-3">
          <span className="text-sm font-semibold text-gray-500">To</span>
        </div>
        <div className="flex-1 px-5 py-3">
          <span className="text-sm font-semibold text-gray-500">Time</span>
        </div>
      </div>

      {/* Rows */}
      {items.map((item, index) => (
        <div
          key={`${item.pk}-${item.sk}-${index}`}
          className={`flex ${index !== items.length - 1 ? 'border-b border-gray-700' : ''}`}
        >
          <div className="flex-1 px-5 py-3">
            <span className={`text-[15px] font-bold ${getActionColor(item)}`}>
              {getActionText(item)}
            </span>
          </div>
          <div className="flex-1 px-5 py-3">
            <span className="text-[15px] font-bold text-gray-400">-</span>
          </div>
          <div className="flex-1 px-5 py-3">
            <span className="text-[15px] font-bold text-gray-400">
              {formatAddress(item.fromAddress)}
            </span>
          </div>
          <div className="flex-1 px-5 py-3">
            <span className="text-[15px] font-bold text-gray-400">
              {formatAddress(item.toAddress)}
            </span>
          </div>
          <div className="flex-1 px-5 py-3 flex items-center gap-2">
            <span className="text-[15px] font-bold text-gray-400">
              {formatDistanceToNow(new Date(item.createdAt), {
                addSuffix: true,
              })}
            </span>
            {item.transactionHash && (
              <LinksShareLink1 className="w-5 h-5 text-gray-500" />
            )}
          </div>
        </div>
      ))}
    </div>
  );
}
