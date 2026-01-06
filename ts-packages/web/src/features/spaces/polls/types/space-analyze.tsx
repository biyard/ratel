import { NetworkCentralityRow } from './network-centrality-row';
import { TfIdf } from './tf-idf';
import { TopicRow } from './topic-row';

export class SpaceAnalyze {
  public pk: string;
  public sk: string;

  public lda_topics?: TopicRow[];
  public network_centrality?: NetworkCentralityRow[];
  public tf_idf?: TfIdf[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const lda_topics = Array.isArray(json.lda_topics) ? json.lda_topics : [];
    const network_centrality = Array.isArray(json.network_centrality)
      ? json.network_centrality
      : [];
    const tf_idf = Array.isArray(json.tf_idf) ? json.tf_idf : [];

    this.pk = json.pk;
    this.sk = json.sk;
    this.lda_topics = lda_topics;
    this.network_centrality = network_centrality;
    this.tf_idf = tf_idf;
  }
}
