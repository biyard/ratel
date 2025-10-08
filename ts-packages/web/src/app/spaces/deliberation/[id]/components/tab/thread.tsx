'use client';

import { useDeliberationSpaceByIdContext } from '../../providers.client';
import SpaceContents from '../space-contents';
import SpaceFiles from '../space-files';
import { File } from '@/lib/api/ratel/spaces/deliberation-spaces.v3';

export default function ThreadPage() {
  const { isEdit, thread, handleUpdateThread } =
    useDeliberationSpaceByIdContext();

  return (
    <div className="flex flex-row w-full gap-5">
      <div className="flex flex-col w-full">
        {/* <SpaceHeader
          isEdit={isEdit}
          title={title}
          status={status}
          userType={userType}
          proposerImage={proposerImage}
          proposerName={proposerName}
          createdAt={createdAt}
          onback={handleGoBack}
          setTitle={setTitle}
        /> */}
        <div className="flex flex-col w-full gap-2.5">
          <SpaceContents
            isEdit={isEdit}
            htmlContents={thread.html_contents}
            setContents={(html_contents: string) => {
              handleUpdateThread({
                ...thread,
                html_contents,
              });
            }}
          />
          <SpaceFiles
            isEdit={isEdit}
            files={thread.files}
            onremove={(index: number) => {
              const newFiles = [...thread.files];
              newFiles.splice(index, 1);
              handleUpdateThread({
                ...thread,
                files: newFiles,
              });
            }}
            onadd={(file: File) => {
              handleUpdateThread({
                ...thread,
                files: [...thread.files, file],
              });
            }}
          />
        </div>
      </div>
    </div>
  );
}
