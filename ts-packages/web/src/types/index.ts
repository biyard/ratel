import type { Dispatch, SetStateAction } from 'react';

export type StateSetter<T> = Dispatch<SetStateAction<T>>;
export const noop: Dispatch<SetStateAction<boolean>> = () => {};
