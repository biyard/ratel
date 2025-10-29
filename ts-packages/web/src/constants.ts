import { FeedStatus } from './features/posts/types/post';

// LocalStorage keys
export const SK_IDENTITY_KEY = 'identity';
export const SK_ANONYMOUS_IDENTITY_KEY = 'anonymous_identity';

// // Query keys
export const QK_USERS_GET_INFO = 'user-get-info';

export const QK_GET_NOTIFICATIONS = 'get-notifications';

export const QK_GET_NETWORK = 'get-networks';

export const QK_GET_NEWS_BY_NEWS_ID = 'get-news-by-news-id';

export const QK_GET_TEAM_BY_USERNAME = 'get-team-by-username';
export const QK_GET_TEAM_BY_PK = 'get-team-by-pk-v3';

export const QK_GET_REDEEM_CODE = 'get-redeem-code';
export const QK_GET_USER_BADGE = 'get-user-badge';
export const QK_GET_PROMOTION = 'get-promotion';

// // Quiz-related query keys

// FIXME: Move to spaces/:spacePk/quizs/
export const QK_LATEST_QUIZ_ATTEMPT = 'latest-quiz-attempt';
export const QK_QUIZ_ATTEMPTS = 'quiz-attempts';
export const QK_QUIZ_ATTEMPT = 'quiz-attempt';
export const QK_QUIZ = 'quiz';

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

// Note: Structured Query Key Hierarchy for Space Features
//
// Each space feature should follow a consistent, hierarchical query key structure
// to enable efficient cache invalidation and refreshing. This hierarchy allows
// invalidating data at different levels (e.g., entire space or specific features).
//
// Key Structure:
// - Base: QK_SPACES > space_pk
// - Feature-specific: QK_SPACES > space_pk > 'feature_name' > sub_keys
//
// Examples:
// - Polls: QK_SPACES > space_pk > 'polls' > 'survey' | 'poll'
//   - Invalidate all polls: QK_SPACES > space_pk > 'polls'
//   - Invalidate specific poll: QK_SPACES > space_pk > 'polls' > poll_pk
//
// - Discussions: QK_SPACES > space_pk > 'discussions' > discussion_pk > 'participants' | 'meeting'
//   - Invalidate all discussions: QK_SPACES > space_pk > 'discussions'
//   - Invalidate specific discussion: QK_SPACES > space_pk > 'discussions' > discussion_pk
//
// Benefits:
// - Entire space invalidation: Use QK_SPACES > space_pk
// - Feature-level invalidation: Use QK_SPACES > space_pk > 'feature_name'
// - Granular control: Target specific sub-keys for precise updates

const QK_SPACES = 'spaces';

export const spaceKeys = {
  all: [QK_SPACES] as const,

  lists: () => [...spaceKeys.all, 'list'] as const,

  details: () => [...spaceKeys.all, 'detail'] as const,
  detail: (spacePk: string) => [...spaceKeys.details(), spacePk] as const,

  sprint_leagues: (spacePk: string) =>
    [...spaceKeys.detail(spacePk), 'sprint_leagues'] as const,

  polls: (spacePk: string) => [...spaceKeys.detail(spacePk), 'polls'] as const,
  poll: (spacePk: string, pollSk: string = 'default') =>
    [...spaceKeys.polls(spacePk), pollSk] as const,
  poll_summary: (spacePk: string, pollSk: string) =>
    [...spaceKeys.poll(spacePk, pollSk), 'summary'] as const,

  files: (spacePk: string) => [...spaceKeys.detail(spacePk), 'files'] as const,

  recommendations: (spacePk: string) =>
    [...spaceKeys.detail(spacePk), 'recommendations'] as const,

  discussions: (spacePk: string) =>
    [...spaceKeys.detail(spacePk), 'discussions'] as const,
  discussion: (spacePk: string, discussionPk: string) =>
    [...spaceKeys.discussions(spacePk), discussionPk] as const,
  discussion_participants: (spacePk: string, discussionPk: string) =>
    [...spaceKeys.discussion(spacePk, discussionPk), 'participants'] as const,
  discussion_meeting: (spacePk: string, discussionPk: string) =>
    [...spaceKeys.discussion(spacePk, discussionPk), 'meeting'] as const,

  panels: (spacePk: string) =>
    [...spaceKeys.detail(spacePk), 'panels'] as const,
  panel: (spacePk: string, panelPk: string) =>
    [...spaceKeys.panels(spacePk), panelPk] as const,
};

export const QK_MEMBERSHIPS = 'memberships';
