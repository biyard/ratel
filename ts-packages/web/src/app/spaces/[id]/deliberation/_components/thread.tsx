'use client';

import React from 'react';
import SpaceContents from '../../_components/space_contents';
import SpaceFiles from './space_files';
import { FileInfo } from '@/lib/api/models/feeds';
import { useDeliberationSpaceContext } from '../provider.client';
import SpaceSideMenu from './space_side_menu';

export default function ThreadPage() {
  const { isEdit, thread, setThread } = useDeliberationSpaceContext();

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
        />
        <div className="flex flex-col w-full mt-7.5 gap-2.5">
          <div className="flex flex-col md:flex-row w-full space-x-4 ">
            <div className="space-y-4 md:min-w-[70vw]">
              <SpaceContents
                isEdit={isEdit}
                htmlContents={thread.html_contents}
                setContents={(html_contents: string) => {
                  setThread({
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
                  setThread({
                    ...thread,
                    files: newFiles,
                  });
                }}
                onadd={(file: FileInfo) => {
                  setThread({
                    ...thread,
                    files: [...thread.files, file],
                  });
                }}
              />
            </div>
            <SpaceSideMenu />
          </div>

          {/* <SpaceFiles
            isEdit={isEdit}
            files={thread.files}
            onremove={(index: number) => {
              const newFiles = [...thread.files];
              newFiles.splice(index, 1);
              setThread({
                ...thread,
                files: newFiles,
              });
            }}
            onadd={(file: FileInfo) => {
              setThread({
                ...thread,
                files: [...thread.files, file],
              });
            }}
          /> */}
        </div>
      </div>
    </div>
  );
}
