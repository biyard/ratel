'use client';

import ContentEditor from './content-editor';
import Artworks from './artworks';
import { useDagitStore, Tab } from '../dagit-store';

export default function MainTab({ spaceId }: { spaceId: number }) {
  const activeTab = useDagitStore().activeTab;

  switch (activeTab) {
    case Tab.Content:
      return <ContentEditor spaceId={spaceId} />;
    case Tab.Artwork:
      return <Artworks spaceId={spaceId} />;
  }
}
