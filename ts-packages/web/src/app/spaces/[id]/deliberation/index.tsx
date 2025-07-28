'use client';
import React, { useContext } from 'react';
import SpaceSideMenu from './_components/space_side_menu';
import ThreadPage from './_components/thread';
import DeliberationPage from './_components/deliberation';
import PollPage from './_components/poll';
import FinalConsensusPage from './_components/final_consensus';
import ClientProviders, {
  useDeliberationFeed,
  useDeliberationSpace,
  useDeliberationSpaceContext,
} from './provider.client';
import { DeliberationTab } from './types';
import AnalyzePage from './_components/analyze';
import SpaceHeader from './_components/space_header';
import { usePopup } from '@/lib/contexts/popup-service';
import GoPublicPopup from './_components/modal/go_public';
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
  const popup = usePopup();
  const space = useDeliberationSpace();
  const feed = useDeliberationFeed(space.feed_id);
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
    handleLike,
    handleShare,
    handlePostingSpace,
    handleDelete,
    setTitle,
  } = useDeliberationSpaceContext();

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
    <div className="flex flex-col w-full gap-6.25">
      <div className="flex flex-row w-full">
        <SpaceHeader
          isEdit={isEdit}
          title={title}
          status={status}
          userType={userType}
          proposerImage={proposerImage}
          proposerName={proposerName}
          createdAt={createdAt}
          authorId={space?.author[0].id}
          rewards={feed.rewards}
          likes={feed.likes}
          shares={feed.shares}
          comments={feed.comments}
          isLiked={space?.is_liked}
          onback={handleGoBack}
          onsave={handleSave}
          onedit={handleEdit}
          onpost={handlePost}
          onlike={handleLike}
          onshare={handleShare}
          setTitle={setTitle}
          ondelete={handleDelete}
        />
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
