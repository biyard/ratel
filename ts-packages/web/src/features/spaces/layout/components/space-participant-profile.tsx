import { Row } from '@/components/ui/row';
import { Col } from '@/components/ui/col';
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar';

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
  // Get initials for fallback
  const initials = displayName
    .split(' ')
    .map((n) => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2);

  return (
    <Row className="p-3 flex" data-testid="space-participant-profile">
      <Avatar className="size-12">
        <AvatarImage src={profileUrl} alt={displayName} />
        <AvatarFallback>{initials}</AvatarFallback>
      </Avatar>
      <Col className="gap-1 flex-1">
        <div className="text-base font-semibold text-gray-900 dark:text-gray-100">
          {displayName}
        </div>
        <div className="text-sm text-gray-600 dark:text-gray-400">
          @{username}
        </div>
      </Col>
    </Row>
  );
}
