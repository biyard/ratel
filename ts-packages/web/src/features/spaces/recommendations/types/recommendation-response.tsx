import FileModel from '../../files/types/file';

export class SpaceRecommendationResponse {
  html_contents: string;
  files: FileModel[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.html_contents = json.html_contents;
    this.files = json.files;
  }
}
