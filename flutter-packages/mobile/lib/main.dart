import 'dart:ui';

import 'package:firebase_core/firebase_core.dart';
import 'package:firebase_messaging/firebase_messaging.dart';
import 'package:ratel/components/layout/layout_service.dart' as l;
import 'package:ratel/firebase_options.dart';
import 'package:ratel/services/rust/rust_service.dart';
import 'package:ratel/services/wallet/wallet_service.dart';
import 'package:ratel/utils/biyard_navigate_observer/biyard_navigate_observer.dart';

import 'exports.dart';

@pragma('vm:entry-point')
Future<void> _firebaseMessagingBackgroundHandler(RemoteMessage message) async {
  WidgetsFlutterBinding.ensureInitialized();
  await Firebase.initializeApp(options: DefaultFirebaseOptions.currentPlatform);

  debugPrint('BG message: id=${message.messageId}, data=${message.data}');
}

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  await Firebase.initializeApp(options: DefaultFirebaseOptions.currentPlatform);

  FirebaseMessaging.onBackgroundMessage(_firebaseMessagingBackgroundHandler);

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
  ReportsService.init();
  UserService.init();
  AssetService.init();
  WalletService.init();
  await NotificationsService.init();
  DocumentsService.init();
  DriveApi.init();
  SpaceFilesService.init();
  SpacePollsService.init();
  SpaceBoardsService.init();
  Get.put<ThemeController>(ThemeController());

  SystemChrome.setPreferredOrientations([DeviceOrientation.portraitUp]).then((
    value,
  ) {
    initializeLogger(Config.logLevel, false);
    logger.i('App is starting...');

    runApp(const MyApp());
  });
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    logger.d(
      '${Get.deviceLocale?.languageCode}, '
      '${Get.deviceLocale?.countryCode}, '
      '${MainLocalization.appName}',
    );

    return Obx(() {
      final mode = ThemeController.to.themeMode.value;

      return GetMaterialApp.router(
        scrollBehavior: const MaterialScrollBehavior().copyWith(
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
        themeMode: mode,
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
          initialRoute: introScreen,
        ),
        getPages: AppRoutes.pages,
      );
    });
  }
}
