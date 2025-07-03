import 'dart:async';
import 'dart:convert';

import 'package:ratel/exports.dart';
import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:firebase_auth/firebase_auth.dart';
import 'package:google_sign_in/google_sign_in.dart';
import 'package:googleapis/drive/v3.dart' as dapi;
import 'package:googleapis_auth/googleapis_auth.dart' as gapis;
// import 'package:googleapis/drive/v3.dart';
// ignore: depend_on_referenced_packages
import 'package:http/http.dart' as http;

export 'package:firebase_analytics/firebase_analytics.dart';
import 'package:sign_in_with_apple/sign_in_with_apple.dart';

class BaseException implements Exception {
  final String message;
  final int code;

  BaseException(this.code, this.message);

  @override
  String toString() {
    return message;
  }

  int getCode() {
    return code;
  }
}

final errorGoogleDriveFileNotFound = BaseException(10, 'file not found');

class AnalyticsArgs {
  String name;
  Map<String, Object?>? parameters;
  AnalyticsCallOptions? callOptions;

  AnalyticsArgs({required this.name, this.parameters, this.callOptions});
}

class ByFirebaseAnalytics {
  FirebaseAnalytics? analytics;
  List<AnalyticsArgs> events = [];
  AnalyticsArgs? userId;
  Completer initialied = Completer();

  Future<FirebaseAnalytics> instance() async {
    await initialied.future;
    return analytics!;
  }

  Future setAnalytics(FirebaseAnalytics analytics) async {
    logger.d('setAnalytics');
    if (userId != null) {
      logger.d('setUserId in setAnalytics');
      analytics.setUserId(id: userId!.name, callOptions: userId!.callOptions);
      userId = null;
    }

    for (var event in events) {
      logger.d('logEvent in setAnalytics: $event');
      await analytics.logEvent(
        name: event.name,
        parameters: event.parameters?.map(
          (key, value) => MapEntry(key, value as Object),
        ),
        callOptions: event.callOptions,
      );
    }
    events.clear();
    await analytics.logEvent(name: 'local_refactoring');

    this.analytics = analytics;
    initialied.complete();
    logger.d('setAnalytics completed');
  }

  Future<void> logAppOpen({
    AnalyticsCallOptions? callOptions,
    Map<String, Object?>? parameters,
  }) async {
    logger.d('firebase: logAppOpen');
    if (analytics == null) {
      events.add(
        AnalyticsArgs(
          name: 'app_open',
          parameters: parameters?.map(
            (key, value) => MapEntry(key, value as Object),
          ),
          callOptions: callOptions,
        ),
      );
      return;
    }

    return await analytics!.logAppOpen(
      callOptions: callOptions,
      parameters: parameters?.map(
        (key, value) => MapEntry(key, value as Object),
      ),
    );
  }

  Future<void> logEvent({
    required String name,
    Map<String, Object?>? parameters,
    AnalyticsCallOptions? callOptions,
  }) async {
    logger.d('firebase: logEvent');
    if (analytics == null) {
      events.add(
        AnalyticsArgs(
          name: name,
          parameters: parameters,
          callOptions: callOptions,
        ),
      );

      return;
    }

    await analytics?.logEvent(
      name: name,
      parameters: parameters?.map(
        (key, value) => MapEntry(key, value as Object),
      ),
      callOptions: callOptions,
    );
  }

  Future<void> logScreenView({
    String? screenClass,
    String? screenName,
    String? previousScreenName,
    Map<String, Object?>? parameters,
    AnalyticsCallOptions? callOptions,
  }) async {
    await logEvent(
      name: 'page_view',
      // ignore: invalid_use_of_visible_for_testing_member
      parameters: filterOutNulls(<String, Object?>{
        "page_location": '${Uri.base.origin}$screenName',
        "page_title": screenName,
        "page_referrer": previousScreenName,
        if (parameters != null) ...parameters,
      }),
      callOptions: callOptions,
    );

    return logEvent(
      name: 'screen_view',
      // ignore: invalid_use_of_visible_for_testing_member
      parameters: filterOutNulls(<String, Object?>{
        "screen_class": screenName,
        "screen_name": screenName,
        if (parameters != null) ...parameters,
      }),
      callOptions: callOptions,
    );
  }

  Future<void> setUserId({
    required String? id,
    AnalyticsCallOptions? callOptions,
  }) async {
    if (analytics == null) {
      userId = AnalyticsArgs(name: id!, callOptions: callOptions);
      return;
    }

    await analytics?.setUserId(id: id, callOptions: callOptions);
  }
}

class ByFirebase extends GetxService {
  ByFirebase();

  GetConnect cli = Get.find<GetConnect>();
  final ByFirebaseAnalytics analytics = ByFirebaseAnalytics();
  final initialied = Completer();

