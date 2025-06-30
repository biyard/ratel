import axios from 'axios';
import { logger } from '../logger';
import { config } from '@/config';

const BASE_URL = 'https://www.googleapis.com/drive/v3';
const UPLOAD_URL =
  'https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart';

export interface DriveFile {
  id: string;
  name: string;
}

export const listFiles = async (
  key: string,
  accessToken: string,
): Promise<DriveFile[]> => {
  const q = encodeURIComponent(`name='${key}'`);
  const response = await axios.get(`${BASE_URL}/files?q=${q}`, {
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
    params: {
      spaces: 'appDataFolder',
    },
  });

  logger.debug('Responses: ', response);

  return response.data.files;
};

export const uploadFile = async (
  accessToken: string,
  content: string,
): Promise<DriveFile> => {
  const metadata = {
    name: config.env,
    parents: ['appDataFolder'],
  };

  const formData = new FormData();

  formData.append(
    'metadata',
    new Blob([JSON.stringify(metadata)], { type: 'application/json' }),
  );
  formData.append('file', new Blob([content], { type: 'text/plain' }));

  const response = await axios.post(UPLOAD_URL, formData, {
    headers: {
      Authorization: `Bearer ${accessToken}`,
      'Content-Type': 'multipart/related',
    },
  });

  return response.data;
};

export const getFile = async (
  accessToken: string,
  fileId: string,
): Promise<string> => {
  const response = await axios.get(`${BASE_URL}/files/${fileId}`, {
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
    params: {
      alt: 'media',
    },
  });

  return response.data;
};
