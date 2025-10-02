'use client';

import React from 'react';
import { useDeliberationSpaceByIdContext } from '../../providers.client';
import SpaceContents from '../space-contents';
import SpaceFiles from '../space-files';
import { File } from '@/lib/api/ratel/spaces/deliberation-spaces.v3';

export default function FinalConsensusPage() {
  const { draft, handleUpdateDraft, isEdit, title } =
    useDeliberationSpaceByIdContext();

  const contents = {
    title: title,
    html_contents: draft.drafts.html_contents,
    files: draft.drafts.files,
  };

  return (
    <div className="flex flex-row w-full gap-5">
      <div className="flex flex-col w-full">
        <div className="flex flex-col w-full gap-2.5">
          <SpaceContents
            isEdit={isEdit}
            htmlContents={contents.html_contents}
            setContents={(html_contents: string) => {
              handleUpdateDraft({
                ...draft,
                drafts: {
                  ...contents,
                  html_contents,
                },
              });
            }}
          />
          <SpaceFiles
            isEdit={isEdit}
            files={contents.files}
            onremove={(index: number) => {
              const newFiles = [...contents.files];
              newFiles.splice(index, 1);
              handleUpdateDraft({
                ...draft,
                drafts: {
                  ...contents,
                  files: newFiles,
                },
              });
            }}
            onadd={(file: File) => {
              handleUpdateDraft({
                ...draft,
                drafts: {
                  ...contents,
                  files: [...contents.files, file],
                },
              });
            }}
          />
        </div>
      </div>
    </div>
  );
}
