import 'package:ratel/services/firebase/firebase.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:ratel/exports.dart';

class BiyardNavigatorObserver extends NavigatorObserver {
  late List<NavigatorObserver> navigatorObservers;
  ByFirebase firebase = Get.find<ByFirebase>();
  void Function(Route<dynamic>, Route<dynamic>?)? pushHook;
  void Function(Route<dynamic>, Route<dynamic>?)? popHook;

  BiyardNavigatorObserver({this.pushHook, this.popHook}) {
    navigatorObservers = [];
  }

  @override
  void didPush(Route<dynamic> route, Route<dynamic>? previousRoute) {
    logger.d('didPush: ${route.settings.name}');
    firebase.analytics.logScreenView(
      screenClass: route.settings.name,
      screenName: route.settings.name,
      previousScreenName: previousRoute?.settings.name,
    );
    pushHook?.call(route, previousRoute);

    for (var observer in navigatorObservers) {
      logger.d('didPush observer: $observer');
      observer.didPush(route, previousRoute);
    }
  }

  @override
  void didPop(Route<dynamic> route, Route<dynamic>? previousRoute) {
    logger.d('didPop: ${route.settings.name}');
    firebase.analytics.logScreenView(screenName: previousRoute?.settings.name);
    popHook?.call(route, previousRoute);

    for (var observer in navigatorObservers) {
      logger.d('didPop observer: $observer');
      observer.didPop(route, previousRoute);
    }
  }
}
