import React from 'react';
import Card from '@/components/card';
import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';
import { LdaTopicTable } from './lda-topic-table';
import { SpaceAnalyze } from '../../polls/types/space-analyze';
import { TfIdfChart } from './tf-idf-chart';
import { NetworkChart } from './network-chart';
import { AnalyzeNumberField } from './analyze_number_field';

type TopicAnalyzeViewProps = {
  analyze?: SpaceAnalyze;
  analyzeFinish?: boolean;
  handleUpdateLda?: (topics: string[], keywords: string[][]) => void;
  handleUpsertAnalyze?: (
    ldaTopics: number,
    tfIdfKeywords: number,
    networkTopNodes: number,
  ) => void | Promise<{ spacePk: string }>;
};

export function TopicAnalyzeView({
  analyze,
  analyzeFinish = true,
  handleUpdateLda,
  handleUpsertAnalyze,
}: TopicAnalyzeViewProps) {
  const { t } = useTranslation('SpacePollAnalyze');

  const clamp = (v: number) => Math.min(20, Math.max(1, v));
  const clampTfIdf = (v: number) => Math.min(50, Math.max(1, v));
  const clampNetwork = (v: number) => Math.min(200, Math.max(1, v));

  const [topicCount, setTopicCount] = React.useState<number>(5);
  const [tfIdfCount, setTfIdfCount] = React.useState<number>(10);
  const [networkTopNodes, setNetworkTopNodes] = React.useState<number>(30);

  const [isSubmitting, setIsSubmitting] = React.useState(false);
  const [submitError, setSubmitError] = React.useState(false);

  const hasLda =
    Array.isArray(analyze?.lda_topics) && analyze.lda_topics.length > 0;
  const hasNetwork =
    analyze?.network != null &&
    Array.isArray(analyze?.network?.nodes) &&
    analyze.network.nodes.length > 0;
  const hasTfIdf = Array.isArray(analyze?.tf_idf) && analyze.tf_idf.length > 0;

  React.useEffect(() => {
    const list = Array.isArray(analyze?.lda_topics) ? analyze.lda_topics : [];

    const uniqTopics = new Set<string>();
    for (const r of list) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const topic = String((r as any)?.topic ?? '').trim();
      if (topic) uniqTopics.add(topic);
    }

    const n = uniqTopics.size;
    setTopicCount(clamp(n > 0 ? n : 5));
  }, [analyze?.lda_topics]);

  React.useEffect(() => {
    const n = Array.isArray(analyze?.tf_idf) ? analyze.tf_idf.length : 0;
    setTfIdfCount(clampTfIdf(n > 0 ? n : 10));
  }, [analyze?.tf_idf]);

  React.useEffect(() => {
    const n = Array.isArray(analyze?.network?.nodes)
      ? analyze!.network.nodes.length
      : 0;
    setNetworkTopNodes(clampNetwork(n > 0 ? n : 30));
  }, [analyze?.network?.nodes]);

  const onConfirm = async () => {
    const n = clamp(Number(topicCount) || 1);
    const m = clampTfIdf(Number(tfIdfCount) || 10);
    const k = clampNetwork(Number(networkTopNodes) || 30);

    try {
      setSubmitError(false);
      setIsSubmitting(true);
      await Promise.resolve(handleUpsertAnalyze?.(n, m, k));
    } catch {
      setSubmitError(true);
    } finally {
      setIsSubmitting(false);
    }
  };

  const pending = isSubmitting;
  const allowRequest = analyzeFinish !== false;

  return (
    <div className="flex flex-col gap-4">
      <Card key="topic-analyze-setting">
        <div className="flex flex-col w-full gap-4">
          <div className="flex flex-col w-full gap-4">
            <AnalyzeNumberField
              label={t('number_of_topics')}
              hint={t('input_hint')}
              value={topicCount}
              onValueChange={setTopicCount}
              min={1}
              max={20}
              clamp={clamp}
              fallbackOnBlur={1}
              disabled={pending || !allowRequest}
            />

            <AnalyzeNumberField
              label={t('number_of_tfidf_keywords')}
              hint={t('tfidf_input_hint')}
              value={tfIdfCount}
              onValueChange={setTfIdfCount}
              min={1}
              max={50}
              clamp={clampTfIdf}
              fallbackOnBlur={10}
              disabled={pending || !allowRequest}
            />

            <AnalyzeNumberField
              label={t('number_of_network_top_nodes')}
              hint={t('network_top_nodes_input_hint')}
              value={networkTopNodes}
              onValueChange={setNetworkTopNodes}
              min={1}
              max={200}
              clamp={clampNetwork}
              fallbackOnBlur={30}
              disabled={pending || !allowRequest}
            />
          </div>

          {submitError && (
            <div className="text-sm text-destructive">{t('analyze_error')}</div>
          )}

          {allowRequest && (
            <div className="flex flex-row w-full justify-end items-end">
              <Button variant="primary" onClick={onConfirm} disabled={pending}>
                {pending ? t('analyzing') : t('confirm')}
              </Button>
            </div>
          )}
        </div>
      </Card>

      {hasLda && (
        <Card key="topic-analyze-table">
          <LdaTopicTable
            t={t}
            ldaTopics={analyze?.lda_topics}
            handleUpdateLda={handleUpdateLda}
          />
        </Card>
      )}

      {hasNetwork && (
        <Card key="network-chart">
          <NetworkChart t={t} network={analyze?.network} />
        </Card>
      )}

      {hasTfIdf && (
        <Card key="tf-idf-chart">
          <TfIdfChart
            t={t}
            isHtml={true}
            tf_idf={analyze?.tf_idf}
            limit={analyze?.tf_idf.length}
          />
        </Card>
      )}
    </div>
  );
}
