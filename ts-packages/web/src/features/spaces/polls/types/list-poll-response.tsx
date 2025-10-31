import { Poll } from './poll';

export class ListPollResponse {
  polls: Poll[];
  bookmark: string | null | undefined;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const rows = Array.isArray(json.polls) ? json.polls : [];
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.polls = rows.map((d: any) => new Poll(d));
    this.bookmark = json.bookmark;
  }
}
