// DEPRECATED: This file has been moved to @/features/teams/api
// This file now re-exports from the new location for backward compatibility
// Please update your imports to use: import * as teamsApi from '@/features/teams/api'

export * from '@/features/teams/api';
export { TeamGroupPermission } from '@/features/auth/utils/team-group-permissions';

// Note: TeamGroupResponse.sk is now TeamGroupResponse.id
// Note: TeamOwnerResponse.user_pk is now TeamOwnerResponse.id
