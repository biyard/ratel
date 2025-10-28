export interface MediaPlacementInfo {
  audio_host_url: string;
  audio_fallback_url: string;
  screen_data_url: string;
  screen_sharing_url: string;
  screen_viewing_url: string;
  signaling_url: string;
  turn_control_url: string;
}

export interface MeetingInfo {
  meeting_id: string;
  media_placement: MediaPlacementInfo;
  media_region: string;
}

export interface AttendeeInfo {
  attendee_id: string;
  join_token: string;
  external_user_id: string;
}

export interface MeetingData {
  meeting: MeetingInfo;
  attemdee: AttendeeInfo;
  participants: Participant[];
  record?: string;
}

export interface Participant {
  id: number;
  nickname?: string;
  profile_url?: string;
  username: string;
}
