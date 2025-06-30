import PageClient from './page.client';

export interface TeamLayoutProps {
  params: Promise<{ username: string }>;
}

export default async function Page({ params }: TeamLayoutProps) {
  const { username } = await params;
  return <PageClient username={username} />;
}
