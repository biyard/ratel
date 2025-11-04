export class InvitationMemberResponse {
  user_pk: string;
  display_name: string;
  profile_url: string;
  username: string;
  email: string;

  authorized: boolean;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.user_pk = json.user_pk;
    this.display_name = json.display_name;
    this.profile_url = json.profile_url;
    this.username = json.username;
    this.email = json.email;

    this.authorized = json.authorized;
  }
}