  User? user;
  UserCredential? credential;

  static void init() {
    Get.put(GetConnect());
    final bf = ByFirebase();
    Get.put<ByFirebase>(bf);
    bf.lazyInit();
  }

  Future lazyInit() async {
    await analytics.setAnalytics(FirebaseAnalytics.instance);

    initialied.complete();
  }

  Future<User?> signIn() async {
    user = null;

    await initialied.future;

    FirebaseAuth auth = FirebaseAuth.instance;

    try {
      logger.d("signIn");
      GoogleSignIn googleSignIn = GoogleSignIn(
        scopes: [
          // "email",
          // 'https://www.googleapis.com/auth/drive.appdata',
        ],
      );
      logger.d("scopes: ${googleSignIn.scopes}");
      GoogleSignInAccount? googleUser = await googleSignIn.signIn();

      if (googleUser == null) {
        logger.d("user cancelled");
        // The user canceled
        return null;
      }

      logger.d("before authentication");
      GoogleSignInAuthentication googleAuth = await googleUser.authentication;

      logger.d(
        "access token ${googleAuth.accessToken} idToken: ${googleAuth.idToken}",
      );

      OAuthCredential oAuthCredential = GoogleAuthProvider.credential(
        accessToken: googleAuth.accessToken,
        idToken: googleAuth.idToken,
      );

      credential = await auth.signInWithCredential(oAuthCredential);
      logger.d("credential: ${credential}");
      user = credential?.user;
    } catch (e) {
      logger.e(e);
    }

    return user;
  }

  Future<User?> signInWithApple() async {
    await initialied.future;

    FirebaseAuth auth = FirebaseAuth.instance;

    user = null;

    try {
      logger.d("signInWithApple");
      final AuthorizationCredentialAppleID
      appleCredential = await SignInWithApple.getAppleIDCredential(
        scopes: [
          // AppleIDAuthorizationScopes.email,
          // AppleIDAuthorizationScopes.fullName,
        ],
        webAuthenticationOptions: WebAuthenticationOptions(
          //FIXME: fix config
          clientId: "com.biyard.dagitMobile",
          redirectUri: Uri.parse(
            "https://glow-abiding-parallelogram.glitch.me/callbacks/sign_in_with_apple",
          ),
        ),
      );
      if (appleCredential.identityToken == null) return null;

      OAuthCredential authCredential = OAuthProvider('apple.com').credential(
        idToken: appleCredential.identityToken,
        accessToken: appleCredential.authorizationCode,
      );

      logger.d("auth credential: ${authCredential}");

      credential = await auth.signInWithCredential(authCredential);
      logger.d("credential: ${credential?.user}");
      user = credential?.user;
    } catch (e) {
      logger.e(e);
    }

    return user;
  }

  Future<String> readFileFromGoogleDrive(String filename) async {
    final api = dapi.DriveApi(googleClient());
    var res = await api.files.list(spaces: 'appDataFolder');
    logger.d(res.files);
    String id = "";
    for (var file in res.files!) {
      if (file.name == filename) {
        id = file.id!;
        break;
      }
    }

    if (id.isEmpty) {
      throw errorGoogleDriveFileNotFound;
    }

    var file =
        await api.files.get(id, downloadOptions: dapi.DownloadOptions.fullMedia)
            as dapi.Media;

    var data = await Encoding.getByName('utf-8')!.decodeStream(file.stream);
    logger.d(data);

    return data;
  }

  http.Client googleClient() {
    final token = credential!.credential!.accessToken!;
    logger.d('google drive token $token');

    final gapis.AccessCredentials credentials = gapis.AccessCredentials(
      gapis.AccessToken(
        'Bearer',
        token,
        DateTime.now().toUtc().add(const Duration(days: 365)),
      ),
      null,
      ['https://www.googleapis.com/auth/drive.appdata'],
    );

    return gapis.authenticatedClient(http.Client(), credentials);
  }

  Future createFileToGoogleDrive(
    String filename,
    String id,
    String seed,
  ) async {
    final api = dapi.DriveApi(googleClient());
    dapi.File fileToUpload = dapi.File();
    fileToUpload.name = filename;
    fileToUpload.parents = ['appDataFolder'];

    var data = Encoding.getByName('utf-8')!.encode('$id:$seed');
    var stream = http.ByteStream.fromBytes(data);
    await api.files.create(
      fileToUpload,
      uploadMedia: dapi.Media(stream, data.length),
    );
  }

  Future<String?> idToken() async {
    return await user?.getIdToken();
  }

  Future<void> tokeninfo() async {
    final response = await cli.get(
      'https://oauth2.googleapis.com/tokeninfo',
      query: {'id_token': await idToken()},
    );
    logger.d(response.body);
  }
}
