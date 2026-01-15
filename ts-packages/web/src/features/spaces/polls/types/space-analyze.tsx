import { NetworkGraph } from './network-graph';
import { TfIdf } from './tf-idf';
import { TopicRow } from './topic-row';

export class SpaceAnalyze {
  public pk: string;
  public sk: string;

  public created_at?: number;

  public lda_topics?: TopicRow[];
  public network?: NetworkGraph;
  public tf_idf?: TfIdf[];
  public remove_topics?: string[];
  public lda_count?: number;
  public tf_idf_count?: number;
  public network_count?: number;

  public html_contents?: string;
  public metadata_url?: string;
  public analyze_finish?: boolean;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const lda_topics = Array.isArray(json?.lda_topics) ? json.lda_topics : [];
    const tf_idf = Array.isArray(json?.tf_idf) ? json.tf_idf : [];

    const network =
      json?.network && typeof json.network === 'object'
        ? json.network
        : json?.network_graph && typeof json.network_graph === 'object'
          ? json.network_graph
          : null;

    this.pk = String(json?.pk ?? '');
    this.sk = String(json?.sk ?? '');
    this.lda_topics = lda_topics;
    this.tf_idf = tf_idf;

    if (network) {
      const nodes = Array.isArray(network?.nodes) ? network.nodes : [];
      const edges = Array.isArray(network?.edges) ? network.edges : [];
      this.network = new NetworkGraph({ nodes, edges });
    } else {
      this.network = undefined;
    }

    this.html_contents = String(json?.html_contents ?? '');
    this.metadata_url = String(json?.metadata_url ?? '');
    this.analyze_finish = json?.analyze_finish ?? false;

    this.remove_topics = Array.isArray(json?.remove_topics)
      ? json?.remove_topics
      : [];

    this.lda_count =
      typeof json?.lda_count === 'number' ? json.lda_count : undefined;
    this.tf_idf_count =
      typeof json?.tf_idf_count === 'number' ? json.tf_idf_count : undefined;
    this.network_count =
      typeof json?.network_count === 'number' ? json.network_count : undefined;
  }
}
