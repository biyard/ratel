import { FileInfo } from './feeds';

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
