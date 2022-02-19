mod cli;
mod parsing;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn dist_folder_created() {
        parsing::input::finalize_dist(vec![]).unwrap();
        assert!(Path::new("./dist").exists());
    }

    #[test]
    fn text_file_conversion() {
        parsing::input::process_arguments(&["test.txt".to_owned()]).unwrap();
        parsing::input::finalize_dist(vec!["test.txt".to_owned()]).unwrap();
        assert!(Path::new("./dist/test.html").exists());
    }

    #[test]
    fn markdown_file_conversion() {
        unimplemented!();
    }

    #[test]
    fn directory_conversion() {
        unimplemented!();
    }
}
