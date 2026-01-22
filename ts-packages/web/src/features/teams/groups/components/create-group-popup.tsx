'use client';
import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import Switch from '@/components/switch/switch';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { checkString } from '@/lib/string-filter-utils';
import { showErrorToast } from '@/lib/toast';
import { useState } from 'react';
import { TeamGroupPermission } from '@/features/auth/utils/team-group-permissions';
import {
  useTeamGroupsI18n,
  type TeamGroupsI18n,
} from '@/features/teams/groups/i18n';

function getPermissionGroups(i18n: TeamGroupsI18n) {
  return {
    [i18n.permission_group_post]: [
      { label: i18n.permission_post_read, value: TeamGroupPermission.PostRead },
      {
        label: i18n.permission_post_write,
        value: TeamGroupPermission.PostWrite,
      },
      {
        label: i18n.permission_post_delete,
        value: TeamGroupPermission.PostDelete,
      },
    ],
    [i18n.permission_group_admin]: [
      {
        label: i18n.permission_group_edit,
        value: TeamGroupPermission.GroupEdit,
      },
      {
        label: i18n.permission_team_edit,
        value: TeamGroupPermission.TeamEdit,
      },
      {
        label: i18n.permission_team_admin,
        value: TeamGroupPermission.TeamAdmin,
      },
    ],
  };
}

export default function CreateGroupPopup({
  onCreate,
}: {
  onCreate: (
    profileUrl: string,
    groupName: string,
    groupDescription: string,
    groupPermissions: TeamGroupPermission[],
  ) => void;
}) {
  const i18n = useTeamGroupsI18n();
  const [groupName, setGroupName] = useState('');
  const [groupDescription, setGroupDescription] = useState('');
  const [groupPermissions, setGroupPermissions] = useState<
    TeamGroupPermission[]
  >([]);
  const [groupNameRequired, setGroupNameRequired] = useState(false);
  const [imageRequired, setGroupImageRequired] = useState(false);
  const [isError, setIsError] = useState(false);

  return (
    <div
      className="flex flex-col w-tablet max-w-tablet min-w-[400px]
    max-h-[700px] max-mobile:w-full! max-mobile:max-w-full!
    gap-5 overflow-y-auto px-[20px]
    custom-scrollbar"
    >
      <GroupName
        i18n={i18n}
        groupName={groupName}
        setGroupName={setGroupName}
      />
      <GroupDescription
        i18n={i18n}
        groupDescription={groupDescription}
        setGroupDescription={setGroupDescription}
      />
      <GroupPermissionSelector
        i18n={i18n}
        groupPermissions={groupPermissions}
        setGroupPermissions={setGroupPermissions}
        isError={isError}
        groupImageRequired={imageRequired}
        groupNameRequired={groupNameRequired}
      />
      <div className="flex flex-row w-full justify-end items-center px-[30px] py-[25px]">
        <CreateButton
          i18n={i18n}
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
  i18n,
  onClick,
  isEnabled,
}: {
  i18n: TeamGroupsI18n;
  isEnabled: boolean;
  onClick: () => void;
}) {
  return (
    <div
      data-pw="create-group-submit-button"
      className={`${isEnabled ? 'cursor-pointer bg-primary' : 'cursor-not-allowed bg-neutral-300'} flex flex-row w-fit h-fit px-[40px] py-[15px] rounded-[10px] font-bold text-bg text-base`}
      onClick={() => {
        onClick();
      }}
    >
      {i18n.create}
    </div>
  );
}

