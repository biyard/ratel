import { useTranslation } from 'react-i18next';

export function ReportDraft() {
  const { t } = useTranslation('SpacePollAnalyze');

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
