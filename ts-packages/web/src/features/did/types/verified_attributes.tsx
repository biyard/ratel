export class VerifiedAttributes {
  public birthDate?: string;
  public gender?: string;
  public university?: string;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.birthDate = json.birth_date;
    this.gender = json.gender;
    this.university = json.university;
  }

  get age(): number | undefined {
    if (!this.birthDate) {
      return undefined;
    }

    const birthYear = Number(this.birthDate.slice(0, 4));
    const nowYear = new Date().getFullYear();

    return nowYear - birthYear;
  }
}
