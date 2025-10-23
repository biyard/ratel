import { call } from './call';

export function createSpaceDiscussion(
  spacePk: string,
  startedAt: number,
  endedAt: number,

  name: string,
  description: string,
  userIds: string[],
): Promise<void> {
  return call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/discussions`, {
    started_at: startedAt,
    ended_at: endedAt,

    name,
    description,
    user_ids: userIds,
  });
}

export function updateSpaceDiscussion(
  spacePk: string,
  discussionPk: string,
  startedAt: number,
  endedAt: number,

  name: string,
  description: string,
  userIds: string[],
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}`,
    {
      started_at: startedAt,
      ended_at: endedAt,

      name,
      description,
      user_ids: userIds,
    },
  );
}

export function deleteSpaceDiscussion(
  spacePk: string,
  discussionPk: string,
): Promise<void> {
  return call(
    'DELETE',
    `/v3/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}`,
  );
}

export function discussionStartMeeting(
  spacePk: string,
  discussionPk: string,
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/meetings/start-meeting`,
  );
}

export function discussionParticipateMeeting(
  spacePk: string,
  discussionPk: string,
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/meetings/participant-meeting`,
  );
}

export function discussionExitMeeting(
  spacePk: string,
  discussionPk: string,
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/meetings/exit-meeting`,
  );
}

export function discussionStartRecording(
  spacePk: string,
  discussionPk: string,
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/meetings/start-recording`,
  );
}

export function discussionEndRecording(
  spacePk: string,
  discussionPk: string,
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/discussions/${encodeURIComponent(discussionPk)}/meetings/end-recording`,
  );
}
