import { NetworkCentralityRow } from './network-centrality-row';
import { NetworkEdgeRow } from './network-edge-row';

export class NetworkGraph {
  public nodes: NetworkCentralityRow[];
  public edges: NetworkEdgeRow[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const nodes = Array.isArray(json?.nodes) ? json.nodes : [];
    const edges = Array.isArray(json?.edges) ? json.edges : [];
    this.nodes = nodes;
    this.edges = edges;
  }
}
