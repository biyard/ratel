'use client';

import React, { useContext } from 'react';
import SpaceSideMenu from './_components/space_side_menu';
import ThreadPage from './_components/thread';
import DeliberationPage from './_components/deliberation';
import PollPage from './_components/poll';
import FinalConsensusPage from './_components/final-consensus';

import ClientProviders, {
  useDeliberationSpace,
  useDeliberationSpaceContext,
} from './provider.client';
import { DeliberationTab } from './types';
import AnalyzePage from './_components/analyze';
import SpaceHeader from './_components/space-header';
import { TeamContext } from '@/lib/contexts/team-context';
import { SpaceStatus } from '@/lib/api/models/spaces';
import { useUserInfo } from '@/app/(social)/_hooks/user';

export default function DeliberationSpacePage() {
  return (
    <ClientProviders>
      <Page />
    </ClientProviders>
  );
}

function Page() {
  const space = useDeliberationSpace();
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
        <SpaceHeader />
      </div>
      <div className="flex flex-row w-full h-full gap-5">
        <div className="flex-1 flex w-full">
          <div className="flex flex-row w-full gap-5">
            {selectedType == DeliberationTab.SUMMARY ? (
              <ThreadPage />
            ) : selectedType == DeliberationTab.DELIBERATION ? (
              <DeliberationPage />
            ) : selectedType == DeliberationTab.POLL ? (
              <PollPage />
            ) : selectedType == DeliberationTab.RECOMMANDATION ? (
              <FinalConsensusPage />
            ) : (
              <AnalyzePage />
            )}
            <SpaceSideMenu />
          </div>
        </div>
      </div>
    </div>
  );
}
