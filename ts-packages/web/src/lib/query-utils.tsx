import {
  QueryClient,
  dehydrate,
  HydrationBoundary,
  QueryKey,
  QueryFunction,
} from '@tanstack/react-query';

export async function prefetchQuery<T>(
  queryClient: QueryClient,
  queryKey: QueryKey,
  queryFn: QueryFunction<T>,
) {
  await queryClient.prefetchQuery({
    queryKey,
    queryFn,
  });
}

interface SSRHydrationProps {
  children: React.ReactNode;
  queryClient: QueryClient;
}

export function SSRHydration({ children, queryClient }: SSRHydrationProps) {
  const dehydratedState = dehydrate(queryClient);

  return (
    <HydrationBoundary state={dehydratedState}>{children}</HydrationBoundary>
  );
}
