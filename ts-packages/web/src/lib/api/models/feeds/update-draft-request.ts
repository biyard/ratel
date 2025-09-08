import { ArtworkMetadata, FeedType, FileInfo, UrlType } from '../feeds';

export interface removeDraftRequest {
  delete: object;
}

export interface updateDraftRequest {
  html_contents: string;
  feed_type: FeedType | null;
  industry_id: number | null;
  title: string;
  quote_feed_id: number | null;
  files: FileInfo[] | null;
  url: string | null;
  url_type: UrlType;
  artwork_metadata: ArtworkMetadata | null;
}

export function updateDraftRequest(
  html_contents: string,
  industry_id: number | null = null,
  title: string,
  quote_feed_id: number | null,
  files: FileInfo[],
  url: string | null,
  url_type: UrlType = UrlType.None,
  feed_type: FeedType | null = null,
  artwork_metadata: ArtworkMetadata | null = null,
): updateDraftRequest {
  return {
    html_contents,
    feed_type,
    industry_id,
    title,
    quote_feed_id,
    files,
    url,
    url_type,
    artwork_metadata,
  };
}

export function removeDraftRequest(): removeDraftRequest {
  return {
    delete: {},
  };
}
