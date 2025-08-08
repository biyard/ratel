'use client';

import { ArrowLeft } from '@/components/icons';
import { UserType } from '@/lib/api/models/user';
import { getTimeAgo } from '@/lib/time-utils';
import Image from 'next/image';
import { BadgeIcon } from '@/components/icons';
import { useRouter } from 'next/navigation';
import { useNewsByID } from '@/app/(social)/_hooks/news';

export default function NewsHeader({ news_id }: { news_id: number }) {
  const { data: news } = useNewsByID(news_id);
  const router = useRouter();
  return (
    <div className="flex flex-col w-full gap-2.5">
      <button onClick={router.back}>
        <ArrowLeft className="[&>path]:stroke-white light:[&>path]:stroke-neutral-700" />
      </button>

      <div>
        <h2 className="text-2xl font-bold text-white light:text-neutral-700">
          {news?.title}
        </h2>
      </div>
      <div className="flex flex-row justify-between">
        <ProposerProfile
          profileUrl={''}
          proposerName={news?.title ?? ''}
          userType={UserType.Individual}
        />
        <div className="font-light text-white text-sm/[14px] light:text-neutral-700">
          {news?.created_at !== undefined ? getTimeAgo(news.created_at) : ''}
        </div>
      </div>
    </div>
  );
}

export function ProposerProfile({
  profileUrl = '',
  proposerName = '',
  userType = UserType.Individual,
}: {
  profileUrl: string;
  proposerName: string;
  userType: UserType;
}) {
  return (
    <div className="flex flex-row w-fit gap-2 justify-between items-center">
      {profileUrl && profileUrl !== '' ? (
        <Image
          src={profileUrl}
          alt={proposerName}
          width={20}
          height={20}
          className={
            userType == UserType.Team
              ? 'rounded-lg object-cover object-top w-6.25 h-6.25'
              : 'rounded-full object-cover object-top w-6.25 h-6.25'
          }
        />
      ) : (
        <div className="w-6.25 h-6.25 rounded-full border border-neutral-500 bg-neutral-600" />
      )}
      <div className="font-semibold text-white text-sm/[20px] light:text-neutral-700">
        {proposerName}
      </div>
      <BadgeIcon />
    </div>
  );
}
