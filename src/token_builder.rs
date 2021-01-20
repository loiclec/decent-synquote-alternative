use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};

pub trait TokenBuilderExtend {
    fn add_to(&self, tb: &mut TokenBuilder);
}
impl TokenBuilderExtend for Ident {
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        tb.extend_tree(self.clone());
    }
}
impl TokenBuilderExtend for Literal {
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        tb.extend_tree(self.clone());
    }
}
impl TokenBuilderExtend for Group {
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        tb.extend_tree(self.clone());
    }
}
impl TokenBuilderExtend for Punct {
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        tb.extend_tree(self.clone());
    }
}
impl TokenBuilderExtend for TokenTree {
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        tb.extend_tree(self.clone());
    }
}
impl TokenBuilderExtend for TokenStream {
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        tb.stream(self.clone());
    }
}
impl TokenBuilderExtend for usize {
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        tb.extend_tree(Literal::usize_unsuffixed(*self));
    }
}
impl TokenBuilderExtend for f64 {
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        tb.extend_tree(Literal::f64_suffixed(*self));
    }
}
impl<T> TokenBuilderExtend for Option<T>
where
    T: TokenBuilderExtend,
{
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        if let Some(x) = self {
            x.add_to(tb)
        }
    }
}
impl<T> TokenBuilderExtend for Vec<T>
where
    T: TokenBuilderExtend,
{
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        for x in self.iter() {
            x.add_to(tb)
        }
    }
}
impl TokenBuilderExtend for String {
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        tb.add(self)
    }
}
impl TokenBuilderExtend for &str {
    #[inline(never)]
    fn add_to(&self, tb: &mut TokenBuilder) {
        tb.add(self)
    }
}

pub struct TokenBuilder {
    pub groups: Vec<(Delimiter, TokenStream)>,
}

impl TokenBuilder {
    #[inline(never)]
    pub fn new() -> Self {
        Self {
            groups: vec![(Delimiter::None, TokenStream::new())],
        }
    }

    #[inline(never)]
    pub fn end(mut self) -> TokenStream {
        if self.groups.len() != 1 {
            panic!("Groups not empty, you missed a pop_group")
        }
        self.groups.pop().unwrap().1
    }

    // #[inline(never)]
    // pub fn eprint(&self) {
    //     eprintln!("{}", self.groups.last().unwrap().1.to_string());
    // }

    #[inline(never)]
    pub fn extend_tree<T: Into<TokenTree>>(&mut self, tt: T) {
        self.groups.last_mut().unwrap().1.extend(Some(tt.into()));
    }

    #[inline(never)]
    pub fn extend<T: TokenBuilderExtend>(&mut self, x: &T) {
        x.add_to(self)
    }

    #[inline(never)]
    pub fn stream(&mut self, what: TokenStream) {
        for c in what.into_iter() {
            self.extend(&c);
        }
    }

    #[inline(never)]
    pub fn add(&mut self, what: &str) {
        fn is_delimiter(c: char) -> bool {
            matches!(c, '{' | '(' | '[' | '}' | ')' | ']')
        }

        let delimiter_matches = what.matches(is_delimiter);
        let mut between_delimiter_matches = what.split(is_delimiter);

        if let Some(first_match_no_delimiter) = between_delimiter_matches.next() {
            let ts = first_match_no_delimiter.parse::<TokenStream>().expect(&format!(
                "Could not parse the following string into a token stream: {}",
                first_match_no_delimiter
            ));
            self.extend(&ts);
        }

        for (delimiter, ts) in delimiter_matches.zip(between_delimiter_matches) {
            match delimiter {
                "{" => self.push_group(Delimiter::Brace),
                "(" => self.push_group(Delimiter::Parenthesis),
                "[" => self.push_group(Delimiter::Bracket),
                "}" => self.pop_group(Delimiter::Brace),
                ")" => self.pop_group(Delimiter::Parenthesis),
                "]" => self.pop_group(Delimiter::Bracket),
                _ => unreachable!(),
            }
            let ts = ts.parse::<TokenStream>().expect(&format!(
                "Could not parse the following string into a token stream: {}",
                ts
            ));
            self.extend(&ts);
        }
    }

    #[inline(never)]
    pub fn push_group(&mut self, delim: Delimiter) {
        self.groups.push((delim, TokenStream::new()));
    }

    #[inline(never)]
    pub fn pop_group(&mut self, delim: Delimiter) {
        if self.groups.len() < 2 {
            // eprintln!("Stack dump for error:\n{}", self.stack_as_string());
            panic!("pop_group stack is empty {}", self.groups.len());
        }
        let ts = self.groups.pop().unwrap();
        if ts.0 != delim {
            // eprintln!("Stack dump for error:\n{}", self.stack_as_string());
            panic!("pop_group Delimiter mismatch, got {:?} expected {:?}", ts.0, delim);
        }
        self.extend_tree(TokenTree::from(Group::new(delim, ts.1)));
    }
}
