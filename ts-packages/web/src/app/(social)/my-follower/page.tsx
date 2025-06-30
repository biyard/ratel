import { RelationType } from '@/types/relation-type';
import PageClient from './page.client';

type Props = {
  searchParams: Promise<{ type?: RelationType }>;
};

export default async function Page({ searchParams }: Props) {
  const { type } = await searchParams;
  const selectedType = type || RelationType.FOLLOWER;

  return <PageClient type={selectedType} />;
}
