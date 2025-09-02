import 'package:ratel/exports.dart';

class NetworkController extends BaseController {
  @override
  void onInit() {
    listNetworks();
    super.onInit();
  }

  void listNetworks() async {
    final items = await networkApi.getNetworksByV1();
    logger.d(
      "items length: ${items.followers.length} ${items.followings.length}",
    );
    invitations(items.followings);
    suggestions(items.followers);
  }

  Future<void> acceptSuggestion(int followeeId) async {
    final suggestionIds = suggestions.map((e) => e.id).toList(growable: false);
    final res = await networkApi.acceptSuggestion(suggestionIds, followeeId);
    if (res != null) {
      suggestions.removeWhere((e) => e.id == followeeId);
      suggestions.add(res);
    } else {
      Biyard.error(
        "Failed to follow user.",
        "Follow user is failed. Please try again later.",
      );
    }
  }

  Future<void> rejectSuggestion(int followeeId) async {
    final suggestionIds = suggestions.map((e) => e.id).toList(growable: false);
    final res = await networkApi.rejectSuggestion(suggestionIds, followeeId);
    if (res != null) {
      suggestions.removeWhere((e) => e.id == followeeId);
      suggestions.add(res);
    } else {
      Biyard.error(
        "Failed to reject suggestion.",
        "Reject Suggestion is failed. Please try again later.",
      );
    }
  }

  Future<void> acceptInvitation(int followeeId) async {
    final invitationIds = invitations.map((e) => e.id).toList(growable: false);
    final res = await networkApi.acceptInvitation(invitationIds, followeeId);
    if (res != null) {
      invitations.removeWhere((e) => e.id == followeeId);

      if (res.id != 0) {
        invitations.add(res);
      }
    } else {
      Biyard.error(
        "Failed to accept invitation.",
        "Accept invitation is failed. Please try again later.",
      );
    }
  }

  Future<void> rejectInvitation(int followeeId) async {
    final invitationIds = invitations.map((e) => e.id).toList(growable: false);
    final res = await networkApi.rejectInvitation(invitationIds, followeeId);
    if (res != null) {
      invitations.removeWhere((e) => e.id == followeeId);

      if (res.id != 0) {
        invitations.add(res);
      }
    } else {
      Biyard.error(
        "Failed to accept invitation.",
        "Accept invitation is failed. Please try again later.",
      );
    }
  }

  final networkApi = Get.find<NetworkApi>();

  RxList<NetworkModel> invitations = <NetworkModel>[].obs;
  RxList<NetworkModel> suggestions = <NetworkModel>[].obs;
}
