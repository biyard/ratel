import 'package:ratel/exports.dart';
import 'package:ratel/presentations/verified/components/credentials.dart';

class MyCredentials extends StatefulWidget {
  const MyCredentials({super.key, required this.did});

  final String did;

  @override
  State<MyCredentials> createState() => _MyCredentialsState();
}

class _MyCredentialsState extends State<MyCredentials> {
  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Column(
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(8, 8, 8, 4),
            child: Row(
              children: [
                IconButton(
                  onPressed: () => Get.back<void>(),
                  icon: const Icon(Icons.arrow_back, color: Colors.white),
                ),
                const Text(
                  'My Credential',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 20,
                    fontWeight: FontWeight.w700,
                  ),
                ),
              ],
            ),
          ),

          Padding(
            padding: const EdgeInsets.fromLTRB(14, 6, 14, 12),
            child: CredentialBanner(
              title: 'Verifiable Credential',
              subtitle: 'ID : ${widget.did}',
              icon: const Icon(
                Icons.verified_rounded,
                size: 56,
                color: AppColors.primary,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
