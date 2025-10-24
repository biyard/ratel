import { ListPostResponse } from '@/features/posts/dto/list-post-response';
import { call } from './call';
import { UserDetailResponse } from './users.v3';

export async function getUserInfo(): Promise<UserDetailResponse> {
  return call('GET', '/v3/me');
}

export async function listMyPosts(
  bookmark?: string,
): Promise<ListPostResponse> {
  let path = '/v3/me/posts';
  if (bookmark) {
    path += `?bookmark=${encodeURIComponent(bookmark)}`;
  }

  return call('GET', path);
}

export async function listMyDrafts(
  bookmark?: string,
): Promise<ListPostResponse> {
  let path = '/v3/me/drafts';
  if (bookmark) {
    path += `?bookmark=${encodeURIComponent(bookmark)}`;
  }

  return call('GET', path);
}

export async function updateUserEvmAddress(
  evmAddress: string,
): Promise<UserDetailResponse> {
  return call('PATCH', '/v3/me', {
    body: { evm_address: evmAddress },
  });
}
