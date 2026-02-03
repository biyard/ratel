// export interface CreateGroupRequest {
//   create: {
//     name: string;
//     description: string;
//     image_url: string;
//     users: number[];
//     permissions: GroupPermission[];
//   };
// }

// export function createGroupRequest(
//   name: string,
//   description: string,
//   image_url: string,
//   users: number[],
//   permissions: GroupPermission[],
// ): CreateGroupRequest {
//   return {
//     create: {
//       name,
//       description,
//       image_url,
//       users,
//       permissions,
//     },
//   };
// }

// export interface DeleteGroupRequest {
//   delete: object;
// }

// export function deleteGroupRequest(): DeleteGroupRequest {
//   return {
//     delete: {},
//   };
// }

// export interface InviteMemberRequest {
//   invite_member: {
//     user_ids: number[];
//   };
// }

// export function inviteMemberRequest(user_ids: number[]): InviteMemberRequest {
//   return {
//     invite_member: {
//       user_ids,
//     },
//   };
// }

// export interface CheckEmailRequest {
//   check_email: {
//     email: string;
//   };
// }

// export function checkEmailRequest(email: string): CheckEmailRequest {
//   return {
//     check_email: {
//       email,
//     },
//   };
// }

// export const GroupPermission = {
//   ReadPosts: 0,
//   WritePosts: 1,
//   DeletePosts: 2,
//   WritePendingPosts: 3,

//   ReadReplies: 4,
//   WriteReplies: 5,
//   DeleteReplies: 6,

//   ReadProfile: 7,
//   UpdateProfile: 8,

//   InviteMember: 9,
//   UpdateGroup: 10,
//   DeleteGroup: 11,

//   ManageSpace: 20,

//   ManagePromotions: 62,
//   ManageNews: 63,
// } as const;

// export type GroupPermission =
//   (typeof GroupPermission)[keyof typeof GroupPermission];
