import { useParams } from 'react-router';
import {
  PollSpaceController,
  Tab,
  usePollSpaceController,
} from './use-poll-space-controller';
import SpaceHeader from '@/features/spaces/components/header';
import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import Survey from '@/features/spaces/components/survey';
import { Analyze, AnalyzeProps } from '@/features/spaces/components/analyze';
import usePollSpaceSummaries from '@/features/poll-space/hooks/use-poll-space-summary';

export default function PollSpacePage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = usePollSpaceController(spacePk);

  return (
    <div className="flex flex-col w-full gap-6">
      <SpaceHeader {...ctrl.headerCtrl} />
      <SpaceHTMLContentEditor
        htmlContent={ctrl.headerCtrl.html_content}
        isEditMode={ctrl.isEditMode}
        onContentChange={ctrl.headerCtrl.updateContent}
      />
      <MainContent {...ctrl} />
    </div>
  );
}

function MainContent({
  activeTab,
  ...ctrl
}: {
  activeTab: Tab;
} & PollSpaceController) {
  switch (activeTab) {
    case Tab.Poll:
      return <Survey {...ctrl} />;
    case Tab.Analyze:
      return (
        <AnalyzeTab
          startedAt={ctrl.space.started_at}
          endedAt={ctrl.space.ended_at}
          totalResponses={ctrl.space.user_response_count}
          {...ctrl}
        />
      );
  }
}

function AnalyzeTab(
  props: Omit<AnalyzeProps, 'summaries'> & { spacePk: string },
) {
  const {
    data: { summaries },
  } = usePollSpaceSummaries(props.spacePk);
  return <Analyze {...props} summaries={summaries} />;
}
