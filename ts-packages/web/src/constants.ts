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
export const QK_GET_NOTIFICATIONS = 'get-notifications';

export const QK_GET_NETWORK = 'get-networks';

export const QK_GET_NEWS_BY_NEWS_ID = 'get-news-by-news-id';

export const QK_GET_SPACE_BY_SPACE_ID = 'get-space-by-space-id';
export const QK_GET_DISCUSSION_BY_DISCUSSION_ID =
  'get-discussion-by-discussion-id';
export const QK_GET_MEETING_BY_DISCUSSION_ID = 'get-meeting-by-discussion-id';

export const QK_GET_TEAM_BY_ID = 'get-team-by-id';
export const QK_GET_TEAM_BY_USERNAME = 'get-team-by-username';

export const QK_GET_REDEEM_CODE = 'get-redeem-code';
export const QK_GET_USER_BADGE = 'get-user-badge';
export const QK_GET_PROMOTION = 'get-promotion';
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
  list: (filters: object) => {
    // 빈 필터 객체는 제외하여 불필요한 복잡성을 줄입니다.
    const nonNullFilters = Object.fromEntries(
      Object.entries(filters).filter(([, value]) => value != null),
    );
    return [...feedKeys.lists(), sortObjectKeys(nonNullFilters)] as const;
  },
  details: () => [...feedKeys.all, 'detail'] as const,
  detail: (id: number) => [...feedKeys.details(), id] as const,
};

// Example Usage
// feedKeys.list({ feed_type: 'news', industry_id: 5 });
// feedKeys.detail(1);
