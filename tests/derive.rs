#[test]
fn test_that_mediaflow_file_can_be_derived_outside_of_project() {
    use mediaflow::MediaflowFile;
    #[derive(MediaflowFile)]
    struct MyFile {}
}

#[test]
fn test_that_mediaflow_folder_can_be_derived_outside_of_project() {
    use mediaflow::MediaflowFolder;
    #[derive(MediaflowFolder)]
    struct MyFolder {}
}

#[test]
fn test_that_mediaflow_file_download_can_be_derived_outside_of_project() {
    use mediaflow::MediaflowFileDownload;
    #[derive(MediaflowFileDownload)]
    struct MyFileDownload {}
}

#[test]
fn test_that_mediaflow_format_can_be_derived_outside_of_project() {
    use mediaflow::MediaflowFormat;
    #[derive(MediaflowFormat)]
    struct MyFormat {}
}
