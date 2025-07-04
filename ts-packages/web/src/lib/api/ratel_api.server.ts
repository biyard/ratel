import { apiFetch } from './apiFetch';
import { Feed } from './models/feeds';
import { Space } from './models/spaces';
import { config } from '@/config';
import {
  QK_GET_FEED_BY_FEED_ID,
  QK_GET_NETWORK,
  QK_GET_POSTS,
  QK_GET_PROMOTION,
  QK_GET_REDEEM_CODE,
  QK_GET_SPACE_BY_SPACE_ID,
  QK_USERS_GET_INFO,
} from '@/constants';

import { RedeemCode } from './models/redeem-code';
import { ratelApi } from './ratel_api';
import { getServerQueryClient } from '../query-utils.server';
import { logger } from '../logger';
import { NetworkData } from './models/network';
import { Promotion } from './models/promotion';
import { User } from './models/user';
import { QueryResponse } from './models/common';

async function getDataFromApollonServer<T>(
  key: (string | number)[],
  url: string,
  force = false,
): Promise<{ key: (string | number)[]; data: T | null }> {
  const queryClient = await getServerQueryClient();

  if (!force) {
    const data = queryClient.getQueryData<T | null>(key);
    if (data) {
      logger.debug('getDataFromServer: using cached data', key);
      return { key, data };
    }
  }

  const res = await apiFetch<T | null>(`${config.api_url}${url}`, {
    ignoreError: true,
    cache: 'no-store',
  });

  if (res.data) {
    queryClient.setQueryData(key, res.data);
  }

  return {
    key,
    data: res.data,
  };
}

async function getDataFromServer<T>(
  key: (string | number)[],
  url: string,
  force = false,
): Promise<{ key: (string | number)[]; data: T | null }> {
  const queryClient = await getServerQueryClient();

  if (!force) {
    const data = queryClient.getQueryData<T | null>(key);
    if (data) {
      logger.debug('getDataFromServer: using cached data', key);
      return { key, data };
    }
  }

  const res = await apiFetch<T | null>(`${config.api_url}${url}`, {
    ignoreError: true,
    cache: 'no-store',
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
    true,
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

export async function getNetwork(): Promise<{
  key: (string | number)[];
  data: NetworkData | null;
}> {
  return getDataFromServer<NetworkData>(
    [QK_GET_NETWORK],
    ratelApi.networks.getNetworks(),
  );
}

export async function getPromotion(): Promise<{
  key: (string | number)[];
  data: Promotion | null;
}> {
  return getDataFromServer<Promotion>(
    [QK_GET_PROMOTION],
    ratelApi.promotions.get_promotions(),
  );
}

export async function getUserInfo(): Promise<{
  key: (string | number)[];
  data: User | null;
}> {
  return getDataFromServer<User>(
    [QK_USERS_GET_INFO],
    ratelApi.users.getUserInfo(),
  );
}

export async function getPosts(
  page: number,
  size: number,
): Promise<{
  key: (string | number)[];
  data: QueryResponse<Feed> | null;
}> {
  return getDataFromServer<QueryResponse<Feed>>(
    [QK_GET_POSTS, page, size],
    ratelApi.feeds.getPosts(page, size),
  );
}
