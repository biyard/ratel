import { FeedStatus } from './lib/api/models/feeds';
import { SpaceType } from './types/space-type';

// LocalStorage keys
export const SK_IDENTITY_KEY = 'identity';
export const SK_ANONYMOUS_IDENTITY_KEY = 'anonymous_identity';

// Query keys
export const QK_USERS_GET_INFO = 'user-get-info';
export const QK_USERS_PROFILE = 'user-profile';
export const QK_ASSETS_GET_PRESIGNED_URL = 'assets-get-presigned-url';
export const QK_GET_POSTS = 'get-posts';
export const QK_GET_POSTS_BY_USER_ID = 'get-posts-by-user-id';
export const QK_GET_FEED_BY_FEED_ID = 'get-feeds-by-feed-id';
export const QK_GET_FEED_BY_FEED_ID_V2 = 'get-feeds-by-feed-id-v2';
export const QK_GET_NOTIFICATIONS = 'get-notifications';

export const QK_GET_NETWORK = 'get-networks';

export const QK_GET_NEWS_BY_NEWS_ID = 'get-news-by-news-id';

export const QK_GET_SPACE_BY_SPACE_ID = 'get-space-by-space-id';
export const QK_GET_DELIBERATION_SPACE_BY_SPACE_ID =
  'get-deliberation-space-by-space-id';
export const QK_GET_DISCUSSION_BY_DISCUSSION_ID =
  'get-discussion-by-discussion-id';
export const QK_GET_MEETING_BY_DISCUSSION_ID = 'get-meeting-by-discussion-id';

export const QK_GET_TEAM_BY_ID = 'get-team-by-id';
export const QK_GET_TEAM_BY_USERNAME = 'get-team-by-username';

export const QK_GET_REDEEM_CODE = 'get-redeem-code';
export const QK_GET_USER_BADGE = 'get-user-badge';
export const QK_GET_PROMOTION = 'get-promotion';
export const QK_GET_PERMISSION = 'get-permission';
export const QK_GET_TOTAL_USER = 'get-total-users';
export const QK_GET_USER_BY_EMAIL = 'get-user-by-emails';
export const QK_GET_USER_BY_USERNAME = 'get-user-by-usernames';

// Quiz-related query keys
export const QK_LATEST_QUIZ_ATTEMPT = 'latest-quiz-attempt';
export const QK_QUIZ_ATTEMPTS = 'quiz-attempts';
export const QK_QUIZ_ATTEMPT = 'quiz-attempt';
export const QK_QUIZ = 'quiz';
export const QK_QUIZ_ANSWERS = 'quiz-answers';

export const QK_GET_SPACE = 'get-spaces';

export const QK_GET_SPRINT_LEAGUE = 'get-sprint-leagues';

export const QK_GET_DAGIT = 'get-dagit';
export const QK_GET_ARTWORK = 'get-artwork';
export const QK_GET_ARTWORK_CERTIFICATE = 'get-artwork-certificate';

export const QK_GET_HOME_DATA = 'get-home-data';
export const QK_TOP_PROMOTION = 'top-promotion';

function sortObjectKeys<T extends object>(obj: T): T {
  const sortedKeys = Object.keys(obj).sort() as Array<keyof T>;
  return sortedKeys.reduce((result, key) => {
    result[key] = obj[key];
    return result;
  }, {} as T);
}

// Use This Pattern
const QK_FEEDS = 'feeds';

export const feedKeys = {
  all: [QK_FEEDS] as const,
  lists: () => [...feedKeys.all, 'list'] as const,
  // {username, status}
  // posts on Home: [list, status]
  // posts on a specific user or team: [list, username, status]
  // invalidate: login/logout
  // For my posts [list, username, FeedStatus.Published]
  list: (filters: { username?: string; status: FeedStatus }) => {
    const nonNullFilters = Object.fromEntries(
      Object.entries(filters).filter(([, value]) => value != null),
    );
    return [...feedKeys.lists(), sortObjectKeys(nonNullFilters)] as const;
  },
  drafts: (username: string) =>
    [...feedKeys.list({ username, status: FeedStatus.Draft })] as const,
  my_posts: (username: string) =>
    [...feedKeys.list({ username, status: FeedStatus.Published })] as const,
  details: () => [...feedKeys.all, 'detail'] as const,
  detail: (pk: string) => [...feedKeys.details(), pk] as const,
  repliesOfComment: (postPk: string, commentSk: string) =>
    [QK_FEEDS, 'comments', postPk, commentSk] as const,
};

const QK_SPACES = 'spaces';

export const spaceKeys = {
  all: [QK_SPACES] as const,
  // detail: (pk: string) => [...spaceKeys.all, 'detail', pk] as const,
  detail: (pk: string, type?: SpaceType) => {
    if (type) {
      return [...spaceKeys.all, 'detail', type, pk] as const;
    } else {
      return [...spaceKeys.all, 'detail', pk] as const;
    }
  },
};
