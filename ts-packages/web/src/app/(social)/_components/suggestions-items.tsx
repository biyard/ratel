import Image from 'next/image';
import { UserType } from '@/lib/api/models/user';
import { useTranslations } from 'next-intl';

type SuggestionItemProps = {
  user: {
    id: number;
    profile_url?: string;
    user_type: UserType;
    username: string;
    email: string;
  };
  onFollow: (userId: number) => void;
};

export default function SuggestionItem({
  user,
  onFollow,
}: SuggestionItemProps) {
  const t = useTranslations('Home');
  const isTeam = user.user_type === UserType.Team;
  const imageClass = isTeam ? 'rounded-lg' : 'rounded-full';

  return (
    <div className="flex flex-col items-start gap-3">
      <div className="flex flex-row gap-2.5">
        {user.profile_url && user.profile_url !== '' ? (
          <Image
            width={32}
            height={32}
            src={user.profile_url}
            alt={`${user.username}'s profile`}
            className={`w-8 h-8 object-cover ${imageClass}`}
          />
        ) : (
          <div className={`w-8 h-8 bg-profile-bg ${imageClass}`} />
        )}
        <div className="flex-1">
          <div className="font-medium text-base text-text-primary">
            {user.username}
          </div>

          <button
            className="font-bold text-xs text-follow-button-text rounded-full bg-follow-button-bg px-4 py-2 mt-2 hover:bg-follow-button-bg/80 transition-colors"
            onClick={() => onFollow(user.id)}
            aria-label={`Follow ${user.username}`}
          >
            + {t('follow')}
          </button>
        </div>
      </div>
    </div>
  );
}
