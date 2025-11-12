import { logger } from '@/lib/logger';
import { SpacePollPathProps } from '../space-poll-path-props';
import { useSpacePollAnalyzeController } from './space-poll-analyze-controller';
import { Col } from '@/components/ui/col';
import Report from '@/features/spaces/components/report';
import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';
import { SpaceType } from '@/features/spaces/types/space-type';

export function SpacePollAnalyzePage({ spacePk, pollPk }: SpacePollPathProps) {
  logger.debug(
    `SpacePollAnalyzeViewerPage: spacePk=${spacePk}, pollPk=${pollPk}`,
  );

  const ctrl = useSpacePollAnalyzeController(spacePk, pollPk);
  const { t } = useTranslation('SpacePollAnalyze');

  return (
    <Col>
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

      {ctrl.space.spaceType == SpaceType.Deliberation && (
        <div className="flex flex-row w-full justify-end">
          <Button className="w-fit" onClick={ctrl.handleBack}>
            {t('btn_back')}
          </Button>
        </div>
      )}
    </Col>
  );
}
