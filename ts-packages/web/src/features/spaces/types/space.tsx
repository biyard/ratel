import { SpaceStatus } from './space-common';
import { SpaceType } from './space-type';
import { TeamGroupPermissions } from '@/features/auth/utils/team-group-permissions';

export class Space {
  readonly permissions: TeamGroupPermissions;

  constructor(
    public pk: string,
    public sk: string,
    public title: string,
    public content: string,
    public created_at: number,
    public updated_at: number,
    public urls: string[],
    public spaceType: SpaceType,
    public features: string[],
    public status: SpaceStatus | null,
    permissions: bigint,
  ) {
    this.permissions = new TeamGroupPermissions(permissions);
  }

  isAdmin() {
    return this.permissions.isAdmin();
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromJson(json: any): Space {
    return new Space(
      json.pk,
      json.sk,
      json.title,
      json.content,
      json.created_at,
      json.updated_at,
      json.urls || [],
      json.space_type,
      json.features || [],
      json.status || SpaceStatus.InProgress,
      json.permissions,
    );
  }
}
