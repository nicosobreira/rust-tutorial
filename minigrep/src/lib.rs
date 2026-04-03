// FIX: The function wiil not panic if the "name" is a directory
// FIX: It also need to failed if the user doesn't have the permission to acess the file
fn file_open(name: &str) -> File {
    File::open(name).unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => {
            panic!("The file \"{}\" does not exist", name);
        }
        _ => panic!("{error}"),
    })
}
