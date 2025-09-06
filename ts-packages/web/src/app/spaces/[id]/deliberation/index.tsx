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
import SpaceHeader from '../_components/header/index';
import { SpaceProvider } from '../_components/header/provider';
import DeliberationAnalyzePage from './_components/analyze-tab';
import { DeliberationSurveyPage } from './_components/survey-tab';
import { useTranslations } from 'next-intl';

export default function DeliberationSpacePage() {
  return (
    <ClientProviders>
      <Page />
    </ClientProviders>
  );
}

function Page() {
  const t = useTranslations('Space');
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
    return <div>{t('no_authorized_user')}</div>;
  }

  return (
    <div className="flex flex-col w-full gap-6.25">
      <div className="flex flex-row w-full">
        <SpaceProvider value={context}>
          <SpaceHeader space={space} feed={feed} />
        </SpaceProvider>
      </div>
      <div className="flex flex-row w-full h-full gap-5">
        <div className="flex-1 flex w-full">
          <div className="flex flex-row w-full gap-5">
            {selectedType == DeliberationTab.SUMMARY ? (
              <ThreadPage />
            ) : selectedType == DeliberationTab.DELIBERATION ? (
              <DeliberationPage />
            ) : selectedType == DeliberationTab.POLL ? (
              <DeliberationSurveyPage space={space} />
            ) : selectedType == DeliberationTab.RECOMMANDATION ? (
              <FinalConsensusPage />
            ) : (
              <DeliberationAnalyzePage />
            )}
            <SpaceSideMenu />
          </div>
        </div>
      </div>
    </div>
  );
}
