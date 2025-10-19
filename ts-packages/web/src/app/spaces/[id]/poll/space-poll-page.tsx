import { useParams } from 'react-router';
import { PollSpaceController, Tab } from './space-poll-controller';
import Survey from '@/features/spaces/components/survey';
import Report, { ReportProps } from '@/features/spaces/components/report';
import usePollSpaceSummaries from '@/features/spaces/polls/hooks/use-poll-space-summary';
import { Button } from '@/components/ui/button';
import '@/features/spaces/polls/poll-side-menus';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpacePollEditorPage } from '@/features/spaces/polls/pages/creator/space-poll-editor-page';
import { SpacePollViewerPage } from '@/features/spaces/polls/pages/viewer/space-poll-viewer-page';
import { SpacePollCreatorPage } from '@/features/spaces/polls/pages/creator/space-poll-creator-page';

export default function SpacePollPage() {
  const { spacePk, pollPk } = useParams<{ spacePk: string; pollPk?: string }>();
  const { data: space } = useSpaceById(spacePk);
  /* const ctrl = useSpacePollController(spacePk); */

  if (!space) {
    throw new Error('Space not found');
  }

  if (pollPk && space.isAdmin()) {
    // Edit Mode
    return <SpacePollEditorPage spacePk={spacePk} pollPk={pollPk} />;
  }

  if (space.isAdmin()) {
    // Admin Mode
    return <SpacePollCreatorPage spacePk={spacePk} />;
  }

  return <SpacePollViewerPage spacePk={spacePk} />;
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
