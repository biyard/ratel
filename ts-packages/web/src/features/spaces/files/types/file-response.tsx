import FileType from './file';

export class FileResponse {
  files: FileType[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.files = json.files;
  }
}