function GroupPermissionSelector({
  i18n,
  groupPermissions,
  setGroupPermissions,
  isError,
  groupNameRequired,
  groupImageRequired,
}: {
  i18n: TeamGroupsI18n;
  groupPermissions: TeamGroupPermission[];
  setGroupPermissions: (groupPermissions: TeamGroupPermission[]) => void;
  isError: boolean;
  groupNameRequired: boolean;
  groupImageRequired: boolean;
}) {
  const PERMISSION_GROUPS = getPermissionGroups(i18n);

  const hasPermission = (perm: TeamGroupPermission) =>
    groupPermissions.includes(perm);

  const togglePermission = (perm: TeamGroupPermission) => {
    if (hasPermission(perm)) {
      setGroupPermissions(groupPermissions.filter((p) => p !== perm));
    } else {
      setGroupPermissions([...groupPermissions, perm]);
    }
  };

  const toggleAllInGroup = (perms: TeamGroupPermission[]) => {
    const allSelected = perms.every((p) => hasPermission(p));
    if (allSelected) {
      setGroupPermissions(groupPermissions.filter((p) => !perms.includes(p)));
    } else {
      setGroupPermissions([...new Set([...groupPermissions, ...perms])]);
    }
  };

  return (
    <div className="flex flex-col w-full gap-6">
      <div className="text-[15px]/[28px] font-bold text-modal-label-text">
        {i18n.permission}
      </div>

      <div className="px-[10px]">
        {Object.entries(PERMISSION_GROUPS).map(([groupName, perms], idx) => {
          const allChecked = perms.every((p) => hasPermission(p.value));
          return (
            <div
              key={groupName}
              className={`flex flex-col gap-1 w-full ${idx !== 0 ? 'mt-[20px]' : ''}`}
            >
              {/* Header */}
              <div className="flex justify-between items-center mb-1">
                <div className="text-sm/[20px] font-semibold text-modal-label-text">
                  {groupName}
                </div>
                <div className="flex items-center gap-1">
                  <span className="text-sm/[20px] font-semibold text-modal-label-text">
                    {i18n.select_all}
                  </span>
                  <CustomCheckbox
                    checked={allChecked}
                    onChange={() => toggleAllInGroup(perms.map((p) => p.value))}
                    data-pw={`permission-select-all-${groupName.toLowerCase()}`}
                  />
                </div>
              </div>

              {/* Individual Toggles */}
              <div className="flex flex-col border-neutral-800 divide-y divide-divider">
                {perms.map((perm) => {
                  const active = hasPermission(perm.value);
                  return (
                    <div
                      key={perm.value}
                      className="flex justify-between items-center py-2 h-[55px]"
                      data-pw={`permission-item-${perm.value}`}
                    >
                      <span className="text-[15px]/[24px] font-normal text-text-primary">
                        {perm.label}
                      </span>
                      <Switch
                        checked={active}
                        onChange={() => togglePermission(perm.value)}
                        data-pw={`permission-toggle-${perm.value}`}
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
          <div className="font-normal text-post-required-marker text-sm">
            {i18n.group_name_required}
          </div>
        ) : groupImageRequired ? (
          <div className="font-normal text-post-required-marker text-sm">
            {i18n.group_image_required}
          </div>
        ) : isError ? (
          <div className="font-normal text-post-required-marker text-sm">
            {i18n.group_option_required}
          </div>
        ) : null}
      </div>
    </div>
  );
}

function GroupDescription({
  i18n,
  groupDescription,
  setGroupDescription,
}: {
  i18n: TeamGroupsI18n;
  groupDescription: string;
  setGroupDescription: (groupDescription: string) => void;
}) {
  return (
    <div className="flex flex-col w-full justify-start items-start gap-[5px]">
      <div className="font-bold text-[15px]/[28px] text-modal-label-text">
        {i18n.description}
      </div>

      <Textarea
        value={groupDescription}
        onChange={(e) => setGroupDescription(e.target.value)}
        maxLength={100}
        placeholder={i18n.description_hint}
        className="w-full px-5 py-[10px] rounded-[8px] border border-input-box-border bg-input-box-bg text-text-primary placeholder:text-neutral-600 text-sm outline-none resize-none"
        data-pw="create-group-description-input"
      />

      <div className="w-full text-right text-[15px]/[22.5px] text-neutral-600">
        {`${groupDescription.length}/100`}
      </div>
    </div>
  );
}

function GroupName({
  i18n,
  groupName,
  setGroupName,
}: {
  i18n: TeamGroupsI18n;
  groupName: string;
  setGroupName: (groupName: string) => void;
}) {
  return (
    <div className="flex flex-col w-full justify-start items-start gap-[5px]">
      <div className="flex flex-row gap-1 items-center">
        <div className="font-bold text-[15px]/[28px] text-modal-label-text">
          {i18n.group_name}
        </div>
        <div className="font-normal text-base/[24px] text-[#eb5757]">*</div>
      </div>

      <Input
        type="text"
        value={groupName}
        onChange={(e) => setGroupName(e.target.value)}
        maxLength={100}
        placeholder={i18n.group_name_hint}
        className="w-full px-5 py-[10.5px] rounded-[8px] border border-input-box-border bg-input-box-bg text-text-primary placeholder:text-neutral-600 text-[15px]/[22.5px] outline-none"
        data-pw="create-group-name-input"
      />

      <div className="w-full text-right text-[15px]/[22.5px] text-neutral-600">{`${groupName.length}/100`}</div>
    </div>
  );
}
