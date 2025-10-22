import Card from '@/components/card';
import { AddDiscussionButton } from './add-discussion-button';
import { TFunction } from 'i18next';
import { SpaceDiscussionResponse } from '../../types/space-discussion-response';
import React from 'react';
import { DiscussionRoom } from './discussion-room';

export type DiscussionEditorProps = {
  t: TFunction<'SpaceDiscussionEditor', undefined>;
  discussions: SpaceDiscussionResponse[];
  bookmark: string | null | undefined;
  canEdit: boolean;
  isPublished: boolean;
  onadd: () => void;
  ondelete: (discussionPk: string) => void;
  onupdate: (discussionPk: string, discussion: SpaceDiscussionResponse) => void;
};

export default function DiscussionEditor({
  t,
  discussions,
  canEdit,
  isPublished,
  onadd,
  ondelete,
  onupdate,
}: DiscussionEditorProps) {
  return (
    <>
      <Card className="flex flex-col gap-3">
        <div className="flex flex-col w-full gap-5">
          <div className="flex flex-row w-full justify-between items-center">
            <div className="font-bold text-text-primary text-[15px]/[20px]">
              {t('discussions')}
            </div>

            {canEdit && <AddDiscussionButton onadd={onadd} />}
          </div>
        </div>

        <div className="flex flex-col w-full gap-2.5">
          {discussions.map((discussion, index) => (
            <React.Fragment key={index}>
              <DiscussionRoom
                discussion={discussion}
                canEdit={canEdit}
                isMember={discussion.is_member}
                isPublished={isPublished}
                onclick={() => {}}
                onupdate={() => {
                  onupdate(discussion.pk, discussion);
                }}
                ondelete={() => {
                  ondelete(discussion.pk);
                }}
              />
              {index !== discussions.length - 1 ? (
                <div className=" w-full h-0.25 gap-1 bg-divider" />
              ) : (
                <></>
              )}
            </React.Fragment>
          ))}
        </div>
      </Card>
    </>
  );
}
