import 'dart:convert';

import 'package:flutter/material.dart';

class RoundContainer extends StatelessWidget {
  final AlignmentGeometry? alignment;
  final EdgeInsetsGeometry? padding;
  final Color? color;
  final Decoration? foregroundDecoration;
  final EdgeInsetsGeometry? margin;
  final Matrix4? transform;
  final AlignmentGeometry? transformAlignment;
  final Clip clipBehavior;
  final Widget? child;
  final double? width;
  final double? height;
  final BoxConstraints? constraints;
  final String? imageUrl;
  final bool enableShadow;
  final LinearGradient? linearGradient;
  final Border? border;
  final double radius;
  final BoxShape shape;
  final GestureTapCallback? onTap;
  final MouseCursor? mouseCursor;

  const RoundContainer({
    super.key,
    this.alignment,
    this.padding,
    this.color,
    this.foregroundDecoration,
    this.width,
    this.height,
    this.constraints,
    this.margin,
    this.transform,
    this.transformAlignment,
    this.child,
    this.clipBehavior = Clip.none,
    this.imageUrl,
    this.enableShadow = false,
    this.border,
    this.linearGradient,
    this.radius = 8.0,
    this.shape = BoxShape.rectangle,
    this.onTap,
    this.mouseCursor,
  });

  convertImage(String url) {
    if (url.startsWith('data:image')) {
      final image = imageUrl!.replaceAll(RegExp(r'data:image\/.*;base64,'), '');
      return Image.memory(base64Decode(image)).image;
    }

    if (url.startsWith('http')) return NetworkImage(url);

    return AssetImage(url);
  }

  @override
  Widget build(BuildContext context) {
    final decoration = BoxDecoration(
        image: imageUrl != null
            ? DecorationImage(
                image: convertImage(imageUrl!), fit: BoxFit.contain)
            : null,
        shape: shape,
        color: color ?? Theme.of(context).colorScheme.primaryContainer,
        borderRadius: BorderRadius.circular(radius),
        boxShadow: enableShadow
            ? const [
                BoxShadow(
                  offset: Offset(0, 3),
                  blurRadius: 5,
                  color: Colors.black26,
                )
              ]
            : null,
        gradient: linearGradient,
        border: border);
    final container = ClipRRect(
      borderRadius: BorderRadius.circular(radius),
      child: Container(
        decoration: decoration,
        margin: margin,
        width: width,
        height: height,
        alignment: alignment,
        padding: padding,
        foregroundDecoration: foregroundDecoration,
        constraints: constraints,
        transform: transform,
        transformAlignment: transformAlignment,
        clipBehavior: clipBehavior,
        child: child,
      ),
    );

    return onTap != null
        ? InkWell(
            onTap: onTap,
            mouseCursor: mouseCursor,
            child: container,
          )
        : container;
  }
}
