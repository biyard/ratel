import { RelationType } from './types/relation-type';

export const route = {
  home: () => '/',
  myProfile: () => '/my-profile',
  explore: () => '/explore',
  settings: () => '/settings',
  myPosts: () => '/my-posts',
  drafts: () => '/drafts',
  teams: () => '/teams',
  groups: () => '/groups',

  login: () => '/login',
  signup: () => '/signup',
  connect: () => `/connect`,

  myNetwork: () => '/my-network',
  myFollower: (type: RelationType) => `/my-follower?type=${type}`,
  messages: () => '/messages',
  notifications: () => '/notifications',
  teamByUsername: (username: string) => `/teams/${username}/home`,
  teamGroups: (username: string) => `/teams/${username}/groups`,
  teamMembers: (username: string) => `/teams/${username}/members`,
  teamSettings: (username: string) => `/teams/${username}/settings`,
  teamDrafts: (username: string) => `/teams/${username}/drafts`,
  space: (spaceId: number | string) => `/spaces/${encodeURIComponent(spaceId)}`,
  commiteeSpaceById: (spaceId: number | string) =>
    `/spaces/${encodeURIComponent(spaceId)}`,
  deliberationSpaceById: (spaceId: number | string) =>
    `/spaces/${encodeURIComponent(spaceId)}`,
  noticeSpaceById: (spaceId: number | string) =>
    `/spaces/${encodeURIComponent(spaceId)}`,
  threadByFeedId: (feedId: number | string) => {
    return `/threads/${encodeURIComponent(feedId)}`;
  },
  discussionById: (spaceId: number, discussionId: number) =>
    `/spaces/${spaceId}/discussions/${discussionId}`,

  telegramSprintLeague: (space_id: number | string) =>
    `/telegram/sprint-league/${encodeURIComponent(space_id)}`,
  telegramSubscribe: (chat_id: number, lang?: string) => {
    return `/telegram/subscribe?chat_id=${chat_id}${lang ? `&lang=${lang}` : ''}`;
  },
};
