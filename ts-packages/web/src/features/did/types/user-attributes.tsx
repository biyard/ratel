export class UserAttributes {
  public age?: number;
  public gender?: string;
  public university?: string;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.age = json.age;
    this.gender = json.gender;
    this.university = json.university;
  }
}
