import { useCallback } from 'react';
import SuggestionItem from './suggestions-items';
import DisableBorderCard from './disable-border-card';
import Link from 'next/link';
import { route } from '@/route';
import { ChevronRight } from 'lucide-react';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { followRequest } from '@/lib/api/models/networks/follow';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { type Follower } from '@/lib/api/models/network';
import { useTranslation } from 'react-i18next';

interface HomeSuggestionsProps {
  suggestedUsers: Follower[];
}

export default function HomeSuggestions({
  suggestedUsers,
}: HomeSuggestionsProps) {
  const { t } = useTranslation('Home');
  const { post } = useApiCall();

  const handleFollow = useCallback(
    async (userId: number) => {
      try {
        await post(ratelApi.networks.follow(userId), followRequest());
        showSuccessToast('Successfully followed user');
        // Note: In the server-side approach, we might want to trigger a refresh
        // or update the local state differently
      } catch (err) {
        showErrorToast('Failed to follow user');
        logger.error('Failed to follow user:', err);
      }
    },
    [post],
  );

  // Don't render if no suggestions
  if (!suggestedUsers || suggestedUsers.length === 0) {
    return null;
  }

  // Take maximum 3 suggestions for display (matching original behavior)
  const displaySuggestions = suggestedUsers.slice(0, 3);

  return (
    <DisableBorderCard>
      <h3 className="font-medium mb-3 text-text-primary">{t('suggested')}</h3>
      <div className="flex flex-col gap-[35px]">
        {displaySuggestions.map((user) => (
          <SuggestionItem key={user.id} user={user} onFollow={handleFollow} />
        ))}
      </div>
      <Link
        href={route.myNetwork()}
        className="mt-5 text-xs text-more-text flex items-center hover:text-card-meta transition-colors"
        aria-label="View all suggestions"
      >
        <span>{t('view_all')}</span>
        <ChevronRight size={14} className="[&>path]:stroke-more-text" />
      </Link>
    </DisableBorderCard>
  );
}
