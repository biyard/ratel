import { FileInfo } from './feeds';
import { File } from './spaces/deliberation-spaces';

export interface Elearning {
  id: number;
  created_at: number;
  updated_at: number;

  space_id: number;
  files: FileInfo[];
}

export interface ElearningCreateRequest {
  files: FileInfo[];
}

export interface NewElearningCreateRequest {
  files: File[];
}
