import { toast, ToastOptions } from 'react-toastify';
import CustomToast from '@/components/custom-toast/custom-toast';

const defaultOptions: ToastOptions = {
  position: 'top-right',
  autoClose: 3000,
  hideProgressBar: true,
  closeOnClick: true,
  pauseOnHover: true,
  draggable: false,
  closeButton: false, // removes default X button
  progress: undefined,
};

export const showSuccessToast = (message: string, options?: ToastOptions) =>
  toast(<CustomToast message={message} />, {
    ...defaultOptions,
    ...options,
    className: 'custom-toast-wrapper',
  });

export const showErrorToast = (message: string, options?: ToastOptions) =>
  toast.error(message, {
    ...defaultOptions,
    ...options,
  });

export const showInfoToast = (message: string, options?: ToastOptions) =>
  toast.info(message, { ...defaultOptions, ...options });

export const showWarningToast = (message: string, options?: ToastOptions) =>
  toast.warn(message, { ...defaultOptions, ...options });

export const showToast = (message: string, options?: ToastOptions) =>
  toast(message, { ...defaultOptions, ...options });
