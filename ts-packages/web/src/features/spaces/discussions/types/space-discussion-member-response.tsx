export class SpaceDiscussionMemberResponse {
  user_pk: string;

  author_display_name: string;
  author_profile_url: string;
  author_username: string;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.user_pk = json.user_pk;
    this.author_display_name = json.author_display_name;
    this.author_profile_url = json.author_profile_url;
    this.author_username = json.author_username;
  }
}
