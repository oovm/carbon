use lazy_static::{self, LazyStatic};
use syntect::dumps::from_binary;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use lazy_static::lazy::Lazy;
use std::fmt::{Debug, Formatter};
use std::fmt;

#[allow(missing_copy_implementations)]
#[allow(dead_code)]
pub struct SyntaxDump {
    private_field: (),
}

#[allow(missing_copy_implementations)]
#[allow(dead_code)]
pub struct ThemeDump {
    private_field: (),
}

pub static SYNTAX_SET: SyntaxDump = SyntaxDump {
    private_field: (),
};

pub static THEME_SET: ThemeDump = ThemeDump {
    private_field: (),
};

impl ::lazy_static::__Deref for SyntaxDump {
    type Target = SyntaxSet;
    fn deref(&self) -> &SyntaxSet {
        #[inline(always)]
        fn __static_ref_initialize() -> SyntaxSet {
            from_binary(include_bytes!("../languages.dump"))
        }
        #[inline(always)]
        fn __stability() -> &'static SyntaxSet {
            static LAZY: Lazy<SyntaxSet> = Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}

impl ::lazy_static::__Deref for ThemeDump {
    type Target = ThemeSet;
    fn deref(&self) -> &ThemeSet {
        #[inline(always)]
        fn __static_ref_initialize() -> ThemeSet {
            from_binary(include_bytes!("../themes.dump"))
        }
        #[inline(always)]
        fn __stability() -> &'static ThemeSet {
            static LAZY: Lazy<ThemeSet> = Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}


impl LazyStatic for SyntaxDump {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}

impl LazyStatic for ThemeDump {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}

impl Debug for SyntaxDump {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.syntaxes().iter().map(|s| (s.name.clone(), s.file_extensions.clone()))).finish()
    }
}

impl Debug for ThemeDump {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.themes.iter().map(|(name, _)| name)).finish()
    }
}

#[test]
fn print() {
    println!("{:#?}", SYNTAX_SET);
    println!("{:#?}", THEME_SET);
}

