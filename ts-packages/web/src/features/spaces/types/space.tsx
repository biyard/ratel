import { UserType } from '@/lib/api/ratel/users.v3';

import {
  SpacePublishState,
  SpaceStatus,
  SpaceVisibility,
} from './space-common';
import { SpaceType } from './space-type';
import { TeamGroupPermissions } from '@/features/auth/utils/team-group-permissions';
import { BoosterType } from './booster-type';
import FileModel from '../files/types/file';

export class Space {
  readonly permissions: TeamGroupPermissions;
  public pk: string;
  public sk: string;
  public title: string;
  public content: string;
  public createdAt: number;
  public updatedAt: number;
  public urls: string[];
  public spaceType: SpaceType;
  public features: string[];
  public status: SpaceStatus | null;
  public authorProfileUrl: string;
  public authorDisplayName: string;
  public authorUsername: string;
  public authorType: UserType;
  public certified: boolean;
  public likes: number = 0;
  public comments: number = 0;
  public shares: number = 0;
  public rewards: number | undefined;
  public visibility: SpaceVisibility;
  public publishState: SpacePublishState;
  public booster: BoosterType | undefined;
  public verified: boolean;
  public files: FileModel[] | undefined;
  public anonymous_participation: boolean;
  public change_visibility: boolean;
  public participated: boolean;
  public participantDisplayName: string | null;
  public participantProfileUrl: string | null;
  public participantUsername: string | null;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.permissions = new TeamGroupPermissions(json.permissions);
    this.pk = json.pk;
    this.sk = json.sk;
    this.title = json.title;
    this.content = json.content;
    this.createdAt = json.created_at;
    this.updatedAt = json.updated_at;
    this.urls = json.urls;
    this.spaceType = json.space_type;
    this.features = json.features;
    this.status = json.status;
    this.authorProfileUrl = json.author_profile_url;
    this.authorDisplayName = json.author_display_name;
    this.authorUsername = json.author_username;
    this.authorType = json.author_type;
    this.certified = json.certified;
    this.likes = json.likes;
    this.comments = json.comments;
    this.shares = json.shares;
    this.rewards = json.rewards;
    this.visibility = json.visibility;
    this.publishState = json.publish_state;
    this.booster = json.booster;
    this.verified = json.verified;
    this.files = json.files;
    this.anonymous_participation = json.anonymous_participation;
    this.change_visibility = json.change_visibility;
    this.participated = json.participated;
    this.participantDisplayName = json.participant_display_name || null;
    this.participantProfileUrl = json.participant_profile_url || null;
    this.participantUsername = json.participant_username || null;
  }

  shouldParticipateManually() {
    return this.anonymous_participation;
  }

  isAdmin() {
    return this.permissions.isAdmin();
  }

  get isDraft() {
    return this.publishState === SpacePublishState.Draft;
  }

  get isPublic() {
    return this.visibility.type === 'Public';
  }

  get isStarted() {
    return this.status === SpaceStatus.Started;
  }

  get isInProgress() {
    return this.status === SpaceStatus.InProgress;
  }

  get isFinished() {
    return this.status === SpaceStatus.Finished;
  }
}
