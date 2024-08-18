struct Formatter {
    code: String,
}

impl Formatter {
    fn format(self) -> String {
        self.code
    }

    pub fn new(code: String, _args: FormatterArguments) -> Self {
        Formatter { code }
    }
}

/// is_sexp function returns true if line's first charactor is left bracket.
/// ```
/// use lisp_fmt_rs::is_sexp;
///
/// assert!(is_sexp(String::from("(print 'Alice')")));
/// assert!(!is_sexp(String::from("print 'This line is missing a left bracket')")));
/// assert!(!is_sexp(String::from("# This is emacs-lisp's comment")));
/// assert!(!is_sexp(String::from(";; This is commonlisp's comment")));
/// ```
pub fn is_sexp(line: String) -> bool {
    todo!()
}


struct FormatterArguments {}

impl FormatterArguments {
    fn new() -> Self {
        FormatterArguments {}
    }
}

#[cfg(test)]
mod tests {
    use super::{Formatter, FormatterArguments};

    #[test]
    fn nothing_to_change() {
        let code: String = String::from("(print 'Hello')");
        let formatter_args: FormatterArguments = FormatterArguments::new();
        let formatter: Formatter = Formatter::new(code, formatter_args);
        let result: String = formatter.format();
        assert_eq!(result, "(print 'Hello')");
    }
}
