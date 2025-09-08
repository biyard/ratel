import { Industry } from './industry';
import { Space } from './spaces';
import { User } from './user';

export enum UrlType {
  None = 0,
  Image = 1,
}

export interface ArtworkMetadata {
  artist_name: string;
  size: string;
  background_color: string;
}

export interface Feed {
  id: number;
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
  onboard?: boolean;

  artwork_metadata?: ArtworkMetadata;
}

export interface Comment {
  id: number;
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
export enum FeedType {
  Artwork = 0,
  Post = 1,
  Reply = 2,
  Repost = 3,
  DocReview = 4,
}

export enum FeedStatus {
  Draft = 1,
  Published = 2,
}

export interface FileInfo {
  name: string;
  size: string;
  ext: string;
  url?: string | null;
}

export enum FileExtension {
  JPG = 1,
  PNG = 2,
  PDF = 3,
  ZIP = 4,
  WORD = 5,
  PPTX = 6,
  EXCEL = 7,
  MP4 = 8,
  MOV = 9,
}

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
