import { PollAnswer, PollQuestion } from './poll-question';

export class Poll {
  public pk: string;
  public sk: string;
  public created_at: number;
  public updated_at: number;
  public started_at: number;
  public ended_at: number;
  public response_editable: boolean;
  public user_response_count: number;

  public questions: PollQuestion[];
  public myResponse?: PollAnswer[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.pk = json.pk;
    this.sk = json.sk;
    this.created_at = json.created_at;

    this.updated_at = json.updated_at;
    this.started_at = json.started_at;
    this.ended_at = json.ended_at;
    this.response_editable = json.response_editable;
    this.user_response_count = json.user_response_count;
    this.questions = json.questions || [];
    this.myResponse = json.my_response || [];
  }
}
