// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

import 'dart:ui';

import 'package:ratel/components/layout/layout_service.dart' as l;
import 'package:ratel/services/rust/rust_service.dart';
import 'package:ratel/utils/biyard_navigate_observer/biyard_navigate_observer.dart';

import 'exports.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // await Firebase.initializeApp();

  // ByFirebase.init();

  RustService.init();
  ByFirebase.init();
  l.LayoutService.init();
  AnonymousService.init();
  AuthService.init();
  IndustryService.init();
  SignupService.init();
  NetworkService.init();
  FeedsService.init();
  SpaceService.init();
  UserService.init();
  DriveApi.init();

  SystemChrome.setPreferredOrientations([DeviceOrientation.portraitUp]).then((
    value,
  ) {
    initializeLogger(Config.logLevel, false);
    logger.i('App is starting...');

    runApp(MyApp());
  });
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    logger.d(
      '${Get.deviceLocale?.languageCode}, ${Get.deviceLocale?.countryCode}, ${MainLocalization.appName}',
    );

    return GetMaterialApp.router(
      scrollBehavior: MaterialScrollBehavior().copyWith(
        dragDevices: {
          PointerDeviceKind.mouse,
          PointerDeviceKind.touch,
          PointerDeviceKind.stylus,
          PointerDeviceKind.unknown,
        },
      ),
      debugShowCheckedModeBanner: false,
      defaultTransition: Transition.rightToLeft,
      theme: getThemeData(Brightness.light),
      darkTheme: getThemeData(Brightness.dark),

      // FIXME: This is a temporary fix for dark mode
      themeMode: ThemeMode.dark,
      routerDelegate: Get.createDelegate(
        navigatorObservers: [BiyardNavigatorObserver(), l.LayoutObserver()],
      ),
      translations: AppLocalization(),
      locale: const Locale('en', 'US'),
      fallbackLocale: const Locale('en', 'US'),
      title: MainLocalization.appName.tr == 'appName'
          ? 'Ratel'
          : MainLocalization.appName.tr,
      initialBinding: InitialBindings(),
      routeInformationParser: Get.createInformationParser(
        initialRoute: AppRoutes.introScreen,
      ),
      getPages: AppRoutes.pages,
    );
  }
}
