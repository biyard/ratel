import FileModel from './file';

export class FileResponse {
  files: FileModel[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.files = json.files;
  }
}
