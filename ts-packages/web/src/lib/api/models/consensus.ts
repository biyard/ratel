export default interface Consensus {
  id: number;
  dagit_id: number;
  artwork_id: number;
  total_oracles: number;
  votes: ConsensusVote[];
  result?: ConsensusResult | null;
}

export enum ConsensusResult {
  Accepted = 1,
  Rejected = 2,
}

export interface ConsensusVote {
  id: number;
  oracle_id: number;
  consensus_id: number;
  vote_type: ConsensusVoteType;
  description?: string | null;
}

export enum ConsensusVoteType {
  Approved = 1,
  Rejected = 2,
}
