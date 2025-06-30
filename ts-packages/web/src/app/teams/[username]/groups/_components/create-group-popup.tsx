'use client';
import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import Switch from '@/components/switch/switch';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { GroupPermission } from '@/lib/api/models/group';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import React, { useState } from 'react';

const PERMISSION_GROUPS: Record<
  string,
  { label: string; value: GroupPermission }[]
> = {
  Post: [
    { label: 'Read posts', value: GroupPermission.ReadPosts },
    { label: 'Write posts', value: GroupPermission.WritePosts },
    { label: 'Delete posts', value: GroupPermission.DeletePosts },
  ],
  Reply: [
    { label: 'Read replies', value: GroupPermission.ReadReplies },
    { label: 'Write replies', value: GroupPermission.WriteReplies },
    { label: 'Delete replies', value: GroupPermission.DeleteReplies },
  ],
  Admin: [
    { label: 'Manage promotions', value: GroupPermission.ManagePromotions },
    { label: 'Manage news', value: GroupPermission.ManageNews },
    { label: 'Invite member', value: GroupPermission.InviteMember },
    { label: 'Update group', value: GroupPermission.UpdateGroup },
    { label: 'Delete group', value: GroupPermission.DeleteGroup },
  ],
};

export default function CreateGroupPopup({
  onCreate,
}: {
  onCreate: (
    profileUrl: string,
    groupName: string,
    groupDescription: string,
    groupPermissions: GroupPermission[],
  ) => void;
}) {
  const [groupName, setGroupName] = useState('');
  const [groupDescription, setGroupDescription] = useState('');
  const [groupPermissions, setGroupPermissions] = useState<GroupPermission[]>(
    [],
  );
  const [groupNameRequired, setGroupNameRequired] = useState(false);
  const [imageRequired, setGroupImageRequired] = useState(false);
  const [isError, setIsError] = useState(false);

  return (
    <div className="flex flex-col w-[900px] max-w-[900px] min-w-[400px] max-h-[700px] max-mobile:!w-full max-mobile:!max-w-full gap-5 overflow-y-auto px-[20px]">
      <GroupName groupName={groupName} setGroupName={setGroupName} />
      <GroupDescription
        groupDescription={groupDescription}
        setGroupDescription={setGroupDescription}
      />
      <GroupPermissionSelector
        groupPermissions={groupPermissions}
        setGroupPermissions={setGroupPermissions}
        isError={isError}
        groupImageRequired={imageRequired}
        groupNameRequired={groupNameRequired}
      />
      <div className="flex flex-row w-full justify-end items-center px-[30px] py-[25px]">
        <CreateButton
          isEnabled={!(checkString(groupName) || checkString(groupDescription))}
          onClick={() => {
            if (checkString(groupName) || checkString(groupDescription)) {
              showErrorToast('Please remove the test keyword');
              return;
            }
            if (groupName.length == 0) {
              setGroupImageRequired(false);
              setGroupNameRequired(true);
              return;
            }
            if (groupPermissions.length == 0) {
              setGroupImageRequired(false);
              setGroupNameRequired(false);
              setIsError(true);
              return;
            }

            onCreate('', groupName, groupDescription, groupPermissions);
          }}
        />
      </div>
    </div>
  );
}

function CreateButton({
  onClick,
  isEnabled,
}: {
  isEnabled: boolean;
  onClick: () => void;
}) {
  return (
    <div
      className={`${isEnabled ? 'cursor-pointer bg-primary' : 'cursor-not-allowed bg-neutral-300'} flex flex-row w-fit h-fit px-[40px] py-[15px] rounded-[10px] font-bold text-bg text-base`}
      onClick={() => {
        onClick();
      }}
    >
      Create
    </div>
  );
}

