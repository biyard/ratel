import { File } from '../utils/deliberation.spaces.v3';

export interface FinalConsensus {
  drafts: RecommendationCreateRequest;
}

export interface RecommendationCreateRequest {
  html_contents: string;
  files: File[];
}
