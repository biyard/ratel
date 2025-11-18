import { Unlock2 } from '@/assets/icons/security';
import { ArrowLeft } from '@/assets/icons/arrows';
import { Save } from '@/assets/icons/other-devices';
import { ArrowUp } from '@/assets/icons/game';
import { Edit1 } from '@/assets/icons/edit';
import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';

function ButtonWithIcon({
  children,
  disabled = false,
  onClick,
}: {
  children: React.ReactNode;
  disabled?: boolean;
  onClick: () => void;
}) {
  return (
    <Button
      variant="default"
      className="font-bold text-zinc-900 text-sm [&_svg]:!size-5"
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </Button>
  );
}

function EditButton({ onClick }: { onClick: () => void }) {
  const { t } = useTranslation('SprintSpace');
  return (
    <ButtonWithIcon onClick={onClick}>
      <Edit1 className="stroke-neutral-500 [&>path]:stroke-2 !size-5" />
      <div>{t('edit')}</div>
    </ButtonWithIcon>
  );
}

function SaveButton({
  onClick,
  disabled,
}: {
  onClick: () => void;
  disabled?: boolean;
}) {
  const { t } = useTranslation('SprintSpace');
  return (
    <ButtonWithIcon onClick={onClick} disabled={disabled}>
      <Save className="stroke-neutral-500 [&>path]:stroke-2 !size-5" />
      <div>{t('save')}</div>
    </ButtonWithIcon>
  );
}

// function JoinSpaceButton({ target }: { target: string }) {
//   const { t } = useTranslation('SprintSpace');
//   return (
//     <Link to={target}>
//       <Button variant="default" className="px-3 py-2 [&>svg]:!size-5">
//         <Lock2 className="stroke-neutral-500 [&>path]:stroke-2 !size-5" />
//         <div className="font-bold text-zinc-900 text-sm">{t('join_space')}</div>
//       </Button>
//     </Link>
//   );
// }

// function CreateSpaceButton({ onClick }: { onClick: () => void }) {
//   const { t } = useTranslation('SprintSpace');
//   return (
//     <ButtonWithIcon onClick={onClick}>
//       <Palace className="stroke-neutral-500 [&>path]:stroke-2 !size-5" />
//       <div>{t('create_space')}</div>
//     </ButtonWithIcon>
//   );
// }

function PublishSpaceButton({ onClick }: { onClick: () => void }) {
  const { t } = useTranslation('SprintSpace');
  return (
    <ButtonWithIcon onClick={onClick} data-testid="publish-space-button">
      <ArrowUp className="stroke-neutral-500 [&>path]:stroke-2 " />
      <div>{t('publish')}</div>
    </ButtonWithIcon>
  );
}

function MakePublicButton({ onClick }: { onClick: () => void }) {
  const { t } = useTranslation('SprintSpace');
  return (
    <ButtonWithIcon onClick={onClick}>
      <Unlock2 className="stroke-neutral-500 [&>path]:stroke-2" />
      <div>{t('make_public')}</div>
    </ButtonWithIcon>
  );
}

function BackButton({ onClick }: { onClick: () => void }) {
  return (
    <div className="cursor-pointer w-fit h-fit" onClick={onClick}>
      <ArrowLeft className="size-6 [&>path]:stroke-foreground" />
    </div>
  );
}

export {
  EditButton,
  SaveButton,
  BackButton,
  PublishSpaceButton,
  MakePublicButton,
};
