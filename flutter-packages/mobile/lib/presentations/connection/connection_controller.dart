import 'package:ratel/exports.dart';

class ConnectionController extends BaseController {
  final query = ''.obs;

  //FIXME: fix to query api
  final networks = [
    Network(
      userId: 1,
      profileUrl: "",
      username: "User name 1",
      description:
          "Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY ",
    ),
    Network(
      userId: 2,
      profileUrl: "",
      username: "User name 2",
      description:
          "Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY ",
    ),
    Network(
      userId: 3,
      profileUrl: "",
      username: "User name 3",
      description:
          "Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY ",
    ),
    Network(
      userId: 4,
      profileUrl: "",
      username: "User name 4",
      description:
          "Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY ",
    ),
    Network(
      userId: 5,
      profileUrl: "",
      username: "User name 5",
      description:
          "Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY ",
    ),
    Network(
      userId: 6,
      profileUrl: "",
      username: "User name 6",
      description:
          "Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY ",
    ),
    Network(
      userId: 7,
      profileUrl: "",
      username: "User name 7",
      description:
          "Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY ",
    ),
    Network(
      userId: 8,
      profileUrl: "",
      username: "User name 8",
      description:
          "Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY ",
    ),
    Network(
      userId: 9,
      profileUrl: "",
      username: "User name 9",
      description:
          "Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY ",
    ),
    Network(
      userId: 10,
      profileUrl: "",
      username: "User name 10",
      description:
          "Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY Candidate for State, Senate, NY ",
    ),
  ];

  final followed = <int>{}.obs;

  List<Network> get filtered {
    final q = query.value.trim().toLowerCase();
    if (q.isEmpty) return networks;
    return networks
        .where(
          (n) =>
              n.username.toLowerCase().contains(q) ||
              n.description.toLowerCase().contains(q),
        )
        .toList();
  }

  bool get hasFollowed => followed.isNotEmpty;

  void onSearchChanged(String v) => query.value = v;

  void toggleFollow(int userId) {
    if (followed.contains(userId)) {
      followed.remove(userId);
    } else {
      followed.add(userId);
    }
  }

  void goBack() => Get.rootDelegate.offNamed(AppRoutes.selectTopicScreen);
  void skip() => Get.rootDelegate.offNamed(AppRoutes.welcomeScreen);
  void next() => Get.rootDelegate.offNamed(AppRoutes.welcomeScreen);
}
