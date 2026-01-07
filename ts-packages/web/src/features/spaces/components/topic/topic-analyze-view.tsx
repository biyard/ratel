import React from 'react';
import { useUpsertAnalyzeMutation } from '../../polls/hooks/use-upsert-analyze-mutation';
import Card from '@/components/card';
import { Input } from '@/components/ui/input';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';
import { LdaTopicTable } from './lda-topic-table';
import { SpaceAnalyze } from '../../polls/types/space-analyze';

type TopicAnalyzeViewProps = {
  analyze?: SpaceAnalyze;
  handleUpdateTopics?: (topics: string[], keywords: string[][]) => void;
  handleUpsertAnalyze?: (ldaTopics: number) => void;
};

export function TopicAnalyzeView({
  analyze,
  handleUpdateTopics,
  handleUpsertAnalyze,
}: TopicAnalyzeViewProps) {
  const upsert = useUpsertAnalyzeMutation();
  const { t } = useTranslation('SpacePollAnalyze');

  const clamp = (v: number) => Math.min(20, Math.max(1, v));

  const [topicCount, setTopicCount] = React.useState<number>(5);

  const onConfirm = () => {
    const n = clamp(Number(topicCount) || 1);
    handleUpsertAnalyze(n);
  };

  return (
    <div className="flex flex-col gap-4">
      <Card key="topic-analyze-setting">
        <div className="flex flex-col gap-4">
          <div className="flex flex-col items-end gap-4">
            <div className="flex flex-col gap-2">
              <div className="text-sm font-medium text-text-primary">
                {t('number_of_topics')}
              </div>

              <div className="flex flex-row w-fit gap-2">
                <Input
                  type="number"
                  min={1}
                  max={20}
                  value={topicCount}
                  onChange={(e) => {
                    const raw = e.target.value;
                    if (raw === '') return;
                    const n = Number(raw);
                    if (!Number.isFinite(n)) return;
                    setTopicCount(clamp(n));
                  }}
                  onBlur={() => setTopicCount((v) => clamp(Number(v) || 1))}
                  className={cn(
                    'h-10 w-32 rounded-md border bg-background px-3 text-sm text-text-primary',
                    'focus:outline-none focus:ring-2 focus:ring-primary/30',
                  )}
                />

                <Button
                  variant="primary"
                  onClick={onConfirm}
                  disabled={upsert.isPending}
                >
                  {upsert.isPending ? t('analyzing') : t('confirm')}
                </Button>
              </div>

              <div className="text-xs text-muted-foreground">
                {t('input_hint')}
              </div>
            </div>
          </div>

          {upsert.isError && (
            <div className="text-sm text-destructive">{t('analyze_error')}</div>
          )}
        </div>
      </Card>

      <Card key="topic-analyze-table">
        <LdaTopicTable
          t={t}
          lda_topics={analyze?.lda_topics}
          handleUpdateTopics={handleUpdateTopics}
        />
      </Card>
    </div>
  );
}
