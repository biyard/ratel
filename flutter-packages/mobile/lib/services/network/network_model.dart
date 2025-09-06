class NetworkModel {
  final int id;
  final String profileUrl;
  final String nickname;
  final String username;
  final String description;

  NetworkModel({
    required this.id,
    required this.profileUrl,
    required this.nickname,
    required this.username,
    required this.description,
  });
}

class MyNetworkModel {
  final List<NetworkModel> followers;
  final List<NetworkModel> followings;

  MyNetworkModel({required this.followers, required this.followings});
}
