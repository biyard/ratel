import { ListPostResponse } from '@/features/posts/dto/list-post-response';
import { DidDocument } from '@/features/did/types/did-document';
import { MySpace } from '@/features/spaces/types/space-common';
import { call } from './call';
import { UserDetailResponse } from './users.v3';

export interface ListMySpacesResponse {
  items: MySpace[];
  bookmark?: string;
}

export type MembershipTier = 'Free' | 'Pro' | 'Max' | 'Vip';

export interface UserMembershipResponse {
  tier: string;
  expired_at: number;
  total_credits: number;
  remaining_credits: number;
  next_membership?: string;
}

export interface PurchaseHistoryItem {
  tx_type: string;
  amount: number;
  payment_id: string;
  tx_id: string;
  created_at: number;
}

export interface ListPurchaseHistoryResponse {
  items: PurchaseHistoryItem[];
  bookmark?: string;
}

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

export async function listMySpaces(
  bookmark?: string,
): Promise<ListMySpacesResponse> {
  let path = '/v3/me/spaces';
  if (bookmark) {
    path += `?bookmark=${encodeURIComponent(bookmark)}`;
  }

  return call('GET', path);
}

export async function updateUserEvmAddress(
  evmAddress: string,
): Promise<UserDetailResponse> {
  return call('PATCH', '/v3/me', {
    EvmAddress: {
      evm_address: evmAddress,
    },
  });
}

export async function updateUserProfile(
  nickname: string,
  profileUrl: string,
  description: string,
): Promise<UserDetailResponse> {
  return call('PATCH', '/v3/me', {
    body: {
      Profile: {
        nickname,
        profile_url: profileUrl,
        description,
      },
    },
  });
}

export async function getDid(): Promise<DidDocument> {
  return call('GET', '/v3/me/did');
}

export async function getUserMembership(): Promise<UserMembershipResponse> {
  return call('GET', '/v3/me/memberships');
}

export async function getPurchaseHistory(
  bookmark?: string,
): Promise<ListPurchaseHistoryResponse> {
  let path = '/v3/me/memberships/history';
  if (bookmark) {
    path += `?bookmark=${encodeURIComponent(bookmark)}`;
  }

  return call('GET', path);
}
