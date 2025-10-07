import { apiFetch } from './apiFetch';
import { Feed, FeedStatus, FeedV2 } from './models/feeds';
import { Space } from './models/spaces';
import { config } from '@/config';
import {
  QK_GET_FEED_BY_FEED_ID,
  QK_GET_HOME_DATA,
  QK_GET_NETWORK,
  QK_GET_POSTS,
  QK_GET_POSTS_BY_USER_ID,
  QK_GET_PROMOTION,
  QK_GET_REDEEM_CODE,
  QK_GET_SPACE_BY_SPACE_ID,
  QK_GET_TEAM_BY_ID,
  QK_GET_TEAM_BY_USERNAME,
  QK_GET_PERMISSION,
  QK_USERS_GET_INFO,
  QK_GET_NEWS_BY_NEWS_ID,
  QK_GET_DELIBERATION_SPACE_BY_SPACE_ID,
  QK_GET_FEED_BY_FEED_ID_V2,
} from '@/constants';

import { RedeemCode } from './models/redeem-code';
import { ratelApi } from './ratel_api';
import { getServerQueryClient } from '../query-utils.server';
import { logger } from '../logger';
import { NetworkData } from './models/network';
import { Promotion } from './models/promotion';
import { User } from './models/user';
import { QueryResponse } from './models/common';
import { Team } from './models/team';
import { HomeGatewayResponse } from './models/home';
import { InfiniteData } from '@tanstack/react-query';
import { GroupPermission } from './models/group';
import { Permission } from './models/permission';
import { NewsDetailItem } from './models/news';
import { DeliberationSpace } from './ratel/spaces/deliberation-spaces.v3';

async function getDataFromServer<T>(
  key: (string | number)[],
  url: string,
  force = false,
): Promise<{ key: (string | number)[]; data: T | null }> {
  logger.debug('getDataFromServer: Starting fetching data', {
    key,
    url,
  });
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

  logger.debug('getDataFromServer: fetched data from server', {
    key,
    url,
  });

  return {
    key,
    data: res.data,
  };
}

export function getTeamByUsername(username: string) {
  return getDataFromServer<Team>(
    [QK_GET_TEAM_BY_USERNAME, username],
    ratelApi.teams.getTeamByUsername(username),
  );
}

export function getTeamById(user_id: number) {
  return getDataFromServer<Team>(
    [QK_GET_TEAM_BY_ID, user_id],
    ratelApi.teams.getTeamById(user_id),
  );
}

export function getPostByUserId(
  user_id: number,
  page: number,
  size: number,
  status: FeedStatus = FeedStatus.Published,
) {
  return getDataFromServer<Feed>(
    [QK_GET_POSTS_BY_USER_ID, user_id, page, size, status],
    ratelApi.feeds.getPostsByUserId(user_id, page, size, status),
  );
}

export function getFeedByIdV2(id: string) {
  const feedPk = 'FEED%23' + id;

  return getDataFromServer<FeedV2>(
    [QK_GET_FEED_BY_FEED_ID_V2, feedPk],
    ratelApi.feeds.getFeedById(feedPk),
  );
}

export function getDeliberationSpaceById(id: string) {
  const spacePk = 'DELIBERATION_SPACE%23' + id;

  return getDataFromServer<DeliberationSpace>(
    [QK_GET_DELIBERATION_SPACE_BY_SPACE_ID, spacePk],
    ratelApi.spaces.getDeliberationSpaceBySpaceId(spacePk),
    true,
  );
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

export async function listNews(): Promise<{
  key: (string | number)[];
  data: NewsDetailItem | null;
}> {
  return getDataFromServer<NewsDetailItem>(
    [QK_GET_NEWS_BY_NEWS_ID],
    ratelApi.news.getNews(1, 3),
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

// TODO: Update to use v3 permissions API with team username
export function getPermission(
  teamUsername: string,
  permission: GroupPermission,
): Promise<{ key: (string | number)[]; data: Permission | null }> {
  return getDataFromServer<Permission>(
    [QK_GET_PERMISSION, teamUsername, permission],
    ratelApi.permissions.getPermissions(teamUsername, permission),
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

export async function prefetchPostInfinite(pageSize: number) {
  const queryClient = await getServerQueryClient();

  const page = 1;
  const res = await apiFetch<QueryResponse<Feed> | null>(
    `${config.api_url}${ratelApi.feeds.getPosts(page, pageSize)}`,
    {
      ignoreError: true,
      cache: 'no-store',
    },
  );

  const items = res.data?.items ?? [];
  const total_count = res.data?.total_count ?? 0;

  const infiniteData: InfiniteData<QueryResponse<Feed>> = {
    pages: [
      {
        ...(res.data ?? {}),
        items,
        total_count,
      },
    ],
    pageParams: [page],
  };

  queryClient.setQueryData<InfiniteData<QueryResponse<Feed>>>(
    [QK_GET_POSTS, pageSize],
    infiniteData,
  );

  return {
    key: [QK_GET_POSTS, pageSize],
    data: infiniteData,
  };
}

export async function getHomeData(
  feedLimit?: number,
  newsLimit?: number,
): Promise<{
  key: (string | number)[];
  data: HomeGatewayResponse | null;
}> {
  return getDataFromServer<HomeGatewayResponse>(
    [QK_GET_HOME_DATA, feedLimit ?? '', newsLimit ?? ''],
    ratelApi.home.getHomeData(feedLimit, newsLimit),
  );
}
