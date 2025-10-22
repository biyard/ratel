import { State } from '@/types/state';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { useState } from 'react';
import { Space } from '@/features/spaces/types/space';
import { ListDiscussionResponse } from '../../types/list-discussion-response';
import { SpaceDiscussionResponse } from '../../types/space-discussion-response';
import useDiscussionSpace from '../../hooks/use-discussion-space';
import { logger } from '@/lib/logger';
import { usePopup } from '@/lib/contexts/popup-service';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import NewDiscussion from '../../components/modals/new_discussion';
import { useDeleteDiscussionMutation } from '../../hooks/use-delete-discussion-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';

export class SpaceDiscussionEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public discussion: ListDiscussionResponse,
    public bookmark: string | null | undefined,
    public discussions: State<SpaceDiscussionResponse[]>,
    public editing: State<boolean>,
    public popup,
    public t: TFunction<'SpaceDiscussionEditor', undefined>,

    public deleteDiscussion: ReturnType<typeof useDeleteDiscussionMutation>,
  ) {}

  handleEdit = () => {
    this.editing.set(true);
  };

  handleSave = async () => {
    this.editing.set(false);
  };

  handleDiscard = () => {
    this.editing.set(false);
  };

  handleDeleteDiscussion = async (discussionPk: string) => {
    try {
      await this.deleteDiscussion.mutateAsync({
        spacePk: this.spacePk,
        discussionPk: discussionPk,
      });

      showSuccessToast('Success to delete discussion');
    } catch {
      showErrorToast('Failed to delete discussion');
    } finally {
      this.popup.close();
    }
  };

  handleUpdateDiscussion = async (
    discussionPk: string,
    discussion: SpaceDiscussionResponse,
  ) => {
    this.popup
      .open(
        <NewDiscussion
          spacePk={this.spacePk}
          discussionPk={discussionPk}
          startedAt={discussion.started_at}
          endedAt={discussion.ended_at}
          name={discussion.name}
          description={discussion.description}
        />,
      )
      .withoutBackdropClose()
      .withTitle(this.t('select_space_type'));
  };

  handleAddDiscussion = async () => {
    logger.debug('handleAddDiscussion');
    this.popup
      .open(
        <NewDiscussion
          spacePk={this.spacePk}
          discussionPk={null}
          startedAt={Date.now()}
          endedAt={Date.now() + 60 * 60 * 1000}
          name={''}
          description={''}
        />,
      )
      .withoutBackdropClose()
      .withTitle(this.t('select_space_type'));
  };
}

export function useSpaceDiscussionEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: discussion } = useDiscussionSpace(spacePk);
  const bookmark = discussion.bookmark;
  const discussions = useState(discussion.discussions || []);
  const editing = useState(false);
  const popup = usePopup();
  const { t } = useTranslation('SpaceDiscussionEditor');

  const deleteDiscussion = useDeleteDiscussionMutation();

  //   console.log('discussion:', discussion.discussions, discussion.bookmark);

  return new SpaceDiscussionEditorController(
    spacePk,
    space,
    discussion,
    bookmark,
    new State(discussions),
    new State(editing),
    popup,
    t,

    deleteDiscussion,
  );
}
