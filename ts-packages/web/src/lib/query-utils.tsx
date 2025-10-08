import {
  QueryClient,
  dehydrate,
  HydrationBoundary,
} from '@tanstack/react-query';

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
