import TeamSelector from '@/app/(social)/_components/team-selector';
import { Team } from '@/features/teams/types/team';

export interface TeamProfileProps {
  team?: Team;
}

export default function TeamProfile({ team }: TeamProfileProps) {
  if (!team) {
    return <div></div>;
  }

  return (
    <div className="flex flex-col gap-5 px-4 py-5 rounded-[10px] bg-card-bg border border-card-border">
      <TeamSelector team={team} />
      <div className="relative">
        {team.profile_url && team.profile_url !== '' ? (
          <img
            src={team?.profile_url}
            alt={team?.nickname ?? 'team profile'}
            width={80}
            height={80}
            className="rounded-full border-2 object-cover object-top w-20 h-20"
          />
        ) : (
          <div className="w-20 h-20 rounded-full bg-profile-bg" />
        )}
      </div>

      <div className="font-medium text-text-primary">{team.nickname}</div>

      <div
        id="user-profile-description"
        className="text-xs text-desc-text"
        dangerouslySetInnerHTML={{ __html: team.html_contents }}
      />
    </div>
  );
}
