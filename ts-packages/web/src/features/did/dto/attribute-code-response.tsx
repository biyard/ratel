export type Gender = 'male' | 'female';

export class AttributeCodeResponse {
  pk: string;
  sk: string;
  created_at: number;
  birth_date?: string;
  gender?: Gender;
  university?: string;

  constructor(json: any) {
    this.pk = json.pk;
    this.sk = json.sk;
    this.created_at = json.created_at;
    this.birth_date = json.birth_date;
    this.gender = json.gender;
    this.university = json.university;
  }

  get code(): string {
    // Extract the code from the partition key (format: "AttributeCode#CODE")
    return this.pk.split('#')[1] || this.pk;
  }

  getFormattedDate(): string {
    return new Date(this.created_at).toLocaleDateString();
  }

  getDisplayAttributes(): string {
    const attrs: string[] = [];
    if (this.birth_date) attrs.push(`Birth: ${this.birth_date}`);
    if (this.gender) attrs.push(`Gender: ${this.gender}`);
    if (this.university) attrs.push(`University: ${this.university}`);
    return attrs.length > 0 ? attrs.join(', ') : 'No attributes';
  }
}
