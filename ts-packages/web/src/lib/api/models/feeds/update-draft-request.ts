import { FileInfo } from '../feeds';

export interface removeDraftRequest {
  delete: object;
}

export interface updateDraftRequest {
  update: {
    html_contents: string;
    industry_id: number;
    title: string;
    quote_feed_id: number | null;
    files: FileInfo[] | null;
    url: string | null;
    url_type: UrlType;
  };
}

export enum UrlType {
  None = 0,
  Image = 1,
}

export function updateDraftRequest(
  html_contents: string,
  industry_id: number,
  title: string,
  quote_feed_id: number | null,
  files: FileInfo[],
  url: string | null,
  url_type: UrlType = UrlType.None,
): updateDraftRequest {
  return {
    update: {
      html_contents,
      industry_id,
      title,
      quote_feed_id,
      files,
      url,
      url_type,
    },
  };
}

export function removeDraftRequest(): removeDraftRequest {
  return {
    delete: {},
  };
}
