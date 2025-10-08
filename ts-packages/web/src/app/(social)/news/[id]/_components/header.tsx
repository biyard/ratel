import { ArrowLeft } from '@/components/icons';
import { UserType } from '@/lib/api/models/user';
import { getTimeAgo } from '@/lib/time-utils';
import { BadgeIcon } from '@/components/icons';
import { useNewsByID } from '@/app/(social)/_hooks/news';
import { useNavigate } from 'react-router';

export default function NewsHeader({ news_id }: { news_id: number }) {
  const { data: news } = useNewsByID(news_id);
  const router = useNavigate();
  return (
    <div className="flex flex-col w-full gap-2.5">
      <button onClick={() => router(-1)}>
        <ArrowLeft className="[&>path]:stroke-text-primary" />
      </button>

      <div>
        <h2 className="text-2xl font-bold text-text-primary">{news?.title}</h2>
      </div>
      <div className="flex flex-row justify-between">
        <ProposerProfile
          profileUrl={''}
          proposerName={news?.title ?? ''}
          userType={UserType.Individual}
        />
        <div className="font-light text-text-primary text-sm/[14px]">
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
        <img
          src={profileUrl}
          alt={proposerName}
          className={
            userType == UserType.Team
              ? 'rounded-lg object-cover object-top w-6.25 h-6.25'
              : 'rounded-full object-cover object-top w-6.25 h-6.25'
          }
        />
      ) : (
        <div className="w-6.25 h-6.25 rounded-full bg-profile-bg" />
      )}
      <div className="font-semibold text-text-primary text-sm/[20px]">
        {proposerName}
      </div>
      <BadgeIcon />
    </div>
  );
}
