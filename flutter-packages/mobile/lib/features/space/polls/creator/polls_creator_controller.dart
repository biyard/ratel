import 'package:ratel/exports.dart';

class PollsCreatorController extends BaseController {
  final SpacePollsApi _pollsApi = Get.find<SpacePollsApi>();

  late final String spacePk;
  final polls = <PollModel>[].obs;
  final bookmark = RxnString();
  final isLoading = false.obs;

  @override
  void onInit() {
    super.onInit();
    logger.d('PollsCreatorController initialized');

    final rawPk = Get.parameters['spacePk'];
    if (rawPk == null || rawPk.isEmpty) {
      logger.w(
        'PollsCreatorController: spacePk is null or empty in parameters',
      );
      return;
    }

    spacePk = Uri.decodeComponent(rawPk);
    _loadPolls();
  }

  Future<void> _loadPolls({String? cursor, bool append = false}) async {
    try {
      isLoading.value = true;

      final result = await _pollsApi.listPolls(
        spacePk,
        bookmark: cursor ?? bookmark.value,
      );

      if (append) {
        polls.addAll(result.polls);
      } else {
        polls.assignAll(result.polls);
      }

      bookmark.value = result.bookmark;

      logger.d(
        'Loaded ${result.polls.length} polls for $spacePk, bookmark: ${result.bookmark}',
      );

      for (final p in result.polls) {
        logger.d(
          'Poll sk=${p.sk}, status=${pollStatusToString(p.status)}, '
          'questions=${p.questions.length}, responses=${p.userResponseCount}, '
          'default=${p.isDefault}',
        );
      }
    } catch (e) {
      logger.e('Failed to load polls for $spacePk: $e');
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> reload() => _loadPolls(append: false);

  Future<void> loadMore() {
    if (bookmark.value == null) {
      logger.d('No more polls to load for $spacePk');
      return Future.value();
    }
    return _loadPolls(cursor: bookmark.value, append: true);
  }
}

extension PollsCreatorControllerX on PollsCreatorController {
  void onPollTap(PollModel poll) {
    if (poll.status != PollStatus.inProgress) {
      return;
    }

    final spaceController = Get.find<SpaceController>();

    spaceController.onTabSelected(
      SpaceTab(id: 'poll', label: 'Polls', route: '/poll'),
      pollSk: poll.sk,
    );
  }
}
