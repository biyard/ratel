import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import Card from '@/components/card';
import { Check, RemoveCircle, Warning } from '@/assets/icons/validations';
import { LayoutAction } from '../use-space-layout-controller';
import { CardSkeleton } from './admin-action-card';

enum CredentialStatus {
  Active = 'active',
  Inactive = 'inactive',
  NotMatch = 'not_match',
}
interface CredentialTagProps {
  label: string;
  status: CredentialStatus;
}

function CredentialTag({ label, status }: CredentialTagProps) {
  return (
    <div
      className={cn(
        'flex items-center gap-1 px-2 py-1 rounded-full border h-[25px]',
        status === CredentialStatus.NotMatch &&
          'bg-red-500/5 border-red-500 text-text-primary',
        status === CredentialStatus.Active &&
          'bg-green-500/5 border-green-500 text-text-primary',
        status === CredentialStatus.Inactive &&
          'bg-gray-500/5 border-gray-500 text-text-primary',
      )}
    >
      {status === CredentialStatus.Inactive && (
        <Warning className="size-4 fill-red-500" />
      )}
      {status === CredentialStatus.NotMatch && (
        <RemoveCircle className="size-4 fill-red-500" />
      )}
      {status === CredentialStatus.Active && (
        <Check className="size-4 fill-green-500" />
      )}
      <span className="font-bold text-xs">{label}</span>
    </div>
  );
}

interface ParticipationCardProps {
  title?: string;
  description?: string;
  verifiedCredentials?: CredentialTagProps[];
  actions: LayoutAction[];
}

export default function ViewerActionCard({
  title = 'Participation',
  description = 'You can read everything, but posting, voting and commenting require verification.',
  verifiedCredentials = [
    { label: 'Age', status: CredentialStatus.Inactive },
    { label: 'Country', status: CredentialStatus.Active },
    { label: 'Univercity', status: CredentialStatus.NotMatch },
  ],
  actions,
}: ParticipationCardProps) {
  return (
    <CardSkeleton title={title} description={description}>
      {/* Attribute Tags */}
      {verifiedCredentials && verifiedCredentials.length > 0 && (
        <div className="flex gap-1 items-center flex-wrap">
          {verifiedCredentials.map((credential) => (
            <CredentialTag
              key={credential.label}
              label={credential.label}
              status={credential.status}
            />
          ))}
        </div>
      )}

      {/* Action Buttons */}
      <div className="flex flex-col gap-2.5 w-full">
        <Button
          variant="rounded_primary"
          className="w-full"
          onClick={actions[0].onClick}
          data-testid="viewer-participate-button"
        >
          {actions[0].label}
        </Button>
        <Button
          variant="rounded_secondary"
          className="w-full"
          onClick={actions[1].onClick}
          data-testid="viewer-credentials-button"
        >
          {actions[1].label}
        </Button>
      </div>
    </CardSkeleton>
  );
}
