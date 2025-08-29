import 'package:ratel/exports.dart';

class DashboardsModel {
  final List<SpaceSummary> topSpaces;
  final List<FeedSummary> matchedFeeds;
  final List<FeedSummary> newFeeds;

  const DashboardsModel({
    required this.topSpaces,
    required this.matchedFeeds,
    required this.newFeeds,
  });
}
