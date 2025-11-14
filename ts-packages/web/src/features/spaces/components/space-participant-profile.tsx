import { Row } from '@/components/ui/row';
import { Col } from '@/components/ui/col';
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar';

export interface SpaceParticipantProfileProps {
  displayName: string;
  profileUrl: string;
  username: string;
  iconOnly?: boolean;
}

export default function SpaceParticipantProfile({
  displayName,
  profileUrl,
  username,
  iconOnly = false,
}: SpaceParticipantProfileProps) {
  // Get initials for fallback
  const initials = displayName
    .split(' ')
    .map((n) => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2);

  if (iconOnly) {
    return (
      <Avatar className="size-12">
        <AvatarImage src={profileUrl} alt={displayName} />
        <AvatarFallback>{initials}</AvatarFallback>
      </Avatar>
    );
  }

  return (
    <Row className="gap-3 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
      <Avatar className="size-12">
        <AvatarImage src={profileUrl} alt={displayName} />
        <AvatarFallback>{initials}</AvatarFallback>
      </Avatar>
      <Col className="gap-1">
        <div className="text-sm font-semibold text-gray-900 dark:text-gray-100">
          {displayName}
        </div>
        <div className="text-xs text-gray-600 dark:text-gray-400">
          @{username}
        </div>
        <div className="text-xs text-blue-600 dark:text-blue-400 mt-1">
          Participant Profile
        </div>
      </Col>
    </Row>
  );
}
