import { logger } from '@/lib/logger';
import { SpacePollPathProps } from '../space-poll-path-props';
import { useSpacePollAnalyzeController } from './space-poll-analyze-controller';
import { Col } from '@/components/ui/col';
import Report from '@/features/spaces/components/report';
import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';

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
        summaries={ctrl.summary.summaries}
        handleDownloadExcel={ctrl.handleDownloadExcel}
      />

      <div className="flex flex-row w-full justify-end">
        <Button className="w-fit" onClick={ctrl.handleBack}>
          {t('btn_back')}
        </Button>
      </div>
    </Col>
  );
}