function GroupPermissionSelector({
  groupPermissions,
  setGroupPermissions,
  isError,
  groupNameRequired,
  groupImageRequired,
}: {
  groupPermissions: GroupPermission[];
  setGroupPermissions: (groupPermissions: GroupPermission[]) => void;
  isError: boolean;
  groupNameRequired: boolean;
  groupImageRequired: boolean;
}) {
  const hasPermission = (perm: GroupPermission) =>
    groupPermissions.includes(perm);

  const togglePermission = (perm: GroupPermission) => {
    if (hasPermission(perm)) {
      setGroupPermissions(groupPermissions.filter((p) => p !== perm));
    } else {
      setGroupPermissions([...groupPermissions, perm]);
    }
  };

  const toggleAllInGroup = (perms: GroupPermission[]) => {
    const allSelected = perms.every((p) => hasPermission(p));
    if (allSelected) {
      setGroupPermissions(groupPermissions.filter((p) => !perms.includes(p)));
    } else {
      setGroupPermissions([...new Set([...groupPermissions, ...perms])]);
    }
  };

  return (
    <div className="flex flex-col w-full gap-6">
      <div className="text-[15px]/[28px] font-bold text-neutral-400">
        Permissions
      </div>

      <div className="h-[400px] overflow-y-auto px-[10px]">
        {Object.entries(PERMISSION_GROUPS).map(([groupName, perms], idx) => {
          const allChecked = perms.every((p) => hasPermission(p.value));
          return (
            <div
              key={groupName}
              className={`flex flex-col gap-1 w-full ${idx !== 0 ? 'mt-[20px]' : ''}`}
            >
              {/* Header */}
              <div className="flex justify-between items-center mb-1">
                <div className="text-sm/[20px] font-semibold text-neutral-400">
                  {groupName}
                </div>
                <div className="flex items-center gap-1">
                  <span className="text-sm/[20px] font-semibold text-neutral-400">
                    Select all
                  </span>
                  <CustomCheckbox
                    checked={allChecked}
                    onChange={() => toggleAllInGroup(perms.map((p) => p.value))}
                  />
                </div>
              </div>

              {/* Individual Toggles */}
              <div className="flex flex-col border-neutral-800 divide-y divide-neutral-800">
                {perms.map((perm) => {
                  const active = hasPermission(perm.value);
                  return (
                    <div
                      key={perm.value}
                      className="flex justify-between items-center py-2 h-[55px]"
                    >
                      <span className="text-[15px]/[24px] font-normal text-white">
                        {perm.label}
                      </span>
                      <Switch
                        checked={active}
                        onChange={() => togglePermission(perm.value)}
                      />
                    </div>
                  );
                })}
              </div>
            </div>
          );
        })}
      </div>

      <div className="mt-[20px]">
        {groupNameRequired ? (
          <div className="font-normal text-[#ef4444] text-sm">
            You should set group name.
          </div>
        ) : groupImageRequired ? (
          <div className="font-normal text-[#ef4444] text-sm">
            You should set group image.
          </div>
        ) : isError ? (
          <div className="font-normal text-[#ef4444] text-sm">
            You should select at least one option.
          </div>
        ) : null}
      </div>
    </div>
  );
}

function GroupDescription({
  groupDescription,
  setGroupDescription,
}: {
  groupDescription: string;
  setGroupDescription: (groupDescription: string) => void;
}) {
  return (
    <div className="flex flex-col w-full justify-start items-start gap-[5px]">
      <div className="font-bold text-[15px]/[28px] text-neutral-400">
        Description
      </div>

      <Textarea
        value={groupDescription}
        onChange={(e) => setGroupDescription(e.target.value)}
        maxLength={100}
        placeholder="What is the purpose of your group?"
        className="w-full px-5 py-[10px] rounded-[8px] border border-[#464646] bg-transparent text-white placeholder:text-neutral-600 text-sm outline-none resize-none"
      />

      <div className="w-full text-right text-[15px]/[22.5px] text-neutral-600">
        {`${groupDescription.length}/100`}
      </div>
    </div>
  );
}

function GroupName({
  groupName,
  setGroupName,
}: {
  groupName: string;
  setGroupName: (groupName: string) => void;
}) {
  return (
    <div className="flex flex-col w-full justify-start items-start gap-[5px]">
      <div className="flex flex-row gap-1 items-center">
        <div className="font-bold text-[15px]/[28px] text-neutral-400">
          Group name
        </div>
        <div className="font-normal text-base/[24px] text-[#eb5757]">*</div>
      </div>

      <Input
        type="text"
        value={groupName}
        onChange={(e) => setGroupName(e.target.value)}
        maxLength={100}
        placeholder="Input your group name."
        className="w-full px-5 py-[10.5px] rounded-[8px] border border-[#464646] bg-transparent text-white placeholder:text-neutral-600 text-[15px]/[22.5px] outline-none"
      />

      <div className="w-full text-right text-[15px]/[22.5px] text-neutral-600">{`${groupName.length}/100`}</div>
    </div>
  );
}
