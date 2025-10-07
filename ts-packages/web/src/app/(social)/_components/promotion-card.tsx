import Link from 'next/link';
import { route } from '@/route';
import { useTranslations } from 'next-intl';
import { TopPromotionResponse } from '@/lib/api/ratel/promotions.v3';

type PromotionCardProps = {
  promotion: TopPromotionResponse;
};

export default function PromotionCard({ promotion }: PromotionCardProps) {
  const t = useTranslations('Home');
  const { feed_id, image_url, name, space_id, space_type } = promotion;
  const getHref = () => {
    if (!space_id) return route.threadByFeedId(feed_id);

    return space_type === 3
      ? route.deliberationSpaceById(space_id)
      : route.commiteeSpaceById(space_id);
  };

  return (
    <div className="flex flex-col gap-2.5">
      <h3 className="font-bold text-text-primary text-[15px]/[20px]">
        {t('hot_promotion')}
      </h3>
      <Link
        href={getHref()}
        className="flex items-center gap-2.5 hover:bg-btn-hover rounded p-2 transition-colors"
        aria-label={`View ${promotion.name} promotion`}
      >
        <img
          src={image_url}
          alt={name}
          className="w-25 h-25 rounded object-cover cursor-pointer"
        />
        <div>
          <div className="font-medium text-text-primary text-base/[25px]">
            {name}
          </div>
        </div>
      </Link>
    </div>
  );
}
