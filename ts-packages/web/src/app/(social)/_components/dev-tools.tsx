'use client';
import { config, Env } from '@/config';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { Feed, FeedType, FileInfo } from '@/lib/api/models/feeds';
import { createDraftRequest } from '@/lib/api/models/feeds/create-draft';
import {
  updateDraftRequest,
  UrlType,
} from '@/lib/api/models/feeds/update-draft-request';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import React, { Fragment } from 'react';

export default function DevTools() {
  const { data: user } = useSuspenseUserInfo();
  const { post } = useApiCall();

  if (config.env !== Env.Dev) {
    return <Fragment />;
  }

  const handleSimulate = async () => {
    const title =
      'DAO Treasury Transparency Act & Crypto Investor Protection Act';
    const html_contents =
      'Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings...';
    const user_id = user.id;
    const industry_id = 1;
    const quote_feed_id = null;
    const files: FileInfo[] = [];
    const url =
      'https://metadata.ratel.foundation/metadata/0faf45ec-35e1-40e9-bff2-c61bb52c7d19';

    const res: Feed = await post(
      ratelApi.feeds.createDraft(),
      createDraftRequest(FeedType.Post, user_id),
    );

    await post(
      ratelApi.feeds.updateDraft(res.id),
      updateDraftRequest(
        html_contents,
        industry_id,
        title,
        quote_feed_id,
        files,
        url,
        UrlType.Image,
      ),
    );

    await post(ratelApi.feeds.publishDraft(res.id), { publish: {} });
  };

  return (
    <div className="px-2 py-5 px-3 w-full rounded-[10px] bg-component-bg">
      <button className="sidemenu-link" onClick={handleSimulate}>
        <span>Simulate a post</span>
      </button>
    </div>
  );
}
