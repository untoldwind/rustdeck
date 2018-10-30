error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        DBus(::dbus::Error);
    }

    errors {
        Hidapi(t: ::hidapi::HidError) {
            description("hidapi error")
            display("hidapi error: '{}'", t)
        }

        NoSerial {
            description("device has no serial number")
            display("device has no serial number")
        }

        InvalidColorFormat {
            description("invalid color format")
            display("invalid color format")
        }
    }
}

impl From<::hidapi::HidError> for Error {
    fn from(hid_error: ::hidapi::HidError) -> Error {
        Error::from_kind(ErrorKind::Hidapi(hid_error))
    }
}
