export class SpaceDiscussionResponse {
  pk: string;

  started_at: number;
  ended_at: number;

  name: string;
  description: string;
  meeting_id: string | undefined | null;
  pipeline_id: string;

  media_pipeline_arn: string | undefined | null;
  record: string | undefined | null;

  is_member: boolean;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.pk = json.pk;
    this.started_at = json.started_at;
    this.ended_at = json.ended_at;

    this.name = json.name;
    this.description = json.description;
    this.meeting_id = json.meeting_id;
    this.pipeline_id = json.pipeline_id;

    this.media_pipeline_arn = json.media_pipeline_arn;
    this.record = json.record;

    this.is_member = json.is_member;
  }
}
