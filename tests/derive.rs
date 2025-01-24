#[test]
fn test_that_mediaflow_file_can_be_derived_outside_of_project() {
    use mediaflow::MediaflowFile;
    #[derive(MediaflowFile)]
    #[allow(unused)]
    struct MyFile {}
}

#[test]
fn test_that_mediaflow_folder_can_be_derived_outside_of_project() {
    use mediaflow::MediaflowFolder;
    #[derive(MediaflowFolder)]
    #[allow(unused)]
    struct MyFolder {}
}

#[test]
fn test_that_mediaflow_file_download_can_be_derived_outside_of_project() {
    use mediaflow::MediaflowFileDownload;
    #[derive(MediaflowFileDownload)]
    #[allow(unused)]
    struct MyFileDownload {}
}

#[test]
fn test_that_mediaflow_format_can_be_derived_outside_of_project() {
    use mediaflow::MediaflowFormat;
    #[derive(MediaflowFormat)]
    #[allow(unused)]
    struct MyFormat {}
}
