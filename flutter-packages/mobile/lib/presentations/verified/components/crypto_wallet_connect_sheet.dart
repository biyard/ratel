import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:reown_appkit/reown_appkit.dart';
import 'package:ratel/services/wallet/wallet_service.dart';

class CryptoWalletConnectSheet extends StatefulWidget {
  const CryptoWalletConnectSheet({super.key});
  @override
  State<CryptoWalletConnectSheet> createState() =>
      _CryptoWalletConnectSheetState();
}

class _CryptoWalletConnectSheetState extends State<CryptoWalletConnectSheet> {
  String? _addr;
  bool _waiting = false;

  late final void Function(ModalConnect?) _onConnect;
  late final void Function(ModalConnect?) _onUpdate;
  late final void Function(ModalDisconnect?) _onDisconnect;
  late final void Function(ModalError?) _onError;

  @override
  void initState() {
    super.initState();
    final ws = Get.find<WalletService>();
    _addr = ws.currentAddress();

    _onConnect = (_) => _afterPossibleConnect();
    _onUpdate = (_) => _afterPossibleConnect();
    _onDisconnect = (_) {};
    _onError = (_) {};

    ws.appKitModal.onModalConnect.subscribe(_onConnect);
    ws.appKitModal.onModalUpdate.subscribe(_onUpdate);
    ws.appKitModal.onModalDisconnect.subscribe(_onDisconnect);
    ws.appKitModal.onModalError.subscribe(_onError);
  }

  @override
  void dispose() {
    try {
      final ws = Get.find<WalletService>();
      ws.appKitModal.onModalConnect.unsubscribe(_onConnect);
      ws.appKitModal.onModalUpdate.unsubscribe(_onUpdate);
      ws.appKitModal.onModalDisconnect.unsubscribe(_onDisconnect);
      ws.appKitModal.onModalError.unsubscribe(_onError);
    } catch (_) {}
    super.dispose();
  }

  Future<void> _afterPossibleConnect() async {
    if (!mounted) return;
    final ws = Get.find<WalletService>();
    setState(() => _waiting = true);
    final addrNow = ws.currentAddress();
    setState(() {
      _addr = addrNow;
      _waiting = false;
    });
    if (addrNow != null && addrNow.isNotEmpty) {
      await ws.signMessage(
        'Ratel wallet verification ${DateTime.now().toIso8601String()}',
      );
      if (!mounted) return;
      Navigator.of(context).pop(addrNow);
    }
  }

  @override
  Widget build(BuildContext context) {
    final ws = Get.find<WalletService>();

    return SafeArea(
      top: false,
      child: SizedBox(
        width: double.infinity,
        child: Padding(
          padding: const EdgeInsets.fromLTRB(20, 16, 20, 24),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              const Text(
                'Connect your wallet',
                style: TextStyle(
                  color: Colors.white,
                  fontSize: 18,
                  fontWeight: FontWeight.w700,
                ),
              ),
              const SizedBox(height: 12),
              AppKitModalConnectButton(
                appKit: ws.appKitModal,
                context: context,
              ),
              const SizedBox(height: 12),
              if (_waiting) ...[
                const SizedBox(height: 8),
                const Text(
                  'Waiting for approval...',
                  style: TextStyle(color: Colors.white70),
                ),
              ],
              if (_addr != null && _addr!.isNotEmpty) ...[
                const SizedBox(height: 8),
                Text(
                  _addr!,
                  style: const TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ],
              const SizedBox(height: 8),
              TextButton(
                onPressed: () => Navigator.of(context).pop(null),
                child: const Text(
                  'Cancel',
                  style: TextStyle(color: Colors.white70),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
