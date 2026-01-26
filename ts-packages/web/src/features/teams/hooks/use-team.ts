// import {
//   useQuery,
//   UseQueryResult,
//   useSuspenseQuery,
//   UseSuspenseQueryResult,
// } from '@tanstack/react-query';
// import { teamKeys } from '@/constants';
// import { call } from '@/lib/api/ratel/call';
// import { Team } from '../types/team';

// async function getTeam(teamPk: string): Promise<Team> {
//   return await call('GET', `/v3/teams/${encodeURIComponent(teamPk)}`);
// }

// export function getTeamOption(teamPk: string) {
//   return {
//     queryKey: teamKeys.detail(teamPk),
//     queryFn: async () => {
//       return await getTeam(teamPk);
//     },
//     refetchOnWindowFocus: false,
//   };
// }

// export function useSuspenseTeam(teamPk: string): UseSuspenseQueryResult<Team> {
//   return useSuspenseQuery(getTeamOption(teamPk));
// }

// export function useTeam(teamPk?: string): UseQueryResult<Team> {
//   return useQuery({
//     ...getTeamOption(teamPk!),
//     enabled: !!teamPk,
//   });
// }
