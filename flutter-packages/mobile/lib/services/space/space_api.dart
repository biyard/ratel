import 'package:ratel/exports.dart';

class SpaceApi extends GetConnect {
  final apiEndpoint = Config.apiEndpoint;

  SpaceApi() {
    httpClient.addRequestModifier<void>((req) async {
      final authApi = Get.find<AuthApi>();
      final cookie = await authApi.cookieHeaderAsync();
      logger.d('${req.method} ${req.url} | FeedsApi Cookie: $cookie');
      if (cookie?.isNotEmpty == true) req.headers['Cookie'] = cookie!;
      return req;
    });
  }

  Future<SpaceModel> getSpaceById(int spaceId) async {
    final uri = Uri.parse(apiEndpoint).resolve('/v1/spaces/$spaceId');

    final headers = <String, String>{'Content-Type': 'application/json'};
    final res = await get(uri.toString(), headers: headers);

    if (!res.isOk) {
      return SpaceModel(
        id: 0,
        title: "",
        htmlContents: "",
        files: [],
        discussions: [],
      );
    }

    logger.d("space info: ${res.body}");
    final item = res.body;
    final List<FileModel> files = [];
    final List<DiscussionModel> discussions = [];

    for (var i = 0; i < item["files"].length; i++) {
      final file = item["files"][i];

      files.add(
        FileModel(
          name: file["name"],
          size: file["size"],
          ext: file["ext"],
          url: file["url"],
        ),
      );
    }

    for (var i = 0; i < item["discussions"].length; i++) {
      final discussion = item["discussions"][i];

      discussions.add(
        DiscussionModel(
          id: int.parse(discussion["id"].toString()),
          startedAt: int.parse(discussion["started_at"].toString()),
          endedAt: int.parse(discussion["ended_at"].toString()),
          name: discussion["name"],
          record: discussion["record"] ?? "",
        ),
      );
    }

    return SpaceModel(
      id: int.parse(item["id"].toString()),
      title: item["title"] ?? "",
      htmlContents: item["html_contents"] ?? "",
      files: files,
      discussions: discussions,
    );
  }
}
