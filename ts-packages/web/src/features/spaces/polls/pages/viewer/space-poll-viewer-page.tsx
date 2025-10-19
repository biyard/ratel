import { useSpacePollViewerController } from './use-space-poll-viewer-controller';

export function SpacePollViewerPage({ spacePk }: { spacePk: string }) {
  // TODO: use or define hooks

  const _ctrl = useSpacePollViewerController();

  return (
    <>
      <div>SpacePollViewerPage</div>
    </>
  );
}
