import { useSpaceHeaderStore } from '@/app/spaces/_components/header/store';
import SpaceContents from '../space-contents';
import SpaceFiles from '../space-files';
import { File } from '../../utils/deliberation.spaces.v3';
import { TFunction } from 'i18next';
import { Thread } from '../../types/thread-type';

export type ThreadPageProps = {
  t: TFunction<'DeliberationSpace', undefined>;
  thread: Thread;
  setThread: (thread: Thread) => void;
};

export default function ThreadPage({ thread, setThread }: ThreadPageProps) {
  const store = useSpaceHeaderStore();
  const isEdit = store.isEditingMode;

  return (
    <div className="flex flex-row w-full gap-5">
      <div className="flex flex-col w-full">
        <div className="flex flex-col w-full gap-2.5">
          <SpaceContents
            isEdit={isEdit}
            htmlContents={thread.html_contents}
            setContents={(html_contents: string) => {
              setThread({
                ...thread,
                html_contents,
              });

              store.onModifyContent();
            }}
          />
          <SpaceFiles
            isEdit={isEdit}
            files={thread.files}
            onremove={(index: number) => {
              const newFiles = [...thread.files];
              newFiles.splice(index, 1);
              setThread({
                ...thread,
                files: newFiles,
              });

              store.onModifyContent();
            }}
            onadd={(file: File) => {
              setThread({
                ...thread,
                files: [...thread.files, file],
              });

              store.onModifyContent();
            }}
          />
        </div>
      </div>
    </div>
  );
}
