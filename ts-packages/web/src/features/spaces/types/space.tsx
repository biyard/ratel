import { UserType } from '@/lib/api/models/user';
import {
  normalizeVisibility,
  SpacePublishState,
  SpaceStatus,
  SpaceVisibility,
} from './space-common';
import { SpaceType } from './space-type';
import { TeamGroupPermissions } from '@/features/auth/utils/team-group-permissions';
import { BoosterType } from './booster-type';

export class Space {
  readonly permissions: TeamGroupPermissions;

  constructor(
    public pk: string,
    public sk: string,
    public title: string,
    public content: string,
    public createdAt: number,
    public updatedAt: number,
    public urls: string[],
    public spaceType: SpaceType,
    public features: string[],
    public status: SpaceStatus | null,
    public authorProfileUrl: string,
    public authorDisplayName: string,
    public authorUsername: string,
    public authorType: UserType,
    public certified: boolean,

    public likes: number = 0,
    public comments: number = 0,
    public shares: number = 0,
    public rewards: number | undefined,
    public visibility: SpaceVisibility,
    public publishState: SpacePublishState,

    public booster: BoosterType | undefined,

    permissions: bigint,
  ) {
    this.permissions = new TeamGroupPermissions(permissions);
  }

  isAdmin() {
    return this.permissions.isAdmin();
  }

  get isDraft() {
    return this.publishState === SpacePublishState.Draft;
  }

  get isPublic() {
    return normalizeVisibility(this.visibility).type === 'public';
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
      json.author_profile_url,
      json.author_display_name,
      json.author_username,
      json.author_type,
      json.certified,

      json.likes || 0,
      json.comments || 0,
      json.shares || 0,
      json.rewards,
      json.visibility,
      json.publish_state,

      json.booster,

      json.permissions,
    );
  }
}
