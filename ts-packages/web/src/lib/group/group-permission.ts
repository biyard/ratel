export enum GroupPermission {
  ReadPosts = 0,
  WritePosts = 1,
  DeletePosts = 2,
  EditPosts = 13,
  WritePendingPosts = 3,
  ReadPostDrafts = 12,

  ReadReplies = 4,
  WriteReplies = 5,
  DeleteReplies = 6,

  ReadProfile = 7,
  UpdateProfile = 8,

  InviteMember = 9,
  ManageGroup = 10,
  DeleteGroup = 11,

  ManageSpace = 20,

  ManagePromotions = 62,
  ManageNews = 63,
}
