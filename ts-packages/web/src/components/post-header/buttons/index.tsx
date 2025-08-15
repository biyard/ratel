import { Unlock2, Lock2 } from '@/assets/icons/security';
import { ArrowLeft } from '@/assets/icons/arrows';
import { Save } from '@/assets/icons/other-devices';
import { ArrowUp } from '@/assets/icons/game';
import { Edit1 } from '@/assets/icons/edit';
import { Palace } from '@/assets/icons/home';
import { Button } from '@/components/ui/button';
import Link from 'next/link';

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
  return (
    <ButtonWithIcon onClick={onClick}>
      <Edit1 className="stroke-neutral-500 [&>path]:stroke-2 !size-5" />
      <div>Edit</div>
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
  return (
    <ButtonWithIcon onClick={onClick} disabled={disabled}>
      <Save className="stroke-neutral-500 [&>path]:stroke-2 !size-5" />
      <div>Save</div>
    </ButtonWithIcon>
  );
}

function JoinSpaceButton({ target }: { target: string }) {
  return (
    <Link href={target}>
      <Button variant="default" className="px-3 py-2 [&>svg]:!size-5">
        <Lock2 className="stroke-neutral-500 [&>path]:stroke-2 !size-5" />
        <div className="font-bold text-zinc-900 text-sm">Join Space</div>
      </Button>
    </Link>
  );
}

function CreateSpaceButton({ onClick }: { onClick: () => void }) {
  return (
    <ButtonWithIcon onClick={onClick}>
      <Palace className="stroke-neutral-500 [&>path]:stroke-2 !size-5" />
      <div>Create Space</div>
    </ButtonWithIcon>
  );
}

function PublishSpaceButton({ onClick }: { onClick: () => void }) {
  return (
    <ButtonWithIcon onClick={onClick}>
      <ArrowUp className="stroke-neutral-500 [&>path]:stroke-2 " />
      <div>Publish</div>
    </ButtonWithIcon>
  );
}

function MakePublicButton({ onClick }: { onClick: () => void }) {
  return (
    <ButtonWithIcon onClick={onClick}>
      <Unlock2 className="stroke-neutral-500 [&>path]:stroke-2" />
      <div>Make Public</div>
    </ButtonWithIcon>
  );
}

function BackButton({ onClick }: { onClick: () => void }) {
  return (
    <div className="cursor-pointer w-fit h-fit" onClick={onClick}>
      <ArrowLeft className="size-6 [&>path]:stroke-white" />
    </div>
  );
}

export {
  EditButton,
  SaveButton,
  JoinSpaceButton,
  CreateSpaceButton,
  BackButton,
  PublishSpaceButton,
  MakePublicButton,
};
