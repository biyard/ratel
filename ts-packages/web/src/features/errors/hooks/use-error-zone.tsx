import { useState, useCallback } from 'react';
import { Error } from '../types/errors';
import ErrorZoneComponent from '../components/error-zone';

export function useErrorZone() {
  const [error, setError] = useState<Error | undefined>(undefined);
  const removeError = useCallback(() => setError(undefined), []);

  // Return a bound ErrorZone component that receives the error state
  const ErrorZone = useCallback(
    () => <ErrorZoneComponent error={error} />,
    [error],
  );

  return { error, setError, removeError, ErrorZone };
}
