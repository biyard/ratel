'use client';

import React from 'react';
import SpaceSideMenu from './_components/space_side_menu';
import ThreadPage from './_components/thread';
import DeliberationPage from './_components/deliberation';
import PollPage from './_components/poll';
import FinalConsensusPage from './_components/final_consensus';

import ClientProviders, {
  useDeliberationSpace,
  useDeliberationSpaceContext,
} from './provider.client';
import { DeliberationTab } from './types';
import AnalyzePage from './_components/analyze';
import SpaceHeader from './_components/space_header';
import { usePopup } from '@/lib/contexts/popup-service';
import GoPublicPopup from './_components/modal/go_public';

export default function DeliberationSpacePage() {
  return (
    <ClientProviders>
      <Page />
    </ClientProviders>
  );
}

function Page() {
  const popup = usePopup();
  const space = useDeliberationSpace();
  const {
    selectedType,
    isEdit,
    title,
    status,
    userType,
    proposerImage,
    proposerName,
    createdAt,
    handleGoBack,
    handleSave,
    handleEdit,
    handlePostingSpace,
    setTitle,
  } = useDeliberationSpaceContext();

  const handlePost = () => {
    popup
      .open(
        <GoPublicPopup
          onclose={() => {
            popup.close();
          }}
          onpublic={async () => {
            await handlePostingSpace();
            popup.close();
          }}
        />,
      )
      .withoutBackdropClose();
  };

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
