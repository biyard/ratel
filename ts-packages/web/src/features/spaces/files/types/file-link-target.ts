export enum FileLinkTarget {
  Files = 'Files',
  Overview = 'Overview',
  Board = 'Board',
}

export type FileLinkTargetString = string;

export interface FileLinkInfo {
  file_url: string;
  linked_targets: FileLinkTargetString[];
  created_at: number;
  updated_at: number;
}

export function isTargetMatch(
  targetString: FileLinkTargetString,
  target: FileLinkTarget,
): boolean {
  const upperTarget = targetString.toUpperCase();

  if (target === FileLinkTarget.Board) {
    return upperTarget.startsWith('BOARD#');
  }

  return upperTarget === target.toUpperCase();
}
