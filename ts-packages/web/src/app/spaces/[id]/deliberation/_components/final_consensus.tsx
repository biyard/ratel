'use client';

import React from 'react';
import SpaceContents from '../../_components/space_contents';
import SpaceFiles from './space_files';
import { FileInfo } from '@/lib/api/models/feeds';
import { useDeliberationSpaceContext } from '../provider.client';
import SpaceSideMenu from './space_side_menu';

export default function FinalConsensusPage() {
  const { draft, setDraft, isEdit } = useDeliberationSpaceContext();

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
        <SpaceHeader
          isEdit={isEdit}
          title={title}
          status={status}
          userType={userType}
          proposerImage={proposerImage}
          proposerName={proposerName}
          createdAt={createdAt}
          onback={onback}
          setTitle={(title: string) => {
            setTitle(title);
          }}
        />

        <div className="flex flex-col md:flex-row md:space-x-4">
          <div className="flex flex-col w-full mt-7.5 gap-2.5">
            <SpaceContents
              isEdit={isEdit}
              htmlContents={contents.html_contents}
              setContents={(html_contents: string) => {
                setDraft({
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
                setDraft({
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
                setDraft({
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

          <div className="mt-7.5">
            <SpaceSideMenu />
          </div>
        </div>
      </div>
    </div>
  );
}
