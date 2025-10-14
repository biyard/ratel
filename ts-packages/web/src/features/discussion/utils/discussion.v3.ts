import { AttendeeInfo, MeetingInfo } from '@/lib/api/models/meeting';
import { call } from '@/lib/api/ratel/call';

export type PartitionString = string;

export function getDiscussionById(
  spacePk: string,
  discussionPk: string,
): Promise<DeliberationDiscussionResponse> {
  return call(
    'GET',
    `/v3/spaces/deliberation/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}`,
  );
}

export function getMeetingByDiscussionId(
  spacePk: string,
  discussionPk: string,
): Promise<MeetingData> {
  return call(
    'GET',
    `/v3/spaces/deliberation/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/meeting`,
  );
}

export function discussionStartMeeting(
  spacePk: string,
  discussionPk: string,
): Promise<DeliberationDiscussionResponse> {
  return call(
    'POST',
    `/v3/spaces/deliberation/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/start-meeting`,
    {},
  );
}

export function discussionParticipantMeeting(
  spacePk: string,
  discussionPk: string,
): Promise<DeliberationDiscussionResponse> {
  return call(
    'POST',
    `/v3/spaces/deliberation/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/participant-meeting`,
    {},
  );
}

export function discussionExitMeeting(
  spacePk: string,
  discussionPk: string,
): Promise<DeliberationDiscussionResponse> {
  return call(
    'POST',
    `/v3/spaces/deliberation/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/exit-meeting`,
    {},
  );
}

export interface MeetingData {
  Meeting: MeetingInfo;
  Attendee: AttendeeInfo;
  Participants: DiscussionUser[];
}

export interface DiscussionUser {
  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
}

export interface DeliberationDiscussionResponse {
  pk: string;

  started_at: number;
  ended_at: number;

  name: string;
  description: string;
  meeting_id: string | undefined | null;
  pipeline_id: string;

  media_pipeline_arn: string | undefined | null;
  record: string | undefined | null;

  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;

  members: DiscussionMemberResponse[];
  participants: DiscussionParticipantResponse[];
}

export interface DiscussionMemberResponse {
  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
}

export interface DiscussionParticipantResponse {
  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
  participant_id: string;
}
