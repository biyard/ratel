import { useTranslation } from 'react-i18next';
import { SpaceAnalyze } from '../../types/space-analyze';

export function ReportDraft({ analyze }: { analyze: SpaceAnalyze }) {
  const { t } = useTranslation('SpacePollAnalyze');
  console.log('analyze: ', analyze);

  return (
    <div className="w-full rounded-lg border border-border bg-card p-6">
      <div className="text-base font-semibold text-foreground">
        {t('report_write')}
      </div>
      <p className="mt-2 text-sm text-muted-foreground">
        {t('report_draft_placeholder')}
      </p>
    </div>
  );
}
