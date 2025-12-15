import 'package:ratel/exports.dart';

class SpacePostListResult {
  final List<SpacePostModel> posts;
  final String? bookmark;

  const SpacePostListResult({required this.posts, required this.bookmark});

  factory SpacePostListResult.fromJson(Json j) {
    final postsJson = (j['posts'] as List? ?? const []);
    final posts = postsJson
        .whereType<Map>()
        .map((e) => SpacePostModel.fromJson(e.cast<String, dynamic>()))
        .toList();

    return SpacePostListResult(
      posts: posts,
      bookmark: j['bookmark'] as String?,
    );
  }

  Json toJson() => {
    'posts': posts.map((p) => p.toJson()).toList(),
    'bookmark': bookmark,
  };
}

class SpacePostModel {
  final String pk;
  final int createdAt;
  final int updatedAt;
  final int startedAt;
  final int endedAt;
  final String title;
  final String htmlContents;
  final String categoryName;
  int reports;
  final int numberOfComments;

  final String userPk;
  final String authorDisplayName;
  final String authorProfileUrl;
  final String authorUsername;

  bool isReport;
  final List<String> urls;
  final List<FileModel> files;
  final List<SpacePostCommentModel> comments;

  SpacePostModel({
    required this.pk,
    required this.createdAt,
    required this.updatedAt,
    required this.startedAt,
    required this.endedAt,
    required this.title,
    required this.htmlContents,
    required this.categoryName,
    required this.numberOfComments,
    required this.userPk,
    required this.authorDisplayName,
    required this.authorProfileUrl,
    required this.authorUsername,
    required this.urls,
    required this.files,
    required this.comments,
    required this.isReport,
    required this.reports,
  });

  factory SpacePostModel.fromJson(Json j) {
    final filesJson = (j['files'] as List? ?? const []);
    final commentsJson = (j['comments'] as List? ?? const []);

    return SpacePostModel(
      pk: j['pk']?.toString() ?? '',
      createdAt: (j['created_at'] as num?)?.toInt() ?? 0,
      updatedAt: (j['updated_at'] as num?)?.toInt() ?? 0,
      startedAt: (j['started_at'] as num?)?.toInt() ?? 0,
      endedAt: (j['ended_at'] as num?)?.toInt() ?? 0,
      title: j['title']?.toString() ?? '',
      htmlContents: j['html_contents']?.toString() ?? '',
      categoryName: j['category_name']?.toString() ?? '',
      numberOfComments: (j['number_of_comments'] as num?)?.toInt() ?? 0,
      userPk: j['user_pk']?.toString() ?? '',
      authorDisplayName: j['author_display_name']?.toString() ?? '',
      authorProfileUrl: j['author_profile_url']?.toString() ?? '',
      authorUsername: j['author_username']?.toString() ?? '',
      urls: (j['urls'] as List? ?? const []).map((e) => e.toString()).toList(),
      files: filesJson
          .whereType<Map>()
          .map((e) => FileModel.fromJson(e.cast<String, dynamic>()))
          .toList(),
      comments: commentsJson
          .whereType<Map>()
          .map((e) => SpacePostCommentModel.fromJson(e.cast<String, dynamic>()))
          .toList(),
      isReport: j['is_report'] as bool? ?? false,
      reports: (j['reports'] as num?)?.toInt() ?? 0,
    );
  }

  Json toJson() => {
    'pk': pk,
    'created_at': createdAt,
    'updated_at': updatedAt,
    'started_at': startedAt,
    'ended_at': endedAt,
    'title': title,
    'html_contents': htmlContents,
    'category_name': categoryName,
    'number_of_comments': numberOfComments,
    'user_pk': userPk,
    'author_display_name': authorDisplayName,
    'author_profile_url': authorProfileUrl,
    'author_username': authorUsername,
    'urls': urls,
    'files': files.map((f) => f.toJson()).toList(),
    'comments': comments.map((c) => c.toJson()).toList(),
  };
}

class SpacePostCommentModel {
  final String pk;
  final String sk;
  final int updatedAt;
  final int createdAt;
  final String content;
  int likes;
  int reports;
  final int replies;
  final String? parentCommentSk;
  final String authorPk;
  final String authorDisplayName;
  final String authorUsername;
  final String authorProfileUrl;
  bool liked;
  bool isReport;

  SpacePostCommentModel({
    required this.pk,
    required this.sk,
    required this.updatedAt,
    required this.createdAt,
    required this.content,
    required this.likes,
    required this.reports,
    required this.replies,
    required this.parentCommentSk,
    required this.authorPk,
    required this.authorDisplayName,
    required this.authorUsername,
    required this.authorProfileUrl,
    required this.liked,
    required this.isReport,
  });

  factory SpacePostCommentModel.fromJson(Json j) => SpacePostCommentModel(
    pk: j['pk']?.toString() ?? '',
    sk: j['sk']?.toString() ?? '',
    updatedAt: (j['updated_at'] as num?)?.toInt() ?? 0,
    createdAt: (j['created_at'] as num?)?.toInt() ?? 0,
    content: j['content']?.toString() ?? '',
    likes: (j['likes'] as num?)?.toInt() ?? 0,
    reports: (j['reports'] as num?)?.toInt() ?? 0,
    replies: (j['replies'] as num?)?.toInt() ?? 0,
    parentCommentSk: j['parent_comment_sk']?.toString(),
    authorPk: j['author_pk']?.toString() ?? '',
    authorDisplayName: j['author_display_name']?.toString() ?? '',
    authorUsername: j['author_username']?.toString() ?? '',
    authorProfileUrl: j['author_profile_url']?.toString() ?? '',
    liked: j['liked'] as bool? ?? false,
    isReport: j['is_report'] as bool? ?? false,
  );

  Json toJson() => {
    'pk': pk,
    'sk': sk,
    'updated_at': updatedAt,
    'created_at': createdAt,
    'content': content,
    'likes': likes,
    'replies': replies,
    'parent_comment_sk': parentCommentSk,
    'author_pk': authorPk,
    'author_display_name': authorDisplayName,
    'author_username': authorUsername,
    'author_profile_url': authorProfileUrl,
    'liked': liked,
  };
}

class SpacePostCommentListResult {
  final List<SpacePostCommentModel> items;
  final String? bookmark;

  SpacePostCommentListResult({required this.items, this.bookmark});

  factory SpacePostCommentListResult.fromJson(Map<String, dynamic> json) {
    final list = (json['items'] as List? ?? [])
        .map((e) => SpacePostCommentModel.fromJson(e as Map<String, dynamic>))
        .toList();
    return SpacePostCommentListResult(
      items: list,
      bookmark: json['bookmark'] as String?,
    );
  }
}
