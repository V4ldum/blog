use ssg::markdown::MarkdownPlugin;
use ssg::pickers::file_picker::FileFilter;
use ssg::pickers::FilePicker;
use ssg::pretty_url::PrettyUrlPlugin;
use ssg::SSGPipelineBuilder;
use ssg::tailwind::TailwindPlugin;
use ssg::template::TemplatePlugin;

fn main() {
    let ssg = SSGPipelineBuilder::new()
    .plugin(MarkdownPlugin::new())
    .plugin(TemplatePlugin::new())
    .plugin(TailwindPlugin::new())
    .plugin(PrettyUrlPlugin::new())
    .picker(FilePicker::new(FileFilter::Directory("public")))
        .picker(FilePicker::new(FileFilter::Name("_404.html")))
    .build();

    ssg.run();
}
