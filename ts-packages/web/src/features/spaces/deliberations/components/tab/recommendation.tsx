import { useSpaceHeaderStore } from '@/app/spaces/_components/header/store';
import SpaceContents from '../space-contents';
import SpaceFiles from '../space-files';
import { TFunction } from 'i18next';
import { FinalConsensus } from '../../types/final-consensus-type';
import { File } from '../../utils/deliberation.spaces.v3';

export type ThreadPageProps = {
  t: TFunction<'DeliberationSpace', undefined>;
  draft: FinalConsensus;
  setDraft: (draft: FinalConsensus) => void;
};

export default function FinalConsensusPage({
  draft,
  setDraft,
}: {
  draft: FinalConsensus;
  setDraft: (draft: FinalConsensus) => void;
}) {
  const store = useSpaceHeaderStore();
  const isEdit = store.isEditingMode;

  return (
    <div className="flex flex-row w-full gap-5">
      <div className="flex flex-col w-full">
        <div className="flex flex-col w-full gap-2.5">
          <SpaceContents
            isEdit={isEdit}
            htmlContents={draft.drafts.html_contents}
            setContents={(html_contents: string) => {
              setDraft({
                ...draft,
                drafts: {
                  ...draft,
                  html_contents,
                  files: draft.drafts.files,
                },
              });

              store.onModifyContent();
            }}
          />
          <SpaceFiles
            isEdit={isEdit}
            files={draft.drafts.files}
            onremove={(index: number) => {
              const newFiles = [...draft.drafts.files];
              newFiles.splice(index, 1);
              setDraft({
                ...draft,
                drafts: {
                  ...draft,
                  html_contents: draft.drafts.html_contents,
                  files: newFiles,
                },
              });

              store.onModifyContent();
            }}
            onadd={(file: File) => {
              setDraft({
                ...draft,
                drafts: {
                  ...draft,
                  html_contents: draft.drafts.html_contents,
                  files: [...draft.drafts.files, file],
                },
              });

              store.onModifyContent();
            }}
          />
        </div>
      </div>
    </div>
  );
}
