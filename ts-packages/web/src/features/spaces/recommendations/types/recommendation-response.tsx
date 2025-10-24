import FileType from '../../files/types/file';

export class SpaceRecommendationResponse {
  html_contents: string;
  files: FileType[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.html_contents = json.html_contents;
    this.files = json.files;
  }
}
