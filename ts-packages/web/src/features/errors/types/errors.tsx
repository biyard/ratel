export class Error {
  constructor(public label: string) {}

  get message(): string {
    return `${this.label}.message`;
  }

  get title(): string {
    return `${this.label}.title`;
  }

  // TODO: implement from ratel error
}

// Space Poll Errors
export const ErrorSpacePollRequiredField = new Error(
  'space.poll.required_field',
);
