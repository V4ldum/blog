import 'dart:io';

import 'package:static_shock/static_shock.dart';

Future<void> main(List<String> arguments) async {
  final staticShock =
      StaticShock()
        ..pick(DirectoryPicker.parse("public"))
        ..pick(FilePicker.parse("_404.html"))
        ..plugin(MarkdownPlugin())
        ..plugin(JinjaPlugin())
        ..plugin(
          TailwindPlugin(
            tailwindPath: Platform.isWindows ? "./tailwindcss.exe" : "./tailwindcss",
            input: "source/styles/tailwind.css",
            output: "build/styles/tailwind.css",
          ),
        )
        ..plugin(PrettyUrlsPlugin())
        ..plugin(
          RssPlugin(
            includePagesByDefault: false,
            rssFeedPath: FileRelativePath("", "rss", "xml"),
            site: RssSiteConfiguration(homePageUrl: "https://blog.valdum.dev"),
          ),
        );

  // Generate the static website.
  await staticShock.generateSite();
}
