'use client';

import React, { useContext } from 'react';
import SpaceSideMenu from './_components/space-side-menu';
import ThreadPage from './_components/thread';
import DeliberationPage from './_components/deliberation';

import FinalConsensusPage from './_components/final-consensus';

import ClientProviders, {
  useDeliberationFeed,
  useDeliberationSpace,
  useDeliberationSpaceContext,
} from './provider.client';
import { DeliberationTab } from './types';
import { TeamContext } from '@/lib/contexts/team-context';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { useUserInfo } from '@/app/(social)/_hooks/user';
import SpaceHeader from '../_components/header';
import PollPage from '../_components/page/poll';
import AnalyzePage from '../_components/page/analyze';

export default function DeliberationSpacePage() {
  return (
    <ClientProviders>
      <Page />
    </ClientProviders>
  );
}

function Page() {
  const space = useDeliberationSpace();
  const feed = useDeliberationFeed(space.feed_id);
  const context = useDeliberationSpaceContext();
  const { selectedType } = useDeliberationSpaceContext();

  const { teams } = useContext(TeamContext);
  const authorId = space?.author[0].id;
  const selectedTeam = teams.some((t) => t.id === authorId);
  const { data: userInfo } = useUserInfo();

  const userId = userInfo ? userInfo.id : 0;

  if (
    space.status === SpaceStatus.Draft &&
    !space.author.some((a) => a.id === userId) &&
    !selectedTeam
  ) {
    return <div>No Authorized User</div>;
  }

  return (
    <div className="flex flex-col w-full gap-6.25">
      <div className="flex flex-row w-full">
        <SpaceHeader context={context} space={space} feed={feed} />
      </div>
      <div className="flex flex-row w-full h-full gap-5">
        <div className="flex-1 flex w-full">
          <div className="flex flex-row w-full gap-5">
            {selectedType == DeliberationTab.SUMMARY ? (
              <ThreadPage />
            ) : selectedType == DeliberationTab.DELIBERATION ? (
              <DeliberationPage />
            ) : selectedType == DeliberationTab.POLL ? (
              <PollPage context={context} />
            ) : selectedType == DeliberationTab.RECOMMANDATION ? (
              <FinalConsensusPage />
            ) : (
              <AnalyzePage context={context} />
            )}
            <SpaceSideMenu />
          </div>
        </div>
      </div>
    </div>
  );
}
