'use client';

import { useDeliberationSpaceById, usePostByIdV2 } from '@/lib/api/ratel_api';
import { useDeliberationSpaceByIdContext } from '../providers.client';
import SpaceHeader from './header';
import SpaceSideMenu, { SpaceTabsMobile } from './space_side_menu';
import ThreadPage from './tab/thread';
import { DeliberationTab } from '../types';
import DeliberationPage from './tab/deliberation';
import { DeliberationSurveyPage } from './tab/poll';
import FinalConsensusPage from './tab/recommendation';
import DeliberationAnalyzePage from './tab/analyze';

export default function DeliberationSpacePage() {
  const { spaceId, selectedType } = useDeliberationSpaceByIdContext();
  const space = useDeliberationSpaceById(spaceId);

  const postPk = space.data.post_pk;
  const id = decodeURIComponent(postPk).replace(/^.*#/, '');

  const feed = usePostByIdV2(id);

  return (
    <div className="flex flex-col w-full gap-6.25">
      <div className="flex flex-row w-full">
        <SpaceHeader space={space.data} feed={feed.data} />
      </div>
      <div className="hidden max-tablet:block w-full">
        <SpaceTabsMobile space={space.data} />
      </div>
      <div className="flex flex-row w-full h-full gap-5">
        <div className="flex-1 flex w-full">
          <div className="flex flex-row w-full gap-5">
            {selectedType == DeliberationTab.SUMMARY ? (
              <ThreadPage />
            ) : selectedType == DeliberationTab.DELIBERATION ? (
              <DeliberationPage />
            ) : selectedType == DeliberationTab.POLL ? (
              <DeliberationSurveyPage space={space.data} />
            ) : selectedType == DeliberationTab.RECOMMANDATION ? (
              <FinalConsensusPage />
            ) : (
              <DeliberationAnalyzePage />
            )}

            <SpaceSideMenu space={space.data} />
          </div>
        </div>
      </div>
    </div>
  );
}
