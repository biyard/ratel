import DeliberationSpacePage from './components/main';

export default async function Page({}: { params: Promise<{ id: string }> }) {
  return <DeliberationSpacePage />;
}
