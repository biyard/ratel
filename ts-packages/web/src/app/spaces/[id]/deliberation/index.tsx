'use client';

import React from 'react';
import SpaceSideMenu from './_components/space_side_menu';
import ThreadPage from './_components/thread';
import DeliberationPage from './_components/deliberation';
import PollPage from './_components/poll';
import FinalConsensusPage from './_components/final_consensus';

import ClientProviders, {
  useDeliberationSpaceContext,
} from './provider.client';
import { DeliberationTab } from './types';
import AnalyzePage from './_components/analyze';

export default function DeliberationSpacePage() {
  return (
    <ClientProviders>
      <Page />
    </ClientProviders>
  );
}

function Page() {
  const { selectedType } = useDeliberationSpaceContext();

  return (
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
          {/* <SpaceSideMenu /> */}
        </div>
      </div>
    </div>
  );
}
