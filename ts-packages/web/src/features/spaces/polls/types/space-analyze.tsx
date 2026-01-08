import { NetworkGraph } from './network-graph';
import { TfIdf } from './tf-idf';
import { TopicRow } from './topic-row';

export class SpaceAnalyze {
  public pk: string;
  public sk: string;

  public lda_topics?: TopicRow[];
  public lda_html_contents?: string;

  public network?: NetworkGraph;
  public network_html_contents?: string;

  public tf_idf?: TfIdf[];
  public tf_idf_html_contents?: string;

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
    this.lda_html_contents = String(json?.lda_html_contents ?? '');
    this.network_html_contents = String(json?.network_html_contents ?? '');
    this.tf_idf_html_contents = String(json?.tf_idf_html_contents ?? '');
    this.tf_idf = tf_idf;

    if (network) {
      const nodes = Array.isArray(network?.nodes) ? network.nodes : [];
      const edges = Array.isArray(network?.edges) ? network.edges : [];
      this.network = new NetworkGraph({ nodes, edges });
    } else {
      this.network = undefined;
    }
  }
}
