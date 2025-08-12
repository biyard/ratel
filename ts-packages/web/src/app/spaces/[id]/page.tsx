import { SpaceType } from '@/lib/api/models/spaces';
import { getSpaceById } from '@/lib/api/ratel_api.server';
import { logger } from '@/lib/logger';
import React from 'react';
import DeliberationSpacePage from './deliberation';
import NoticeSpacePage from './notice';
import CommitteeSpacePage from './committee/page.client';
import SprintLeaguePage from './sprint-league';
import PollSpacePage from './poll';

export default async function Page({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  const spaceId = Number(id);

  const space = await getSpaceById(spaceId);
  if (space.data?.space_type === SpaceType.Deliberation) {
    return <DeliberationSpacePage />;
  } else if (space.data?.space_type === SpaceType.Committee) {
    return <CommitteeSpacePage />;
  } else if (space.data?.space_type === SpaceType.SprintLeague) {
    return <SprintLeaguePage space_id={spaceId} />;
  } else if (space.data?.space_type === SpaceType.Notice) {
    return <NoticeSpacePage />;
  } else if (space.data?.space_type === SpaceType.Poll) {
    return <PollSpacePage />;
  }

  logger.debug('Unknown space type:', space.data?.space_type);
  return <div>Unsupported space type</div>;
}
