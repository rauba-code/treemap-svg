use std::process;

pub trait UnwrapOrExt<T> {
    fn unwrap_or_exit(self) -> T;
}

impl<T> UnwrapOrExt<T> for std::result::Result<T, String> {
    fn unwrap_or_exit(self) -> T {
        self.unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        })
    }
}

impl<T> UnwrapOrExt<T> for std::io::Result<T> {
    fn unwrap_or_exit(self) -> T {
        self.unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        })
    }
}