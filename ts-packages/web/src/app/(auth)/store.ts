import { create } from 'zustand';

export const Service = {
  Telegram: 'Telegram',
} as const;

export type Service = (typeof Service)[keyof typeof Service];

interface State {
  redirectUrl?: string;
  service?: Service;
  token?: string;

  username?: string;
  email?: string;
  profileImage?: string;
}

interface Action {
  clearState: () => void;
  updateSearchParams: (params: URLSearchParams) => void;
  updateUserInfo: ({
    email,
    profileImage,
    username,
  }: {
    email?: string;
    profileImage?: string;
    username?: string;
  }) => void;
}

const initialState = {};

export const useAuthStore = create<Action & State>((set) => ({
  ...initialState,

  clearState: () => set(initialState),
  updateSearchParams: (params: URLSearchParams) => {
    const redirectUrl = params.get('redirectUrl') || undefined;
    const serviceParam = params.get('service') || undefined;
    const token = params.get('token') || undefined;
    const service: Service | null =
      serviceParam && Object.values(Service).includes(serviceParam as Service)
        ? (serviceParam as Service)
        : null;
    set((state) => ({
      ...state,
      redirectUrl: redirectUrl ?? state.redirectUrl,
      service: service ?? state.service,
      token: token ?? state.token,
    }));
  },

  updateUserInfo: ({ email, profileImage, username }) => {
    set((state) => ({
      ...state,
      email: email ?? state.email,
      profileImage: profileImage ?? state.profileImage,
      username: username ?? state.username,
    }));
  },
}));
