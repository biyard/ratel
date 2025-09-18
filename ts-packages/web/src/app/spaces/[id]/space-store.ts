import { create } from 'zustand';
import { Space } from '@/lib/api/models/spaces';

type PageSaveHandler = (
  commonData: Partial<CommonEditableData>,
) => Promise<boolean>;

export type CommonEditableData = Pick<
  Space,
  'title' | 'html_contents' | 'started_at' | 'ended_at'
>;

type State = {
  isEdit: boolean;
  isModified: boolean;
  commonData: Partial<CommonEditableData> | null;
  pageSaveHandler: PageSaveHandler | null;
  spacePublishValidatorImpl: () => boolean;
};

type Actions = {
  startEditing: (initialData: Partial<CommonEditableData>) => void;
  stopEditing: () => void;
  setModified: () => void;
  updateCommonData: (data: Partial<CommonEditableData>) => void;
  triggerGlobalSave: () => Promise<void>;
  setPageSaveHandler: (handler: PageSaveHandler) => void;
  spacePublishValidator: () => boolean;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  setSpacePublishValidator: (handler: any) => void;
};

const initialState: State = {
  isEdit: false,
  isModified: false,
  commonData: null,
  pageSaveHandler: null,
  spacePublishValidatorImpl: () => true,
};

export const useEditCoordinatorStore = create<State & Actions>((set, get) => ({
  ...initialState,
  startEditing: (initialData) => {
    set({
      isEdit: true,
      isModified: false,
      commonData: initialData,
      pageSaveHandler: null,
    });
  },
  stopEditing: () => set({ ...initialState }),
  setModified: () => set({ isModified: true }),
  updateCommonData: (data) => {
    set((state) => ({
      commonData: state.commonData ? { ...state.commonData, ...data } : data,
      isModified: true,
    }));
  },
  setPageSaveHandler: (handler) => set({ pageSaveHandler: handler }),
  triggerGlobalSave: async () => {
    const { pageSaveHandler, commonData, isModified } = get();
    if (!isModified || !pageSaveHandler) return;

    const success = await pageSaveHandler(commonData ?? {});
    if (success) {
      get().stopEditing();
    }
  },
  spacePublishValidator: () => {
    const { spacePublishValidatorImpl } = get();
    return spacePublishValidatorImpl();
  },

  setSpacePublishValidator: (handler) =>
    set({ spacePublishValidator: handler }),
}));
