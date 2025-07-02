import '../../../exports.dart';

extension Gap on double {
  Widget get vgap {
    return SizedBox(height: this);
  }

  Widget get gap {
    return SizedBox(width: this);
  }
}

extension IntGap on int {
  Widget get vgap {
    return SizedBox(height: this.toDouble());
  }

  Widget get gap {
    return SizedBox(width: this.toDouble());
  }
}
