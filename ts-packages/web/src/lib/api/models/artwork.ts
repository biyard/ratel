import { ConsensusVoteType } from './consensus';
import { FileInfo } from './feeds';

export default interface Artwork {
  id: number;
  created_at: number;
  owner_id: number;
  title: string;
  description?: string;
  file: FileInfo;
  is_certified: boolean;
  is_voted: boolean;
  has_consensus: boolean;
}

export interface ArtworkDetail {
  id: number;
  created_at: number;
  updated_at: number;
  artwork_id: number;
  owner_id: number;
  image: string;
  is_certified: boolean;
}

export interface ArtworkCertificate {
  total_votes: number;
  total_oracles: number;
  approved_votes: number;
  rejected_votes: number;
  certified_at: number;
  voters: {
    nickname: string;
    vote_type: ConsensusVoteType;
    description?: string | null;
  }[];
}
