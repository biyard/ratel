import { apiFetch } from './apiFetch';
import { Feed } from './models/feeds';
import { Space } from './models/spaces';
import { config } from '@/config';
import {
  QK_GET_FEED_BY_FEED_ID,
  QK_GET_REDEEM_CODE,
  QK_GET_SPACE_BY_SPACE_ID,
} from '@/constants';

import { RedeemCode } from './models/redeem-code';
import { ratelApi } from './ratel_api';
import { getServerQueryClient } from '../query-utils.server';
import { logger } from '../logger';

async function getDataFromServer<T>(
  key: (string | number)[],
  url: string,
): Promise<{ key: (string | number)[]; data: T | null }> {
  const queryClient = await getServerQueryClient();

  const data = queryClient.getQueryData<T | null>(key);

  if (data) {
    logger.debug('getDataFromServer: using cached data', key);
    return { key, data };
  }

  const res = await apiFetch<T | null>(`${config.api_url}${url}`, {
    ignoreError: true,
  });

  if (res.data) {
    queryClient.setQueryData(key, res.data);
  }

  return {
    key,
    data: res.data,
  };
}

export function getSpaceById(
  id: number,
): Promise<{ key: (string | number)[]; data: Space | null }> {
  return getDataFromServer<Space>(
    [QK_GET_SPACE_BY_SPACE_ID, id],
    ratelApi.spaces.getSpaceBySpaceId(id),
  );
}

export function getRedeemCode(
  meta_id: number,
): Promise<{ key: (string | number)[]; data: RedeemCode | null }> {
  return getDataFromServer<RedeemCode>(
    [QK_GET_REDEEM_CODE, meta_id],
    ratelApi.spaces.getSpaceRedeemCodes(meta_id),
  );
}

export async function getFeedById(
  id: number,
): Promise<{ key: (string | number)[]; data: Feed | null }> {
  return getDataFromServer<Feed>(
    [QK_GET_FEED_BY_FEED_ID, id],
    ratelApi.feeds.getFeedsByFeedId(id),
  );
}
