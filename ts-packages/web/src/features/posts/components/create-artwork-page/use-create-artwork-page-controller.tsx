import { useState, useEffect, useRef } from 'react';
import { State } from '@/types/state';
import {
  CreatePostPageController,
  useCreatePostPageController,
  EditorStatus,
} from '../create-post-page/use-create-post-page-controller';
import { useCreateArtworkPageI18n } from './i18n';
import { ArtworkTrait } from '@/features/posts/types/post-artwork';
import {
  DEFAULT_REQUIRED_TRAITS,
  DEFAULT_TRAIT_CONFIGS,
} from './artwork-traits';
import { updateArtworkMetadata } from '@/features/posts/hooks/use-update-artwork-metadata-mutation';
import { logger } from '@/lib/logger';
import { SpaceType } from '@/features/spaces/types/space-type';
import { showSuccessToast } from '@/components/custom-toast/toast';
import { getPost } from '@/features/posts/hooks/use-post';

export type ArtworkTab = 'content' | 'image' | 'traits';

export class CreateArtworkPageController extends CreatePostPageController {
  public previousTraits: State<ArtworkTrait[]>;
  public isTraitsSaving: State<boolean>;
  public traitsLastSavedAt: State<Date | null>;
  public activeTab: State<ArtworkTab>;

  constructor(
    postController: CreatePostPageController,
    public traits: State<ArtworkTrait[]>,
    previousTraits: State<ArtworkTrait[]>,
    isTraitsSaving: State<boolean>,
    traitsLastSavedAt: State<Date | null>,
    activeTab: State<ArtworkTab>,
    public t: ReturnType<typeof useCreateArtworkPageI18n>,
  ) {
    super(
      postController.postPk,
      postController.teamPk,
      postController.title,
      postController.content,
      postController.image,
      postController.skipCreatingSpace,
      postController.spaceName,
      postController.spaceDescription,
      postController.status,
      postController.lastSavedAt,
      postController.isModified,
      postController.selected,
      postController.previousTitle,
      postController.previousContent,
      postController.disableSpaceSelector,
      postController.spacePk,
      postController.editorRef,
      postController.createPost,
      postController.updateDraft,
      postController.publishDraft,
      postController.updateDraftImage,
      postController.navigate,
      postController.t,
      postController.createSpace,
    );
    this.previousTraits = previousTraits;
    this.isTraitsSaving = isTraitsSaving;
    this.traitsLastSavedAt = traitsLastSavedAt;
    this.activeTab = activeTab;
  }

  handleArtworkNext = async () => {
    try {
      await this.autoSave();
      await this.autoSaveTraits();
      await this.handleCreateSpace({
        spaceType: SpaceType.Nft,
        postPk: this.postPk.get(),
      });
      showSuccessToast('Success to process request');
    } catch (error) {
      console.error('Error saving artwork:', error);
    }
  };

  // Trait management methods
  // Note: These methods do NOT modify isModified state
  // Traits are auto-saved separately from title/content
  handleTraitAdd = (trait: ArtworkTrait) => {
    const currentTraits = this.traits.get();
    this.traits.set([...currentTraits, trait]);
  };

  handleTraitUpdate = (index: number, trait: ArtworkTrait) => {
    const currentTraits = this.traits.get();
    const updatedTraits = [...currentTraits];
    updatedTraits[index] = trait;
    this.traits.set(updatedTraits);
  };

  handleTraitRemove = (index: number) => {
    const currentTraits = this.traits.get();
    const traitToRemove = currentTraits[index];

    // Prevent removing required traits
    if (DEFAULT_REQUIRED_TRAITS.includes(traitToRemove.trait_type)) {
      return;
    }

    const updatedTraits = currentTraits.filter((_, i) => i !== index);
    this.traits.set(updatedTraits);
  };

  // Override isPublishDisabled to include artwork-specific validation
  override get isPublishDisabled(): boolean {
    const currentTraits = this.traits.get();

    // Check if all required traits have values
    const requiredTraitsComplete = DEFAULT_REQUIRED_TRAITS.every(
      (traitType) => {
        const trait = currentTraits.find((t) => t.trait_type === traitType);
        return trait && String(trait.value).trim() !== '';
      },
    );

    return (
      super.isPublishDisabled || !requiredTraitsComplete || !this.image.get() // Artwork requires an image
    );
  }

