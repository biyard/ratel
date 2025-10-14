import { File } from '@/features/deliberation-space/utils/deliberation.spaces.v3';

export interface Thread {
  html_contents: string;
  files: File[];
}
