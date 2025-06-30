'use client';

import { checkEmailFormat } from '@/lib/valid-utils';
import { useEffect, useState } from 'react';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';

import { Input } from '@/components/ui/input';
import { ratelApi } from '@/lib/api/ratel_api';
import { User } from '@/lib/api/models/user';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';
import { LoadablePrimaryButton } from '@/components/button/primary-button';
import { Remove } from '@/components/icons';
import { logger } from '@/lib/logger';

export default function InviteCommittee({
  onSend,
}: {
  onSend: (user_ids: number[]) => void;
}) {
  const [input, setInput] = useState('');
  const [emails, setEmails] = useState<string[]>([]);
  const [isLoading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [emailUserMap, setEmailUserMap] = useState<Map<string, User>>(
    new Map(),
  );

  const addUserToMap = (email: string, user: User) => {
    const newMap = new Map(emailUserMap);
    newMap.set(email, user);
    setEmailUserMap(newMap);
  };

  const parseEmail = () => {
    const email = input
      .split(',')
      .map((email) => email.trim())
      .filter((email) => checkEmailFormat(email));
    setEmails(email);
  };

  const handleSend = async () => {
    setLoading(true);
    const user_ids: number[] = [];
    for (const email of emails) {
      const user = emailUserMap.get(email);
      try {
        if (user) {
          user_ids.push(user.id);
        } else {
          const res = await apiFetch<User>(
            `${config.api_url}${ratelApi.users.getUserByEmail(email)}`,
          );
          if (res.data) {
            user_ids.push(res.data.id);
          }
        }
      } catch (error) {
        logger.debug('Error fetching user by email:', error);
        setError(`Invalid User: ${email}`);
        setLoading(false);
      }
    }
    onSend(user_ids);
    setLoading(false);
  };
  return (
    <>
      <div className="pb-6">
        <div className="flex items-center gap-3">
          <span className="text-white text-sm">To:</span>
          <Input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onBlur={() => parseEmail()}
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                e.preventDefault();
                parseEmail();
              }
            }}
            placeholder="Search..."
            className="flex-1 bg-transparent border-none text-white placeholder:text-[#6b6b6b] focus-visible:ring-0 focus-visible:ring-offset-0 p-0"
          />
        </div>
        {error && <div className="text-red-500 text-sm mt-2">{error}</div>}
        <div className="h-px bg-[#262626] mt-4" />
      </div>

      <div className="space-y-3 max-h-80 overflow-y-auto">
        <h3 className="text-white text-sm font-medium mb-4">Invited</h3>
        {emails.map((email) => (
          <InvitedUser
            key={email}
            saved_user={emailUserMap.get(email) || null}
            removeUser={(email) => {
              setEmails((prev) => prev.filter((e) => e !== email));
              setInput((prev) => {
                const emailsInInput = prev.split(',').map((e) => e.trim());
                const updatedEmails = emailsInInput.filter((e) => e !== email);
                return updatedEmails.join(', ');
              });
            }}
            onSuccess={(user: User) => {
              addUserToMap(email, user);
            }}
            email={email}
          />
        ))}
      </div>
      <LoadablePrimaryButton
        className="w-full mt-4"
        onClick={handleSend}
        isLoading={isLoading}
      >
        Send
      </LoadablePrimaryButton>
    </>
  );
}

function LoadingIndicator() {
  return (
    <div className="flex items-center justify-center h-10 w-10">
      <svg
        className="animate-spin h-5 w-5 text-white"
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
      >
        <circle
          className="opacity-25"
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          strokeWidth="4"
        ></circle>
        <path
          className="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8v4a4 4 0 00-4 4H4z"
        ></path>
      </svg>
    </div>
  );
}

function InvitedUser({
  email,
  saved_user,
  removeUser,
  onSuccess,
}: {
  email: string;
  saved_user: User | null;
  removeUser: (email: string) => void;
  onSuccess: (user: User) => void;
}) {
  const [user, setUser] = useState<User | null>(saved_user);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<boolean>(false);
  useEffect(() => {
    async function handler() {
      try {
        const res = await apiFetch<User | null>(
          `${config.api_url}${ratelApi.users.getUserByEmail(email)}`,
        );
        if (res.data === null) {
          throw new Error('User not found');
        }
        onSuccess(res.data);
        setUser(res.data);
      } catch (error) {
        setError(true);
        logger.debug('Error fetching user by email:', error);
      } finally {
        setIsLoading(false);
      }
    }
    if (!saved_user) {
      handler();
    } else {
      setIsLoading(false);
    }
  }, [saved_user, email, onSuccess]);

  if (isLoading) {
    return (
      <div className="flex items-center gap-3">
        <LoadingIndicator />
        <span className="text-white text-sm">Loading...</span>
      </div>
    );
  }

  return (
    <div
      className={`flex items-center gap-3 group p-2 rounded-lg border ${error ? 'border-red-800' : 'border-transparent'}`}
    >
      <Avatar className="h-10 w-10">
        <AvatarImage src={user?.profile_url || '/placeholder.svg'} />
        <AvatarFallback className="bg-[#d9d9d9] text-black text-sm">
          {email.charAt(0)}
        </AvatarFallback>
      </Avatar>

      <div className="flex-1 min-w-0">
        <div className="text-white text-sm font-medium">{user?.nickname}</div>
        <div className="text-[#6b6b6b] text-xs truncate">{email}</div>
      </div>
      <button onClick={() => removeUser(email)}>
        <Remove className="[&>path]:stroke-neutral-80 hover:[&>path]:stroke-white" />
      </button>
    </div>
  );
}
