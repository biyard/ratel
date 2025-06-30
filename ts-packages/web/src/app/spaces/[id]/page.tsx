import { SpaceType } from '@/lib/api/models/spaces';
import { getSpaceById } from '@/lib/api/ratel_api.server';
import { logger } from '@/lib/logger';
import React from 'react';
import DeliberationSpacePage from './deliberation';
import CommitteeSpacePage from './committee/page.client';

export default async function Page({
  params,
}: {
  params: Promise<{ id: number }>;
}) {
  const { id } = await params;

  const space = await getSpaceById(id);

  if (space.data?.space_type === SpaceType.Deliberation) {
    return <DeliberationSpacePage />;
  } else if (space.data?.space_type === SpaceType.Committee) {
    return <CommitteeSpacePage />;
  }

  logger.debug('Unknown space type:', space.data?.space_type);
  return <div>Unsupported space type</div>;
}
