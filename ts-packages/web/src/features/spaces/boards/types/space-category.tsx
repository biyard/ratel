export class SpaceCategory {
  categories: string[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.categories = json;
  }
}
