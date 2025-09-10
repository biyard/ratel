import React, { useCallback, useEffect, useState } from 'react';
import SuggestionItem from './suggestions-items';
import BlackBox from './black-box';
import Link from 'next/link';
import { route } from '@/route';
import { ChevronRight } from 'lucide-react';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi, useNetwork } from '@/lib/api/ratel_api';
import { followRequest } from '@/lib/api/models/networks/follow';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { Follower } from '@/lib/api/models/network';
import { useTranslations } from 'next-intl';

export default function Suggestions() {
  const t = useTranslations('Home');
  const { post } = useApiCall();
  const network = useNetwork();

  const [suggestions, setSuggestions] = useState<Follower[]>([]);

  useEffect(() => {
    if (
      !network.data ||
      !Array.isArray(network.data.suggested_teams) ||
      !Array.isArray(network.data.suggested_users)
    ) {
      return;
    }

    const items = [
      ...network.data.suggested_teams.slice(0, 3),
      ...network.data.suggested_users.slice(0, 3),
    ];

    for (let i = items.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [items[i], items[j]] = [items[j], items[i]];
    }

    setSuggestions(items.slice(0, 3));
  }, [network.data]);

  const handleFollow = useCallback(
    async (userId: number) => {
      try {
        await post(ratelApi.networks.follow(userId), followRequest());
        showSuccessToast('Successfully followed user');
        network.refetch();
      } catch (err) {
        showErrorToast('Failed to follow user');
        logger.error('Failed to follow user:', err);
      }
    },
    [post, network],
  );

  if (!suggestions || suggestions.length === 0) {
    return <></>; // No suggestions to display
  }

  return (
    <BlackBox>
      <h3 className="font-medium mb-3 text-text-primary">{t('suggested')}</h3>
      <div className="flex flex-col gap-[35px]">
        {suggestions.map((user) => (
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
    </BlackBox>
  );
}
