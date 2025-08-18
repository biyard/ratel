'use client';

import { useCallback, useEffect } from 'react';
import { CommonEditableData, useEditCoordinatorStore } from '../../space-store';
import { useDagitStore } from '../dagit-store';
import { useUpdateSpace } from '@/hooks/use-space-by-id';
import { spaceUpdateRequest } from '@/lib/api/models/spaces';
import useDagitBySpaceId, { useDagitByIdMutation } from '@/hooks/use-dagit';

export default function Initial({ spaceId }: { spaceId: number }) {
  const { data: dAgit } = useDagitBySpaceId(spaceId);
  const { isEdit, setPageSaveHandler } = useEditCoordinatorStore();
  const { initialize, insertedArtworks, clearInsertedArtworks } =
    useDagitStore();
  const { mutateAsync: updateMutateAsync } = useUpdateSpace(spaceId);
  const {
    createArtwork: { mutateAsync: createArtworkAsync },
  } = useDagitByIdMutation(spaceId);

  const saveHandler = useCallback(
    async (commonData: Partial<CommonEditableData>) => {
      if (!dAgit) {
        return false;
      }
      const promises = insertedArtworks.map((artwork) =>
        createArtworkAsync({
          title: artwork.title,
          description: artwork.description || null,
          image: artwork.file,
        }),
      );
      try {
        await Promise.all([
          ...promises,
          updateMutateAsync(
            spaceUpdateRequest(
              commonData.html_contents ?? '',
              [],
              [],
              [],
              [],
              [],
              commonData.title,
              commonData.started_at,
              commonData.ended_at,
            ),
          ),
        ]);
        clearInsertedArtworks();
        return true;
      } catch (error) {
        console.error('Save failed:', error);
        return false;
      }
    },
    [
      createArtworkAsync,
      dAgit,
      insertedArtworks,
      updateMutateAsync,
      clearInsertedArtworks,
    ],
  );

  useEffect(() => {
    if (isEdit) {
      setPageSaveHandler(saveHandler);
    }
  }, [isEdit, setPageSaveHandler, saveHandler]);

  useEffect(() => {
    initialize(dAgit.artworks);
  }, [dAgit.artworks, initialize]);

  return null;
}
