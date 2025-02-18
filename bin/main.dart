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
        ..plugin(PrettyUrlsPlugin());

  // Generate the static website.
  await staticShock.generateSite();
}
