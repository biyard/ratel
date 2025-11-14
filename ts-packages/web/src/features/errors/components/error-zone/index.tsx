import { useTranslation } from 'react-i18next';
import { Error } from '../../types/errors';

export type ErrorZoneProps = {
  error?: Error;
};

export default function ErrorZone({ error }: ErrorZoneProps) {
  const { t } = useTranslation('Errors');

  if (!error) {
    return null;
  }

  return (
    <>
      <div className="p-4 mb-4 text-red-800 bg-red-50 rounded-lg border border-red-200">
        <p className="font-semibold">{t(error.title)}</p>
        <p className="mt-1 text-sm">{t(error.message)}</p>
      </div>
    </>
  );
}
