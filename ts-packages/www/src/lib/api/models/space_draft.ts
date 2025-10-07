import { FileInfo } from './feeds';

export interface SpaceDraft {
  id: number;
  created_at: number;
  updated_at: number;

  space_id: number;

  title: string;
  html_contents: string;
  files: FileInfo[];
}

export interface SpaceDraftCreateRequest {
  title: string;
  html_contents: string;
  files: FileInfo[];
}
