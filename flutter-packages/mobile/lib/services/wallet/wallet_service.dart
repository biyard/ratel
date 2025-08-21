import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:reown_appkit/reown_appkit.dart';

class WalletService extends GetxService {
  static void init() {
    Get.put<WalletService>(WalletService());
  }

  //FIXME: add project id to wallet service file
  static const String _projectId = '';
  static const String _nativeScheme = 'ratelapp://';

  ReownAppKitModal? _kit;
  ReownAppKitModal get appKitModal => _kit!;
  bool get isReady => _kit != null;

  Future<void> ensureInit(BuildContext context) async {
    if (_kit != null) return;
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
  }

  String? currentAddress() {
    if (_kit == null) return null;
    final chainId = _kit!.selectedChain?.chainId ?? '1';
    final ns = ReownAppKitModalNetworks.getNamespaceForChainId(chainId);
    return _kit!.session?.getAddress(ns);
  }

  void openModal() => _kit?.openModalView();
}
