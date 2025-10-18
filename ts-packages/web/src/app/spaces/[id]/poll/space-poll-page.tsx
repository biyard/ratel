import { useParams } from 'react-router';
import {
  PollSpaceController,
  Tab,
  useSpacePollController,
} from './space-poll-controller';
import Survey from '@/features/spaces/components/survey';
import Report, { ReportProps } from '@/features/spaces/components/report';
import usePollSpaceSummaries from '@/features/spaces/polls/hooks/use-poll-space-summary';
import { Button } from '@/components/ui/button';
import TabSelector from '@/features/spaces/components/side-menu/tab-selector';
import { Vote } from '@/assets/icons/email';
import TimelineMenu from '@/features/spaces/components/side-menu/timeline';
import { PieChart1 } from '@/assets/icons/graph';
import { SpaceStatus } from '@/features/spaces/types/space-common';
import { menusForSpaceType } from '../use-space-home-controller';

menusForSpaceType;

export default function SpacePollPage() {
  const { spacePk } = useParams<{ spacePk: string }>();
  const ctrl = useSpacePollController(spacePk);
  return <MainContent {...ctrl} />;
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
        <div className="flex flex-col gap-4 w-full">
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
    ctrl.space.status === SpaceStatus.Finished &&
    ctrl.headerCtrl.hasEditPermission
  ) {
    items.push({
      icon: <PieChart1 className="[&>path]:stroke-neutral-80 w-5 h-5" />,
      label: ctrl.t('tab_analyze_label'),
      tab: Tab.Analyze,
    });
  }
  return (
    <div className="flex flex-col w-full max-w-[250px] gap-[10px]">
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