  // Auto-save traits separately
  autoSaveTraits = async () => {
    const postPkValue = this.postPk.get();
    if (
      !postPkValue ||
      this.isTraitsSaving.get() ||
      this.status.get() === EditorStatus.Saving
    ) {
      return;
    }

    const currentTraits = this.traits.get();
    const previousTraits = this.previousTraits.get();

    // Check if traits have changed
    if (JSON.stringify(currentTraits) === JSON.stringify(previousTraits)) {
      logger.debug('No trait changes detected. Skipping auto-save.');
      return;
    }

    // Only save if there are traits to save
    if (currentTraits.length === 0) {
      logger.debug('No traits to save. Skipping auto-save.');
      return;
    }

    this.isTraitsSaving.set(true);
    try {
      await updateArtworkMetadata(postPkValue, currentTraits);
      this.previousTraits.set(currentTraits);
      this.traitsLastSavedAt.set(new Date());
      logger.debug('Traits auto-saved successfully');
    } catch (error) {
      logger.error('Traits auto-save failed:', error);
    } finally {
      this.isTraitsSaving.set(false);
    }
  };

  get isTraitsModified(): boolean {
    const currentTraits = this.traits.get();
    const previousTraits = this.previousTraits.get();
    return JSON.stringify(currentTraits) !== JSON.stringify(previousTraits);
  }
}

// Initialize default required traits
const initializeDefaultTraits = (): ArtworkTrait[] => {
  return DEFAULT_REQUIRED_TRAITS.map((traitType) => {
    const config = DEFAULT_TRAIT_CONFIGS[traitType];
    return {
      trait_type: traitType,
      value: String(config.default_value),
      display_type: config.display_type,
    };
  });
};

export function useCreateArtworkPageController() {
  const postController = useCreatePostPageController();
  const t = useCreateArtworkPageI18n();

  // Artwork-specific state
  const traits = useState<ArtworkTrait[]>(initializeDefaultTraits());
  const previousTraits = useState<ArtworkTrait[]>(initializeDefaultTraits());
  const isTraitsSaving = useState<boolean>(false);
  const traitsLastSavedAt = useState<Date | null>(null);
  const activeTab = useState<ArtworkTab>('content');

  const controller = new CreateArtworkPageController(
    postController,
    new State(traits),
    new State(previousTraits),
    new State(isTraitsSaving),
    new State(traitsLastSavedAt),
    new State(activeTab),
    t,
  );

  // Load artwork metadata when postPk is available
  const initializedMetadataRef = useRef(false);
  const postPkValue = postController.postPk.get();

  useEffect(() => {
    const loadArtworkMetadata = async () => {
      if (!postPkValue || initializedMetadataRef.current) {
        return;
      }

      initializedMetadataRef.current = true;

      try {
        const postData = await getPost(postPkValue);

        // Load artwork metadata if it exists
        if (postData.artwork_metadata && postData.artwork_metadata.length > 0) {
          logger.debug('Loading artwork metadata:', postData.artwork_metadata);
          controller.traits.set(postData.artwork_metadata);
          controller.previousTraits.set(postData.artwork_metadata);
        }
      } catch (error) {
        logger.error('Failed to load artwork metadata:', error);
      }
    };

    loadArtworkMetadata();
  }, [postPkValue, controller]);

  // Auto-save traits when they change
  const autoSaveTimerRef = useRef<NodeJS.Timeout | null>(null);
  const currentTraits = traits[0];

  useEffect(() => {
    // Clear existing timer
    if (autoSaveTimerRef.current) {
      clearTimeout(autoSaveTimerRef.current);
    }

    // Set new timer for auto-save (debounced by 2 seconds)
    autoSaveTimerRef.current = setTimeout(() => {
      void controller.autoSaveTraits();
    }, 2000);

    // Cleanup on unmount
    return () => {
      if (autoSaveTimerRef.current) {
        clearTimeout(autoSaveTimerRef.current);
      }
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [currentTraits]); // Watch traits changes

  return controller;
}
