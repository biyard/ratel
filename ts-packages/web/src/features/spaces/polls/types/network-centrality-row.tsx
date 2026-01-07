export class NetworkCentralityRow {
  public node: string;
  public degree_centrality: number;
  public betweenness_centrality: number;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.node = String(json?.node ?? '');
    this.degree_centrality = Number(json?.degree_centrality ?? 0);
    this.betweenness_centrality = Number(json?.betweenness_centrality ?? 0);
  }
}
