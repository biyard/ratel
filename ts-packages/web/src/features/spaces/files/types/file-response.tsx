import { File } from '@/lib/api/models/feeds';

export class FileResponse {
  files: File[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.files = json.files;
  }
}
