use std::{
    borrow::Cow,
    io::{Error as IoError, ErrorKind as IoErrorKind},
    marker::PhantomData,
};

struct English;
struct Otaku;

pub struct Message<T> {
    msg: Cow<'static, str>,
    _useless: PhantomData<T>,
}

impl Message<()> {
    pub fn io_error(e: IoError, otaku: bool) -> Box<dyn std::fmt::Display> {
        if otaku {
            Box::new(Message::<Otaku>::from(e))
        } else {
            Box::new(Message::<English>::from(e))
        }
    }

    pub fn not_a_regular_file(otaku: bool) -> Box<dyn std::fmt::Display> {
        if otaku {
            Box::new(Message::<Otaku>::NOT_A_REGULAR_FILE)
        } else {
            Box::new(Message::<English>::NOT_A_REGULAR_FILE)
        }
    }

    pub fn non_utf8_content(otaku: bool) -> Box<dyn std::fmt::Display> {
        if otaku {
            Box::new(Message::<Otaku>::NON_UTF8_CONTENT)
        } else {
            Box::new(Message::<English>::NON_UTF8_CONTENT)
        }
    }

    pub fn file_contains_crlf_line_endings(otaku: bool) -> Box<dyn std::fmt::Display> {
        if otaku {
            Box::new(Message::<Otaku>::FILE_CONTAINS_CRLF_LINE_ENDINGS)
        } else {
            Box::new(Message::<English>::FILE_CONTAINS_CRLF_LINE_ENDINGS)
        }
    }

    pub fn please_fix(otaku: bool) -> Box<dyn std::fmt::Display> {
        if otaku {
            Box::new(Message::<Otaku>::PLEASE_FIX)
        } else {
            Box::new(Message::<English>::PLEASE_FIX)
        }
    }
}

impl Message<English> {
    const NOT_A_REGULAR_FILE: Self = Self {
        msg: Cow::Borrowed("Not a regular file"),
        _useless: PhantomData,
    };

    const NON_UTF8_CONTENT: Self = Self {
        msg: Cow::Borrowed("Non-UTF-8 content"),
        _useless: PhantomData,
    };

    const FILE_CONTAINS_CRLF_LINE_ENDINGS: Self = Self {
        msg: Cow::Borrowed("File contains CRLF line endings"),
        _useless: PhantomData,
    };

    const PLEASE_FIX: Self = Self {
        msg: Cow::Borrowed("File doesn't end with LF, re-run without -c/--check to fix it"),
        _useless: PhantomData,
    };
}

impl Message<Otaku> {
    const NOT_A_REGULAR_FILE: Self = Self {
        msg: Cow::Borrowed("哼！(￣へ￣) 这破文件一点都不正经！"),
        _useless: PhantomData,
    };

    const NON_UTF8_CONTENT: Self = Self {
        msg: Cow::Borrowed("哼！(￣へ￣) 不是UTF-8的坏文件！"),
        _useless: PhantomData,
    };

    const FILE_CONTAINS_CRLF_LINE_ENDINGS: Self = Self {
        msg: Cow::Borrowed(
            "啊啦~ (。-`ω´-) 居然敢用 Windows 的换行符，看来这个文件需要好好\"教育\"一下呢！",
        ),
        _useless: PhantomData,
    };

    const PLEASE_FIX: Self = Self {
        msg: Cow::Borrowed("呜呼~ (。-`ω´-) 又遇到没有 LF 结尾的文件啦？再运行一次就能修好哦，不要用 -c/--check 参数哦！"),
        _useless: PhantomData,
    };
}

impl From<IoError> for Message<English> {
    fn from(e: IoError) -> Self {
        Self {
            msg: Cow::Owned(e.to_string()),
            _useless: PhantomData,
        }
    }
}

impl From<IoError> for Message<Otaku> {
    fn from(e: IoError) -> Self {
        match e.kind() {
            IoErrorKind::NotFound => Self {
                msg: Cow::Borrowed("呜哇~文件酱不见啦！(´;ω;｀) 一定是害羞躲起来啦~"),
                _useless: PhantomData,
            },
            IoErrorKind::PermissionDenied => Self {
                msg: Cow::Borrowed(
                    "噗噗~！(￣▽￣*) 权限酱傲娇地说\"不行就是不行\"！你这个没权限的小可爱~",
                ),
                _useless: PhantomData,
            },
            _ => Self {
                msg: Cow::Owned(e.to_string()),
                _useless: PhantomData,
            },
        }
    }
}

impl<T> std::fmt::Display for Message<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
