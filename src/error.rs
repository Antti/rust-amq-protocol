#[cfg(windows)]
impl From<::std::io::Error> for Error {
    fn from(s: ::std::io::Error) -> Self {
        s.into()
    }
}

error_chain! {
    errors {
        Protocol(t: String) {
            description("protocol error")
            display("protocol error: '{}'", t)
        }
    }

    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
    }
}
