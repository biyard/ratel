import { File } from '@/lib/api/models/feeds';

export class SpaceRecommendationResponse {
  html_contents: string;
  files: File[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.html_contents = json.html_contents;
    this.files = json.files;
  }
}
