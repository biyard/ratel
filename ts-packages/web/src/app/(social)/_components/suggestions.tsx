import React, { useCallback } from 'react';
import SuggestionItem from './suggestions-items';
import BlackBox from './black-box';
import { useHomeContext } from '../providers.client';
import Link from 'next/link';
import { route } from '@/route';
import { ChevronRight } from 'lucide-react';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi, useNetwork } from '@/lib/api/ratel_api';
import { followRequest } from '@/lib/api/models/networks/follow';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';

export default function Suggestions() {
  const { suggestions } = useHomeContext();
  const { post } = useApiCall();
  const network = useNetwork();
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
      <h3 className="font-medium mb-3">Suggested</h3>
      <div className="flex flex-col gap-[35px]">
        {suggestions.map((user) => (
          <SuggestionItem key={user.id} user={user} onFollow={handleFollow} />
        ))}
      </div>
      <Link
        href={route.myNetwork()}
        className="mt-5 text-xs text-gray-400 flex items-center hover:text-gray-300 transition-colors"
        aria-label="View all suggestions"
      >
        <span>View all</span>
        <ChevronRight size={14} />
      </Link>
    </BlackBox>
  );
}
