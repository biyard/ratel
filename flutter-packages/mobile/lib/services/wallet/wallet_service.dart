import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:reown_appkit/reown_appkit.dart';

class WalletService extends GetxService {
  static void init() {
    Get.put<WalletService>(WalletService());
  }

  //FIXME: add to project id
  static const String _projectId = '';
  static const String _nativeScheme = 'ratelapp://';

  ReownAppKitModal? _kit;
  bool _ready = false;

  bool get isReady => _ready;
  ReownAppKitModal get appKitModal => _kit!;

  Future<void> ensureInit(
    BuildContext context, {
    bool logoutIfConnected = false,
  }) async {
    if (_kit == null) {
      _kit = ReownAppKitModal(
        context: context,
        projectId: _projectId,
        metadata: const PairingMetadata(
          name: 'Ratel',
          description: 'Ratel mobile dApp',
          url: 'https://dev.ratel.foundation',
          icons: ['https://dev.ratel.foundation/favicon.png'],
          redirect: Redirect(native: _nativeScheme),
        ),
      );
      await _kit!.init();
      _ready = true;
    }
    if (logoutIfConnected &&
        (_kit!.session != null || (currentAddress() ?? '').isNotEmpty)) {
      try {
        await _kit!.disconnect();
      } catch (_) {}
    }
  }

  String? currentAddress() {
    if (_kit == null) return null;
    final chainId = _kit!.selectedChain?.chainId ?? '1';
    final ns = ReownAppKitModalNetworks.getNamespaceForChainId(chainId);
    return _kit!.session?.getAddress(ns);
  }

  Future<void> disconnect() async {
    try {
      await _kit?.disconnect();
    } catch (_) {}
  }
}
