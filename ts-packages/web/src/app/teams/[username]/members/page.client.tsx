'use client';
import React from 'react';
import { useTeamByUsername } from '../../_hooks/use-team';
import { User } from '@/lib/api/models/user';
import Image from 'next/image';
import { checkString } from '@/lib/string-filter-utils';

export default function TeamMembers({ username }: { username: string }) {
  const query = useTeamByUsername(username);

  const members: User[] = (query.data?.members ?? [])
    .flat()
    .filter(
      (g): g is User =>
        g !== undefined &&
        !(checkString(g.nickname) || checkString(g.html_contents)),
    );

  const team = query.data;

  return (
    <div className="flex flex-col w-full max-w-[1152px] px-4 py-5 gap-[10px] bg-[#191919] rounded-lg h-fit">
      {members.map((member) => (
        <div
          key={member.id}
          className="flex flex-col w-full h-fit gap-[15px] bg-transparent rounded-sm border border-neutral-800 p-5"
        >
          <div
            key={member.id}
            className="flex flex-row w-full h-fit gap-[15px] bg-transparent"
          >
            {!member.profile_url || member.profile_url.includes('test') ? (
              <div className="w-12 h-12 rounded-full bg-neutral-500" />
            ) : (
              <Image
                src={member.profile_url || '/default-profile.png'}
                alt={member.username}
                width={48}
                height={48}
                className="rounded-lg object-cover w-12 h-12"
              />
            )}

            <div className="flex flex-col justify-between items-start flex-1 min-w-0">
              <div className="font-bold text-white text-base/[20px]">
                {member.email}
              </div>
              <div className="font-semibold text-neutral-400 text-sm/[20px]">
                {member.username}
              </div>
            </div>
          </div>

          <div className="flex flex-wrap w-full justify-start items-start gap-[10px]">
            {member.groups
              .filter(
                (group) =>
                  group.creator_id === team.id && !checkString(group.name),
              )
              .map((group) => (
                <div
                  key={group.id}
                  className="flex flex-row w-fit h-fit px-[5px] py-[3px] border border-neutral-800 bg-black rounded-lg font-medium text-base text-white"
                >
                  {group.name}
                </div>
              ))}
          </div>
        </div>
      ))}
    </div>
  );
}
