import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import Card from '@/components/card';
import { Check, RemoveCircle, Warning } from '@/assets/icons/validations';
import { LayoutAction } from '../use-space-layout-controller';

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

export function ParticipationCard({
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
    <Card variant="outlined" rounded="default" className="gap-2.5 bg-divider">
      <div className="flex flex-col gap-1 w-full">
        <h3 className="font-bold text-[15px] leading-[18px] tracking-[-0.16px] text-text-primary">
          {title}
        </h3>
        <p className="font-medium text-[13px] leading-5 text-neutral-400">
          {description}
        </p>
      </div>

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
        >
          {actions[0].label}
        </Button>
        <Button
          variant="outline"
          className="w-full"
          onClick={actions[1].onClick}
        >
          {actions[1].label}
        </Button>
      </div>
    </Card>
  );
}
