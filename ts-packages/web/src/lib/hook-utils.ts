import { getQueryClient } from '@/providers/getQueryClient';
import { InfiniteData } from '@tanstack/react-query';

export type Rollbackable<T> = T & {
  rollback: () => void;
};

export async function removeQueries<T>({
  queryKey,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
}: any): Promise<Rollbackable<T> | undefined> {
  const queryClient = getQueryClient();
  await queryClient.cancelQueries({ queryKey });

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const rollbackData: any = queryClient.getQueryData<T>(queryKey);

  queryClient.removeQueries({ queryKey });

  if (rollbackData) {
    rollbackData!.rollback = () => {
      queryClient.setQueryData(queryKey, rollbackData);
    };
  }

  return rollbackData;
}

export async function optimisticUpdate<T>(
  {
    queryKey,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  }: any,
  updater: (oldData: T | undefined) => T | undefined,
): Promise<Rollbackable<[readonly unknown[], T | undefined][]>> {
  const queryClient = getQueryClient();
  await queryClient.cancelQueries({ queryKey });

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const rollbackData: any = queryClient.getQueriesData<T>({ queryKey });

  queryClient.setQueriesData<T>({ queryKey }, updater);

  if (rollbackData) {
    rollbackData!.rollback = () => {
      queryClient.setQueryData(queryKey, rollbackData);
    };
  }

  return rollbackData;
}

export async function optimisticListUpdate<T>(
  {
    queryKey,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  }: any,
  updater: (oldData: T) => T | undefined,
): Promise<
  Rollbackable<readonly [readonly unknown[], InfiniteData<T[]> | undefined][]>
> {
  const queryClient = getQueryClient();
  await queryClient.cancelQueries({ queryKey });

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const rollbackData: any = queryClient.getQueriesData<InfiniteData<T[]>>({
    queryKey,
  });

  queryClient.setQueriesData<InfiniteData<T[]>>({ queryKey }, (oldData) => {
    if (!oldData) return oldData;
    const newPages = oldData.pages.map((v) => {
      return v.map(updater).filter((page): page is T => page !== undefined);
    });
    return { ...oldData, pages: newPages };
  });

  if (rollbackData) {
    rollbackData.rollback = () => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      rollbackData.forEach(([key, data]: any) => {
        queryClient.setQueryData(key, data);
      });
    };
  }

  return rollbackData;
}
