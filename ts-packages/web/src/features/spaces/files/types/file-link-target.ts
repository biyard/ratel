export enum FileLinkTarget {
  Files = 'Files',
  Overview = 'Overview',
  /** Base identifier for board targets. Actual values are "Board#<post_id>" */
  Board = 'Board',
}

export type FileLinkTargetString = string;

/** Creates a properly formatted board target string */
export function createBoardTarget(postId: string): FileLinkTargetString {
  return `Board#${postId}`;
}

export interface FileLinkInfo {
  file_url: string;
  link_target: FileLinkTargetString;
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
