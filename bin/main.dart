import 'dart:io';

import 'package:static_shock/static_shock.dart';
import './plugins/html_prettifier_plugin.dart';

Future<void> main(List<String> arguments) async {
  final staticShock =
      StaticShock()
        ..pick(DirectoryPicker.parse("public"))
        ..pick(FilePicker(FileRelativePath("", "_404", "html")))
        ..pick(FilePicker(FileRelativePath("", "favicon", "ico")))
        ..pick(FilePicker(FileRelativePath("", "robots", "txt")))
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
        ..plugin(HtmlPrettifierPlugin())
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
