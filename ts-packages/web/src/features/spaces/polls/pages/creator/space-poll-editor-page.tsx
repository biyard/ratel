import { useSpacePollEditorController } from './use-space-poll-editor-controller';

export type SpacePollEditorPageProps = {
  spacePk: string;
  pollPk: string;
};

export function SpacePollEditorPage({
  spacePk,
  pollPk,
}: SpacePollEditorPageProps) {
  // TODO: use or define hooks
  const _ctrl = useSpacePollEditorController();

  return (
    <>
      <div>SpacePollEditorPage</div>
    </>
  );
}
