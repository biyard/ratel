import { useTranslation } from 'react-i18next';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

type SpaceIncentiveDepositDialogProps = {
  open: boolean;
  depositAmount: string;
  isDepositing: boolean;
  onClose: () => void;
  onDepositAmountChange: (value: string) => void;
  onConfirmDeposit: () => void;
};

export function SpaceIncentiveDepositDialog({
  open,
  depositAmount,
  isDepositing,
  onClose,
  onDepositAmountChange,
  onConfirmDeposit,
}: SpaceIncentiveDepositDialogProps) {
  const { t } = useTranslation('SpaceIncentiveEditor');

  return (
    <Dialog
      open={open}
      onOpenChange={(nextOpen) => {
        if (!nextOpen) {
          onClose();
        }
      }}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('incentive_info_deposit_title')}</DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <div>
            <label className="text-sm text-text-secondary mb-2 block">
              {t('incentive_info_deposit_amount_label')}
            </label>
            <div className="flex flex-row w-full justify-start items-center gap-2">
              <Input
                type="number"
                min={0}
                value={depositAmount}
                onChange={(e) => onDepositAmountChange(e.target.value)}
                placeholder={t('incentive_info_deposit_amount_placeholder')}
              />
              <div className="text-sm text-text-secondary">USDT</div>
            </div>
          </div>
          <div className="flex justify-end gap-2">
            <Button
              type="button"
              variant="outline"
              size="sm"
              onClick={onClose}
              disabled={isDepositing}
            >
              {t('incentive_info_deposit_cancel')}
            </Button>
            <Button
              type="button"
              variant="rounded_primary"
              size="sm"
              onClick={onConfirmDeposit}
              disabled={isDepositing}
            >
              {isDepositing
                ? t('incentive_info_deposit_processing')
                : t('incentive_info_deposit_confirm')}
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
