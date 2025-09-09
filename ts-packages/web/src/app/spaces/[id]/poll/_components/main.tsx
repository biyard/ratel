'use client';

import useSpaceById from '@/hooks/use-space-by-id';
import { Tab, usePollStore } from '../store';
import { PollSurveyPage } from './survey-tab';
import { PollAnalyzePage } from './analyze-tab';

export default function MainTab({ spaceId }: { spaceId: number }) {
  const activeTab = usePollStore().activeTab;
  const { data: space } = useSpaceById(spaceId);
  if (!space) {
    return null;
  }

  switch (activeTab) {
    case Tab.Poll:
      return <PollSurveyPage space={space} />;
    case Tab.Analyze:
      return <PollAnalyzePage space={space} />;
  }
}
