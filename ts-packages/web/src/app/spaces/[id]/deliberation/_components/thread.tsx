'use client';

import React from 'react';
import SpaceContents from '../../_components/space_contents';
import SpaceFiles from './space_files';
import { FileInfo } from '@/lib/api/models/feeds';
import { useDeliberationSpaceContext } from '../provider.client';

export default function ThreadPage() {
  const { isEdit, thread, handleUpdateThread } = useDeliberationSpaceContext();

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
            onadd={(file: FileInfo) => {
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
