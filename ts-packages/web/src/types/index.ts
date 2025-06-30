import { Dispatch, SetStateAction } from 'react';

export * from './thread';

export type StateSetter<T> = Dispatch<SetStateAction<T>>;
export const noop: Dispatch<SetStateAction<boolean>> = () => {};
