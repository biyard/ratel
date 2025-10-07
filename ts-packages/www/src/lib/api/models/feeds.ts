import type { Industry } from './industry';
import type { Space } from './spaces';
import type { User } from './user';

export const UrlType = {
  None: 0,
  Image: 1,
} as const;

export type UrlType = typeof UrlType[keyof typeof UrlType];

export interface ArtworkTrait {
  trait_type: string;
  value: string | number | boolean | Record<string, unknown> | null;
  display_type?: ArtworkTraitDisplayType | null;
}

export const ArtworkTraitDisplayType = {
  String: 'string',
  Color: 'color',
  Number: 'number',
} as const;

export type ArtworkTraitDisplayType = typeof ArtworkTraitDisplayType[keyof typeof ArtworkTraitDisplayType];

export interface ArtworkMetadata {
  traits: ArtworkTrait[];
}

export interface FeedListResponse {
  posts: Feed[];
  is_ended: boolean | null;
}

export type PartitionString = string;
export type EntityTypeString = string;

export type Visibility =
  | 'Public'
  | `Team:${string}`
  | `TeamGroupMember:${string}`;

export type PostType = string;
export type PostStatus = string;
export type BoosterType = string;
export type SortedVisibility = string;

export interface FeedV2 {
  pk: PartitionString;
  sk: EntityTypeString;

  created_at: number;
  updated_at: number;

  title: string;
  html_contents: string;
  post_type: PostType;

  status: PostStatus;
  visibility?: Visibility | null;

  shares: number;
  likes: number;
  comments: FeedComment[];

  user_pk: PartitionString;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;

  space_pk?: PartitionString | null;
  booster?: BoosterType | null;
  rewards?: number | null;

  sorted_visibility: SortedVisibility;
  urls: string[];
}

export interface FeedComment {
  pk: PartitionString;
  sk: EntityTypeString;

  updated_at: number;

  content: string;

  author_pk: PartitionString;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;
}

export interface Feed {
  id: number | string;
  created_at: number;
  updated_at: number;

  html_contents: string;

  feed_type: FeedType;

  user_id: number;
  industry_id: number;

  proposer_name?: string | null;
  profile_image?: string | null;

  parent_id?: number | null;
  title?: string | null;
  part_id?: number | null;
  quote_feed_id?: number | null;

  likes: number;
  is_liked: boolean;
  comments: number;
  comment_list: Comment[];
  files: FileInfo[];
  rewards: number;
  shares: number;

  url?: string;
  url_type: UrlType;
  status: FeedStatus;

  author: [User];
  industry: [Industry];
  spaces: [Space];
  space: [Space];
  onboard?: boolean;

  artwork_metadata?: ArtworkMetadata;
}

export interface Comment {
  id: number | string;
  created_at: number;
  feed_type: FeedType;
  user_id: number;
  parent_id?: number | null;
  quote_feed_id?: number | null;
  html_contents: string;
  num_of_likes: number;
  is_liked: boolean;
  num_of_replies: number;
  author: [User];
  quote_comment?: Comment | null;
  replies: Reply[];
}

export interface Reply {
  id: number;
  html_contents: string;
  author: [User];
}
export const FeedType = {
  Artwork: 0,
  Post: 1,
  Reply: 2,
  Repost: 3,
  DocReview: 4,
} as const;

export type FeedType = typeof FeedType[keyof typeof FeedType];

export const FeedStatus = {
  Draft: 1,
  Published: 2,
} as const;

export type FeedStatus = typeof FeedStatus[keyof typeof FeedStatus];

export interface FileInfo {
  name: string;
  size: string;
  ext: string;
  url?: string | null;
}

export const FileExtension = {
  JPG: 'JPG',
  PNG: 'PNG',
  PDF: 'PDF',
  ZIP: 'ZIP',
  WORD: 'WORD',
  PPTX: 'PPTX',
  EXCEL: 'EXCEL',
  MP4: 'MP4',
  MOV: 'MOV',
} as const;

export type FileExtension = typeof FileExtension[keyof typeof FileExtension];

export const FileExtensionLabel: Record<
  FileExtension,
  { ko: string; en: string }
> = {
  [FileExtension.JPG]: { ko: 'JPG', en: 'JPG' },
  [FileExtension.PNG]: { ko: 'PNG', en: 'PNG' },
  [FileExtension.PDF]: { ko: 'PDF', en: 'PDF' },
  [FileExtension.ZIP]: { ko: 'ZIP', en: 'ZIP' },
  [FileExtension.WORD]: { ko: 'WORD', en: 'WORD' },
  [FileExtension.PPTX]: { ko: 'PPTX', en: 'PPTX' },
  [FileExtension.EXCEL]: { ko: 'EXCEL', en: 'EXCEL' },
  [FileExtension.MP4]: { ko: 'MP4', en: 'MP4' },
  [FileExtension.MOV]: { ko: 'MOV', en: 'MOV' },
};

export function toFileExtension(
  input: string | undefined | null,
): FileExtension {
  if (!input) return FileExtension.PDF;

  const s = input.trim().toLowerCase();

  const mimeMap: Record<string, string> = {
    'image/jpeg': 'jpg',
    'image/jpg': 'jpg',
    'image/png': 'png',
    'application/pdf': 'pdf',
    'application/zip': 'zip',
    'application/x-zip-compressed': 'zip',
    'application/msword': 'doc',
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document':
      'docx',
    'application/vnd.ms-powerpoint': 'ppt',
    'application/vnd.openxmlformats-officedocument.presentationml.presentation':
      'pptx',
    'application/vnd.ms-excel': 'xls',
    'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet': 'xlsx',
    'text/csv': 'csv',
    'video/mp4': 'mp4',
    'video/quicktime': 'mov',
  };

  let ext = mimeMap[s];

  if (!ext) {
    if (s.includes('.')) {
      ext = s.split('.').pop() || '';
    } else if (s.startsWith('.')) {
      ext = s.slice(1);
    } else {
      ext = s;
    }
  }

  switch (ext) {
    case 'jpg':
    case 'jpeg':
      return FileExtension.JPG;
    case 'png':
      return FileExtension.PNG;
    case 'pdf':
      return FileExtension.PDF;
    case 'zip':
      return FileExtension.ZIP;
    case 'doc':
    case 'docx':
      return FileExtension.WORD;
    case 'ppt':
    case 'pptx':
      return FileExtension.PPTX;
    case 'xls':
    case 'xlsx':
    case 'csv':
      return FileExtension.EXCEL;
    case 'mp4':
      return FileExtension.MP4;
    case 'mov':
      return FileExtension.MOV;
    default:
      return FileExtension.PDF; // 모르면 PDF로
  }
}
