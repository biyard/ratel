'use client';

import { ArrowLeft } from '@/components/icons';
import { UserType } from '@/lib/api/models/user';
import { getTimeAgo } from '@/lib/time-utils';
import Image from 'next/image';
import { BadgeIcon } from '@/components/icons';
import { useRouter } from 'next/navigation';
import { useNewsByID } from '@/app/(social)/_hooks/news';
import { useUserInfo } from '@/lib/api/hooks/users';

export default function NewsHeader({ news_id }: { news_id: number }) {
  const { data: user } = useUserInfo();
  const { data: news } = useNewsByID(news_id);

  const router = useRouter();

  if (!user || !news) {
    return null; // Or a spinner/loading state
  }

  return (
    <div className="flex flex-col w-full gap-2.5">
      <button onClick={router.back}>
        <ArrowLeft />
      </button>

      <div>
        <h2 className="text-2xl font-bold">{news.html_content}</h2>
      </div>

      <div className="flex flex-row justify-between">
        <ProposerProfile
          profileUrl={user.profile_url || ''}
          proposerName={user.nickname || user.username || ''}
          userType={user.user_type ?? UserType.Individual}
        />
        <div className="font-light text-white text-sm/[14px]">
          {getTimeAgo(news.created_at)}
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
    <div className="flex flex-row w-fit gap-2 items-center">
      <Image
        src={profileUrl || '/default-profile.png'}
        alt={proposerName}
        width={25}
        height={25}
        className={
          userType === UserType.Team
            ? 'rounded-[8px] object-cover object-top w-[25px] h-[25px]'
            : 'rounded-full object-cover object-top w-[25px] h-[25px]'
        }
      />
      <div className="font-semibold text-white text-sm/[20px]">
        {proposerName}
      </div>
      <BadgeIcon />
    </div>
  );
}
