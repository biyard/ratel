import { logger } from '@/lib/logger';

export class TeamGroupPermissions {
  private readonly set: Set<TeamGroupPermission>;

  constructor(public permissions: bigint) {
    this.set = new Set<TeamGroupPermission>();

    for (const perm of ALL_PERMISSIONS) {
      if ((BigInt(permissions) & (1n << BigInt(perm))) !== 0n) {
        logger.debug('Added permission: ', perm);
        this.set.add(perm);
      }
    }
  }

  has(permission: TeamGroupPermission): boolean {
    return this.set.has(permission);
  }

  isAdmin(): boolean {
    return this.has(TeamGroupPermission.TeamAdmin);
  }
}

export enum TeamGroupPermission {
  // Post
  PostRead = 0,
  PostWrite = 1, // for creating a post with team, require PostWrite + PostEdit
  PostEdit = 2,
  PostDelete = 3,

  // Space
  SpaceRead = 10,
  SpaceWrite = 11,
  SpaceEdit = 12,
  SpaceDelete = 13,

  // Team
  TeamAdmin = 20, // change group permissions + all others
  TeamEdit = 21, // edit team info, add/remove group
  GroupEdit = 22, // edit group members, change group info

  // Admin
  ManagePromotions = 62,
  ManageNews = 63,
}

export const ALL_PERMISSIONS: readonly TeamGroupPermission[] = [
  TeamGroupPermission.PostRead,
  TeamGroupPermission.PostWrite,
  TeamGroupPermission.PostEdit,
  TeamGroupPermission.PostDelete,

  TeamGroupPermission.SpaceRead,
  TeamGroupPermission.SpaceWrite,
  TeamGroupPermission.SpaceEdit,
  TeamGroupPermission.SpaceDelete,

  TeamGroupPermission.TeamAdmin,
  TeamGroupPermission.TeamEdit,
  TeamGroupPermission.GroupEdit,

  TeamGroupPermission.ManagePromotions,
  TeamGroupPermission.ManageNews,
] as const;
