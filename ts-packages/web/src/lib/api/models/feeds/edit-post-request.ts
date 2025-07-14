import { FileInfo } from '../feeds';
import { UrlType } from './update-draft-request';

export interface editPostRequest {
  edit: {
    html_contents: string;
    industry_id: number;
    title: string;
    quote_feed_id: number | null;
    files: FileInfo[] | null;
    url: string | null;
    url_type: UrlType;
  };
}

export function editPostRequest(
  html_contents: string,
  industry_id: number,
  title: string,
  quote_feed_id: number | null,
  files: FileInfo[],
  url: string | null,
  url_type: UrlType = UrlType.None,
): editPostRequest {
  return {
    edit: {
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
