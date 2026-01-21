import 'package:ratel/exports.dart';

class AccountTile extends StatelessWidget {
  final String displayName;
  final String username;
  final String profileUrl;
  final VoidCallback onTap;

  const AccountTile({
    super.key,
    required this.displayName,
    required this.username,
    required this.profileUrl,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      color: const Color(0xFF171717),
      borderRadius: BorderRadius.circular(14),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(14),
        child: Container(
          width: double.infinity,
          padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 20),
          decoration: BoxDecoration(
            color: const Color(0xFF171717),
            borderRadius: BorderRadius.circular(14),
            border: Border.all(color: const Color(0xFF464646), width: 0.5),
          ),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              _Avatar52(profileUrl: profileUrl),
              const SizedBox(width: 10),
              Expanded(
                child: _AccountTexts(
                  displayName: displayName,
                  username: username,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _Avatar52 extends StatelessWidget {
  final String profileUrl;
  const _Avatar52({required this.profileUrl});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 52,
      height: 52,
      child: ClipRRect(
        borderRadius: BorderRadius.circular(100),
        child: Container(
          color: const Color(0xFFD9D9D9),
          child: profileUrl.isNotEmpty
              ? Image.network(
                  profileUrl,
                  fit: BoxFit.cover,
                  errorBuilder: (_, __, ___) => const SizedBox.shrink(),
                )
              : Image.network(
                  defaultProfileImage,
                  fit: BoxFit.cover,
                  errorBuilder: (_, __, ___) => const SizedBox.shrink(),
                ),
        ),
      ),
    );
  }
}

class _AccountTexts extends StatelessWidget {
  final String displayName;
  final String username;

  const _AccountTexts({required this.displayName, required this.username});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Flexible(
                child: Text(
                  displayName,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    fontSize: 16,
                    height: 24 / 16,
                    fontWeight: FontWeight.w600,
                    color: Colors.white,
                  ),
                ),
              ),
            ],
          ),
          const SizedBox(height: 2),
          Text(
            '@${username}',
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              fontSize: 13,
              height: 20 / 13,
              fontWeight: FontWeight.w600,
              color: Color(0xFF8C8C8C),
            ),
          ),
        ],
      ),
    );
  }
}
