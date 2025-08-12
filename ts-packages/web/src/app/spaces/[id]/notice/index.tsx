'use client';

import React, { useContext } from 'react';
import {
  useNoticeFeed,
  useNoticeSpace,
  useNoticeSpaceContext,
} from './provider.client';

import ClientProviders from './provider.client';
import SpaceHeader from './_components/space_header';
import SpaceSideMenu from './_components/space_side_menu';
import NoticePage from './_components/notice';
import { usePopup } from '@/lib/contexts/popup-service';
import PublishForm from './_components/modal/publish-form';
import { SpaceType, SpaceStatus } from '@/lib/api/models/spaces';
import { PublishingScope } from '@/lib/api/models/notice';
import { TeamContext } from '@/lib/contexts/team-context';
import { useUserInfo } from '@/app/(social)/_hooks/user';

export default function NoticeSpacePage() {
  return (
    <ClientProviders>
      <Page />
    </ClientProviders>
  );
}

function Page() {
  const space = useNoticeSpace();
  const feed = useNoticeFeed(space.feed_id);
  const popup = usePopup();
  const {
    isEdit,
    title,
    status,
    userType,
    proposerImage,
    proposerName,
    createdAt,
    isPrivatelyPublished,
    handleGoBack,
    handleSave,
    handleEdit,
    handleLike,
    handleShare,
    handlePublishWithScope,
    setTitle,
    // setSelectedType,
  } = useNoticeSpaceContext();

  const { teams } = useContext(TeamContext);
  const authorId = space?.author[0].id;
  const selectedTeam = teams.some((t) => t.id === authorId);
  const { data: userInfo } = useUserInfo();

  const userId = userInfo ? userInfo.id : 0;

  // Block access to draft notice spaces for unauthorized users
  if (
    space.status === SpaceStatus.Draft &&
    !space.author.some((a) => a.id === userId) &&
    !selectedTeam
  ) {
    return <div>No Authorized User</div>;
  }

  const handlePost = async () => {
    // For notice spaces, show the publish form modal
    if (space.space_type === SpaceType.Notice) {
      popup
        .open(
          <PublishForm
            currentScope={space.publishing_scope}
            onPublish={async (scope: PublishingScope) => {
              await handlePublishWithScope(scope);
              popup.close();
            }}
            onClose={() => popup.close()}
          />,
        )
        .withoutClose()
        .withoutBackdropClose();
    } else {
      // For other space types, use the regular posting
      await handlePublishWithScope(PublishingScope.Private);
    }
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
          authorId={authorId}
          rewards={feed.rewards}
          likes={feed.likes}
          shares={feed.shares}
          comments={feed.comments}
          isLiked={space?.is_liked}
          isPrivatelyPublished={isPrivatelyPublished}
          onback={handleGoBack}
          onsave={handleSave}
          onedit={handleEdit}
          onpost={handlePost}
          onlike={handleLike}
          onshare={handleShare}
          setTitle={setTitle}
        />
      </div>
      <div className="flex flex-row w-full h-full gap-5">
        <div className="flex-1 flex w-full">
          <div className="flex flex-row w-full gap-5">
            {/* For now, show our custom notice page */}
            <NoticePage />
            <SpaceSideMenu />
          </div>
        </div>
      </div>
    </div>
  );
}
