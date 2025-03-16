
#[const_trait]
pub trait ProvideTraitNames {
    fn trait_names(&self) -> TraitNames;
}


#[marker]
pub trait CanNameTrait {}
impl<I> CanNameTrait for I where I: TraitImpl_10 {}
impl<I> CanNameTrait for I where I: TraitImpl_11 {}
impl<I> CanNameTrait for I where I: TraitImpl_12f {}
impl<I> CanNameTrait for I where I: TraitImpl_12i {}
impl<I> CanNameTrait for I where I: TraitImpl_21 {}
impl<I> CanNameTrait for I where I: TraitImpl_22 {}

#[const_trait]
pub trait NameTrait: Sized + Copy {
    fn new_trait_named(self, name: &'static str) -> Elaborated<Self>;
}
impl<I> const NameTrait for I
where
    I: CanNameTrait + Copy,
{
    fn new_trait_named(self, name: &'static str) -> Elaborated<Self> {
        Elaborated::new_with_name(self, name)
    }
}

/// Each TraitKey should be the final name of a trait, and correspond
/// to exactly one trait item in rust. It should be in UpperCamelCase
/// with no funny business or special characters. It will be converted
/// to lowerCamelCase for output in shaders and lower_snake_case for
/// function names inside the trait item.
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub struct TraitKey {
    final_name: &'static str,
}

lazy_static! {
    static ref TRAIT_KEY_REGEX: Regex = Regex::new("^[A-Z][a-zA-Z0-9]+$").expect("TraitKey regex is valid");
}
impl TraitKey {
    pub const fn new(name: &'static str) -> Self {
        // Cannot validate with regex in const eval.
        // So we'll do validation at use sites instead.

        // assert!(
        //     TRAIT_KEY_REGEX.is_match(name),
        //     "TraitKeys must be UpperCamelCase without any funny business or special characters."
        // );
        Self { final_name: name }
    }

    pub fn as_upper_camel(&self) -> String {
        assert!(
            TRAIT_KEY_REGEX.is_match(self.final_name),
            "TraitKeys must be UpperCamelCase without any funny business or special characters."
        );
        self.final_name.to_string()
    }

    pub fn as_lower_camel(&self) -> String {
        assert!(
            TRAIT_KEY_REGEX.is_match(self.final_name),
            "TraitKeys must be UpperCamelCase without any funny business or special characters."
        );
        let n = self.final_name;
        let f = n[0..1].to_lowercase();
        let inal_name = n[1..n.len()].to_string();
        format!("{f}{inal_name}")
    }

    pub fn as_lower_snake(&self) -> String {
        assert!(
            TRAIT_KEY_REGEX.is_match(self.final_name),
            "TraitKeys must be UpperCamelCase without any funny business or special characters."
        );
        let mut snake = String::new();
        let mut chars = self.final_name.chars().peekable();

        while let Some(c) = chars.next() {
            if c.is_uppercase() {
                // If not the first character and the next character is not uppercase,
                // or the previous character is not uppercase, add an underscore.
                if !snake.is_empty() && (chars.peek().map_or(false, |next| !next.is_uppercase()) || snake.chars().last().map_or(false, |prev| !prev.is_uppercase())) {
                    snake.push('_');
                }
                for lowercase in c.to_lowercase() {
                    snake.push(lowercase);
                }
            } else {
                snake.push(c);
            }
        }

        snake
    }

    pub fn as_screaming_snake(&self) -> String {
        self.as_lower_snake().to_uppercase()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub struct TraitNames {
    pub trait_key: TraitKey,
    // ArrayVec does not allow quite enough const operations
    // pub aliases: ArrayVec<[TraitAlias; 16]>,
    pub aliases: [Option<TraitAlias>; 16],
}
impl TraitNames {
    pub const fn just(name: &'static str) -> Self {
        TraitNames {
            trait_key: TraitKey::new(name),
            aliases: [None; 16],
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub struct TraitAlias {
    pub alias_key: TraitKey,
    pub mention_in_documentation: bool,
    pub rust_trait_sharing: TraitSharing,
    pub output_in_shaders: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum TraitSharing {
    DontOutput,
    Consolidate,
    DistinctTrait,
}

impl TraitAlias {
    pub const fn usual(alias: &'static str) -> Self {
        Self::new(alias, true, TraitSharing::Consolidate, true)
    }
    pub const fn usual_except_shaders(alias: &'static str) -> Self {
        Self::new(alias, true, TraitSharing::Consolidate, false)
    }
    pub const fn docs_only(alias: &'static str) -> Self {
        Self::new(alias, true, TraitSharing::DontOutput, false)
    }
    pub const fn new(alias: &'static str, docs: bool, share: TraitSharing, shaders: bool) -> Self {
        TraitAlias {
            alias_key: TraitKey::new(alias),
            mention_in_documentation: docs,
            rust_trait_sharing: share,
            output_in_shaders: shaders,
        }
    }
}
