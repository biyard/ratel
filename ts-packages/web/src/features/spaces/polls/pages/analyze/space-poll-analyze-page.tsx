import * as React from 'react';
import { logger } from '@/lib/logger';
import { SpacePollPathProps } from '../space-poll-path-props';
import { useSpacePollAnalyzeController } from './space-poll-analyze-controller';
import { Col } from '@/components/ui/col';
import Report from '@/features/spaces/components/report';
import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';
import { SpaceType } from '@/features/spaces/types/space-type';
import { cn } from '@/lib/utils';
import { TopicAnalyzeView } from '@/features/spaces/components/topic/topic-analyze-view';

type TabKey = 'response' | 'topic';

export function SpacePollAnalyzePage({ spacePk, pollPk }: SpacePollPathProps) {
  logger.debug(
    `SpacePollAnalyzeViewerPage: spacePk=${spacePk}, pollPk=${pollPk}`,
  );

  const ctrl = useSpacePollAnalyzeController(spacePk, pollPk);
  const { t } = useTranslation('SpacePollAnalyze');

  const showTabs =
    ctrl.space.spaceType === SpaceType.Deliberation && ctrl.space.isFinished;

  const [tab, setTab] = React.useState<TabKey>('response');

  React.useEffect(() => {
    if (!showTabs) setTab('response');
  }, [showTabs]);

  return (
    <Col className="gap-4">
      {showTabs && (
        <div className="w-fit">
          <div className="flex items-center gap-6">
            <button
              type="button"
              onClick={() => setTab('response')}
              className={cn(
                'relative px-1 py-3 text-sm transition-colors',
                tab === 'response'
                  ? 'text-foreground'
                  : 'text-muted-foreground hover:text-foreground',
                tab === 'response' &&
                  "after:absolute after:left-0 after:bottom-0 after:h-0.5 after:w-full after:bg-primary after:content-['']",
              )}
            >
              {t('response_analyze')}
            </button>

            <button
              type="button"
              onClick={() => setTab('topic')}
              className={cn(
                'relative px-1 py-3 text-sm transition-colors',
                tab === 'topic'
                  ? 'text-foreground'
                  : 'text-muted-foreground hover:text-foreground',
                tab === 'topic' &&
                  "after:absolute after:left-0 after:bottom-0 after:h-0.5 after:w-full after:bg-primary after:content-['']",
              )}
            >
              {t('topic_analyze')}
            </button>
          </div>
        </div>
      )}

      {(!showTabs || tab === 'response') && (
        <Report
          startedAt={ctrl.poll.started_at}
          endedAt={ctrl.poll.ended_at}
          totalResponses={ctrl.poll.user_response_count}
          questions={ctrl.poll.questions}
          editable={ctrl.poll.response_editable}
          summaries={ctrl.summary.summaries}
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          summariesByGender={ctrl.summary.summaries_by_gender as any}
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          summariesByAge={ctrl.summary.summaries_by_age as any}
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          summariesBySchool={ctrl.summary.summaries_by_school as any}
          handleDownloadExcel={ctrl.handleDownloadExcel}
        />
      )}

      {showTabs && tab === 'topic' && (
        <TopicAnalyzeView
          analyze={ctrl.analyze}
          analyzeFinish={!!ctrl.analyze.analyze_finish}
          handleUpdateLda={ctrl.handleUpdateLda}
          handleUpsertAnalyze={ctrl.handleUpsertAnalyze}
        />
      )}

      {ctrl.space.spaceType === SpaceType.Deliberation && (
        <div className="flex w-full flex-row justify-end">
          <Button className="w-fit" onClick={ctrl.handleBack}>
            {t('btn_back')}
          </Button>
        </div>
      )}
    </Col>
  );
}
