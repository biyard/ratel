import React from 'react';

export class State<S> {
  constructor(private state: [S, React.Dispatch<React.SetStateAction<S>>]) {}

  get(): S {
    return this.state[0];
  }

  set(value: S) {
    this.state[1](value);
  }
}
