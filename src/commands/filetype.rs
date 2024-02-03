use std::path::Path;

pub fn file_extension(file: &Path) -> &str {
    let extension = file.extension().unwrap().to_str().unwrap(); // HACK: find better way to do this

    let filetype = match extension.to_lowercase().as_str() {
        // at some point for languages just say the extension again... or support idk
        "jpeg" | "jpg" => "JPEG image",
        "png" => "PNG image",
        "gif" => "GIF animation",
        "txt" => "Text file",
        "py" => "Python file",
        "rs" => "Rust file",
        "md" => "Markdown file",
        // maybe find a way to differentiate?
        _ => "Binary executable or unsupported",
    };
    return filetype;
}
