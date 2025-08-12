import { toast, ToastOptions } from 'react-toastify';

const defaultOptions: ToastOptions = {
  position: 'top-right',
  autoClose: 3000,
  hideProgressBar: false,
  closeOnClick: true,
  pauseOnHover: true,
  draggable: true,
  progress: undefined,
};

export const showSuccessToast = (message: string, options?: ToastOptions) =>
  toast.success(message, { ...defaultOptions, ...options });

export const showErrorToast = (message: string, options?: ToastOptions) =>
  toast.error(message, { ...defaultOptions, ...options });

export const showInfoToast = (message: string, options?: ToastOptions) =>
  toast.info(message, { ...defaultOptions, ...options });

export const showWarningToast = (message: string, options?: ToastOptions) =>
  toast.warn(message, { ...defaultOptions, ...options });

export const showToast = (message: string, options?: ToastOptions) =>
  toast(message, { ...defaultOptions, ...options });
