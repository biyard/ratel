import { Row } from '@/components/ui/row';
import { Col } from '@/components/ui/col';
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar';
import { Extra2 } from '@/components/icons';
import { useTheme } from '@/hooks/use-theme';
import { useAuth } from '@/lib/contexts/auth-context';
import Switch from '@/components/switch/switch';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useSpaceLayoutI18n } from '../space-layout-i18n';

export interface SpaceParticipantProfileProps {
  displayName: string;
  profileUrl: string;
  username: string;
}

export default function SpaceParticipantProfile({
  displayName,
  profileUrl,
  username,
}: SpaceParticipantProfileProps) {
  const i18n = useSpaceLayoutI18n();

  const { theme, setTheme } = useTheme();
  const { logout } = useAuth();

  // Get initials for fallback
  const initials = displayName
    .split(' ')
    .map((n) => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2);

  return (
    <Row
      className="flex justify-center items-center"
      data-testid="space-participant-profile"
    >
      <Avatar className="size-12">
        <AvatarImage src={profileUrl} alt={displayName} />
        <AvatarFallback>{initials}</AvatarFallback>
      </Avatar>
      <Col className="gap-1 flex-1">
        <div className="text-base font-semibold text-text-primary dark:text-gray-100">
          {displayName}
        </div>
        <div className="text-sm text-text-secondary">@{username}</div>
      </Col>

      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Extra2 />
        </DropdownMenuTrigger>

        <DropdownMenuContent
          side="top"
          align="end"
          className="w-[250px] bg-bg rounded-lg p-4 border-none"
        >
          {/* Actions Section */}
          <DropdownMenuGroup className="flex flex-col gap-3">
            <DropdownMenuItem className="flex items-center justify-between px-0 hover:bg-transparent focus:bg-transparent">
              <span className="text-sm font-bold text-text-primary">
                {i18n.profile_dark_theme}
              </span>
              <Switch
                checked={theme === 'dark'}
                onChange={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
              />
            </DropdownMenuItem>

            <DropdownMenuItem
              onClick={logout}
              className="flex items-center gap-1 cursor-pointer px-0 hover:bg-transparent focus:bg-transparent"
            >
              <span className="text-sm font-bold text-text-primary">
                {i18n.profile_sign_out}
              </span>
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    </Row>
  );
}
