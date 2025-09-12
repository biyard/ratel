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
  space: (spaceId: number) => `/spaces/${spaceId}`,
  commiteeSpaceById: (spaceId: number) => `/spaces/${spaceId}`,
  deliberationSpaceById: (spaceId: number) => `/spaces/${spaceId}`,
  noticeSpaceById: (spaceId: number) => `/spaces/${spaceId}`,
  threadByFeedId: (feedId: number) => {
    return `/threads/${feedId}`;
  },
  discussionById: (spaceId: number, discussionId: number) =>
    `/spaces/${spaceId}/discussions/${discussionId}`,

  telegramSprintLeague: (space_id: number) =>
    `/telegram/sprint-league/${space_id}`,
  telegramSubscribe: (chat_id: number, lang?: string) => {
    return `/telegram/subscribe?chat_id=${chat_id}${lang ? `&lang=${lang}` : ''}`;
  },
};
