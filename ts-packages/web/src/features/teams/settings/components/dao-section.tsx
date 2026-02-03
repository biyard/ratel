import { Button } from '@/components/ui/button';
import { Row } from '@/components/ui/row';
import { TeamSettingsI18n } from '../i18n';

interface DaoSectionProps {
  daoAddress: string | null;
  isConnectingWallet: boolean;
  onActivate: () => void;
  i18n: TeamSettingsI18n;
}

export function DaoSection({
  daoAddress,
  isConnectingWallet,
  onActivate,
  i18n,
}: DaoSectionProps) {
  return (
    <Row className="items-center">
      <label className="w-35 font-bold text-text-primary">
        {i18n.dao_address}
      </label>
      {daoAddress ? (
        <span className="text-sm text-text-primary break-all">
          {daoAddress}
        </span>
      ) : (
        <Button
          variant="primary"
          onClick={onActivate}
          disabled={isConnectingWallet}
          data-pw="team-dao-activate-button"
        >
          {isConnectingWallet ? i18n.activating_dao : i18n.activate_dao}
        </Button>
      )}
    </Row>
  );
}
