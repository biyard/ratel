import Image from 'next/image';
import Link from 'next/link';
import { route } from '@/route';

type PromotionCardProps = {
  promotion: {
    image_url: string;
    name: string;
  };
  feed?: {
    id: number;
    spaces?: Array<{
      id: number;
      space_type: number;
    }>;
  };
};

export default function PromotionCard({ promotion, feed }: PromotionCardProps) {
  const getHref = () => {
    if (!feed?.spaces?.length) return route.threadByFeedId(feed?.id || 0);

    return feed.spaces[0].space_type === 3
      ? route.deliberationSpaceById(feed.spaces[0].id)
      : route.commiteeSpaceById(feed.spaces[0].id);
  };

  return (
    <div className="flex flex-col gap-2.5">
      <h3 className="font-bold text-white text-[15px]/[20px]">Hot Promotion</h3>
      <Link
        href={getHref()}
        className="flex items-center gap-2.5 hover:bg-btn-hover rounded p-2 transition-colors"
        aria-label={`View ${promotion.name} promotion`}
      >
        <Image
          src={promotion.image_url}
          alt={promotion.name}
          width={60}
          height={60}
          className="rounded object-cover cursor-pointer"
          priority
        />
        <div>
          <div className="font-medium text-white text-base/[25px]">
            {promotion.name}
          </div>
        </div>
      </Link>
    </div>
  );
}
