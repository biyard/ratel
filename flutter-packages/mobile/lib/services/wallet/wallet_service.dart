import 'dart:convert';
import 'package:convert/convert.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:reown_appkit/reown_appkit.dart';

class WalletService extends GetxService {
  static void init() =>
      Get.put<WalletService>(WalletService(), permanent: true);

  //FIXME: add project id
  static const String _projectId = '';
  static const String _nativeScheme = 'ratelapp://';

  ReownAppKitModal? _kit;
  ReownAppKitModal get appKitModal => _kit!;

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

  Future<String?> signMessage(String message) async {
    if (_kit == null || !_kit!.isConnected || _kit!.session == null)
      return null;
    final chainId = _kit!.selectedChain!.chainId;
    final ns = ReownAppKitModalNetworks.getNamespaceForChainId(chainId);
    final addr = _kit!.session!.getAddress(ns);
    if (addr == null || addr.isEmpty) return null;
    final encoded = hex.encode(utf8.encode(message));
    final res = await _kit!.request(
      topic: _kit!.session!.topic,
      chainId: chainId,
      request: SessionRequestParams(
        method: 'personal_sign',
        params: ['0x$encoded', addr],
      ),
    );
    return res is String ? res : res?.toString();
  }

  Future<void> disconnect() async {
    try {
      await _kit?.disconnect();
    } catch (_) {}
  }
}
