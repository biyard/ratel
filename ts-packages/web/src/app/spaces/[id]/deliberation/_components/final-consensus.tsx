'use client';

import React from 'react';
import SpaceContents from '../../_components/space_contents';
import SpaceFiles from './space-files';
import { FileInfo } from '@/lib/api/models/feeds';
import { useDeliberationSpaceContext } from '../provider.client';

export default function FinalConsensusPage() {
  const { draft, handleUpdateDraft, isEdit } = useDeliberationSpaceContext();

  const contents =
    draft.drafts && draft.drafts.length != 0
      ? draft.drafts[0]
      : {
          title: '',
          html_contents: '',
          files: [],
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
                drafts: [
                  {
                    ...contents,
                    html_contents,
                  },
                ],
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
                drafts: [
                  {
                    ...contents,
                    files: newFiles,
                  },
                ],
              });
            }}
            onadd={(file: FileInfo) => {
              handleUpdateDraft({
                ...draft,
                drafts: [
                  {
                    ...contents,
                    files: [...contents.files, file],
                  },
                ],
              });
            }}
          />
        </div>
      </div>
    </div>
  );
}
