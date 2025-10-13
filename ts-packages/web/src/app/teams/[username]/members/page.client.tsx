'use client';
import { useTeamMembers } from '../../_hooks/use-team';
import { checkString } from '@/lib/string-filter-utils';

export default function TeamMembers({ username }: { username: string }) {
  const query = useTeamMembers(username);
  const membersData = query.data;

  const members = membersData?.members?.filter(
    (member) =>
      member !== undefined &&
      !(checkString(member.display_name) || checkString(member.username))
  ) ?? [];

  return (
    <div className="flex flex-col w-full max-w-[1152px] px-4 py-5 gap-[10px] bg-card-bg border border-card-border rounded-lg h-fit">
      {members.map((member) => (
        <div
          key={member.user_id}
          className="flex flex-col w-full h-fit gap-[15px] bg-transparent rounded-sm border border-card-border p-5"
        >
          <div
            key={member.user_id}
            className="flex flex-row w-full h-fit gap-[15px] bg-transparent"
          >
            {!member.profile_url ||
            member.profile_url.includes('test') ||
            member.profile_url === '' ? (
              <div className="w-12 h-12 rounded-full bg-profile-bg" />
            ) : (
              <img
                src={member.profile_url}
                alt={member.username}
                width={48}
                height={48}
                className="rounded-lg object-cover w-12 h-12"
              />
            )}

            <div className="flex flex-col justify-between items-start flex-1 min-w-0">
              <div className="font-bold text-text-primary text-base/[20px]">
                {member.username}
              </div>
              <div className="font-semibold text-desc-text text-sm/[20px]">
                {member.display_name}
              </div>
              {member.is_owner && (
                <div className="text-xs text-blue-500 font-medium">
                  Team Owner
                </div>
              )}
            </div>
          </div>

          <div className="flex flex-wrap w-full justify-start items-center gap-[10px]">
            {member.groups
              .filter((group) => !checkString(group.group_name))
              .map((group) => (
                <div
                  key={group.group_id}
                  className="flex flex-row w-fit h-fit px-[5px] py-[3px] border border-neutral-800 bg-black light:bg-neutral-600 light:border-transparent rounded-lg font-medium text-base text-white"
                >
                  {group.group_name}
                </div>
              ))}
          </div>
        </div>
      ))}
    </div>
  );
}
