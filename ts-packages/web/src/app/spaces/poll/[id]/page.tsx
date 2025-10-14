import { useParams } from 'react-router';
import {
  PollSpaceController,
  Tab,
  usePollSpaceController,
} from './use-poll-space-controller';
import SpaceHeader from '@/features/spaces/components/header';
import SpaceHTMLContentEditor from '@/features/spaces/components/content-editor';
import Survey from '@/features/spaces/components/survey';
import Report, { ReportProps } from '@/features/spaces/components/report';
import usePollSpaceSummaries from '@/features/spaces/polls/hooks/use-poll-space-summary';
import { Button } from '@/components/ui/button';
import TabSelector from '@/features/spaces/components/side-menu/tab-selector';
import { Vote } from '@/assets/icons/email';
import TimelineMenu from '@/features/spaces/components/side-menu/timeline';
import { PieChart1 } from '@/assets/icons/graph';
import { SpaceStatus } from '@/types/space-common';

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

      <div className="flex flex-row w-full gap-5">
        <MainContent {...ctrl} />
        <SideMenu {...ctrl} />
      </div>
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
      return (
        <div className="flex flex-col w-full gap-4">
          <Survey {...ctrl} />
          {!ctrl.isEditMode && (
            <Button
              className="self-end max-w-40"
              variant="rounded_primary"
              disabled={!ctrl.isSurveyProgress || !ctrl.isAnswerModified}
              onClick={ctrl.onSubmitSurvey}
            >
              {ctrl.t('save_survey_button_label')}
            </Button>
          )}
        </div>
      );
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
  props: Omit<ReportProps, 'summaries'> & { spacePk: string },
) {
  const {
    data: { summaries },
  } = usePollSpaceSummaries(props.spacePk);
  console.log('summaries', summaries);
  return <Report {...props} summaries={summaries} />;
}

function SideMenu({ ...ctrl }: PollSpaceController) {
  const items = [
    {
      icon: (
        <Vote className="[&>path]:stroke-neutral-80 [&>rect]:stroke-neutral-80 w-5 h-5" />
      ),
      label: ctrl.t('tab_poll_label'),
      tab: Tab.Poll,
    },
  ];

  if (
    (ctrl.space.status === SpaceStatus.Finished &&
      ctrl.headerCtrl.hasEditPermission) ||
    true
  ) {
    items.push({
      icon: <PieChart1 className="[&>path]:stroke-neutral-80 w-5 h-5" />,
      label: ctrl.t('tab_analyze_label'),
      tab: Tab.Analyze,
    });
  }
  return (
    <div className="flex flex-col max-w-[250px] w-full gap-[10px]">
      <TabSelector<Tab>
        items={items}
        onClick={ctrl.onSelectTab}
        activeTab={ctrl.activeTab}
      />
      <TimelineMenu
        isEditing={false}
        handleSetting={() => {}}
        items={[
          {
            label: 'Created',
            time: ctrl.space.created_at,
          },
          {
            label: 'Start',
            time: ctrl.space.started_at,
          },
          {
            label: 'End',
            time: ctrl.space.ended_at,
          },
        ]}
        titleLabel={ctrl.t('timeline_title')}
      />
    </div>
  );
}
