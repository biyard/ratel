import { useSuspenseFindTeam } from '@/features/teams/hooks/use-find-team';
import { useSuspenseTeamMembers } from '@/features/teams/hooks/use-team-members';
import { checkString } from '@/lib/string-filter-utils';

export function useMembersData(username: string) {
  const { data: team } = useSuspenseFindTeam(username);
  const { data: teamMembers } = useSuspenseTeamMembers(username);

  const members =
    teamMembers?.items?.filter(
      (member) =>
        member !== undefined &&
        !(checkString(member.display_name) || checkString(member.username)),
    ) ?? [];

  return {
    team,
    members,
  };
}
