// import { initData, useSignal } from '@telegram-apps/sdk-react';

// function RootInner({ children }: { children?: ReactNode }) {
//   // const initDataUser = useSignal(initData.user);

//   // // Set the user locale.
//   // useEffect(() => {
//   //   console.log('initDataUser:', initDataUser);
//   // }, [initDataUser]);

//   return <>{children}</>;
// }

export default async function Layout({
  children,
}: {
  children: React.ReactNode;
}) {
  //https://0.0.0.0:3000/telegram?tgWebAppStartParam=123123123
  // const { id } = await params;

  return <div className="absolute left-0 top-0 inset-0">{children}</div>;
}
