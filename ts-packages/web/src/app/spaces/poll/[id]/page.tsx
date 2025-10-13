import { useParams } from 'react-router';
import {
  PollSpaceController,
  Tab,
  usePollSpaceController,
} from './use-poll-space-controller';
import SpaceHeader from '@/features/spaces/components/header';
import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import SpaceSurvey from '@/features/spaces/components/survey';

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
      return <SpaceSurvey {...ctrl} />;
    case Tab.Analyze:
      return <div></div>;
    // return <PollAnalyzePage space={space} />;
  }
}
