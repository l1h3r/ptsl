use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::borrow::Cow;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_quote;
use syn::parse_str;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Attribute;
use syn::Data;
use syn::DeriveInput;
use syn::Error;
use syn::Expr;
use syn::Field;
use syn::Fields;
use syn::FieldsNamed;
use syn::Ident;
use syn::Result;
use syn::Stmt;
use syn::Token;
use syn::Type;
use syn::Variant;

use crate::ast::ExtType;
use crate::attrs::Once;
use crate::attrs::ParseAttr;

fn error(spanned: impl Spanned, error: &'static str) -> Error {
  Error::new(spanned.span(), error)
}

type Variants = Punctuated<Variant, Token![,]>;

// =============================================================================
// Protobuf Type
// =============================================================================

pub enum ProtoType {
  Data(ProtoData),
  Enum(ProtoEnum),
}

impl Parse for ProtoType {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    static ERR_STRUCT_TUPLE: &str = "Cannot use #[derive(ProtoType)] on tuple structs";
    static ERR_STRUCT_UNIT: &str = "Cannot use #[derive(ProtoType)] on unit structs";
    static ERR_UNION: &str = "Cannot use #[derive(ProtoType)] on unions";

    let input: DeriveInput = input.parse()?;

    match input.data {
      Data::Struct(inner) => match inner.fields {
        Fields::Named(fields) => ProtoData::parse(input.ident, fields).map(Self::Data),
        Fields::Unnamed(_) => Err(error(input.ident, ERR_STRUCT_TUPLE)),
        Fields::Unit => Err(error(input.ident, ERR_STRUCT_UNIT)),
      },
      Data::Enum(inner) => ProtoEnum::parse(input.ident, inner.variants).map(Self::Enum),
      Data::Union(_) => Err(error(input.ident, ERR_UNION)),
    }
  }
}

impl ToTokens for ProtoType {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match self {
      Self::Data(inner) => inner.to_tokens(tokens),
      Self::Enum(inner) => inner.to_tokens(tokens),
    }
  }
}

// =============================================================================
// Protobuf Enum
// =============================================================================

pub struct ProtoEnum {
  name: Ident,
  data: Vec<ProtoVariant>,
}

impl ProtoEnum {
  fn parse(name: Ident, variants: Variants) -> Result<Self> {
    Ok(Self {
      name,
      data: Self::parse_variants(variants)?,
    })
  }

  fn parse_variants(variants: Variants) -> Result<Vec<ProtoVariant>> {
    variants.into_iter().map(ProtoVariant::parse).collect()
  }

  fn variants(&self) -> Vec<&Ident> {
    self
      .data
      .iter()
      .filter(|variant| variant.include())
      .map(|variant| &variant.ident)
      .collect()
  }
}

impl ToTokens for ProtoEnum {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let name: &Ident = &self.name;
    let vars: Vec<&Ident> = self.variants();
    let size: usize = vars.len();

    // =========================================================================
    // Misc. Helpers
    // =========================================================================

    tokens.extend(quote! {
      impl #name {
        pub const SIZE: usize = #size;
        pub const LIST: [Self; Self::SIZE] = [#(Self::#vars),*];
      }

      impl ::core::fmt::Display for #name {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.write_str(self.as_str_name())
        }
      }
    });

    // =========================================================================
    // Impl TryFrom<&str>
    // =========================================================================

    tokens.extend(quote! {
      impl TryFrom<&'_ str> for #name {
        type Error = crate::error::Error;

        #[inline]
        fn try_from(other: &'_ str) -> Result<Self, Self::Error> {
          match Self::from_str_name(other) {
            Some(this) => Ok(this),
            None => Err(::prost::DecodeError::new("invalid enumeration value").into()),
          }
        }
      }
    });

    // =========================================================================
    // Impl Serialize
    // =========================================================================

    let serialize: TokenStream = quote! {
      ::serde::Serialize::serialize(&i32::from(*self), serializer)
    };

    tokens.extend(to_serialize(name, serialize));

    // =========================================================================
    // Impl Deserialize
    // =========================================================================

    let visit_i32: TokenStream = visit_i32(quote! {
      #name::try_from(value).map_err(|_| {
        ::serde::de::Error::invalid_value(::serde::de::Unexpected::Signed(value.into()), &self)
      })
    });

    let visit_str: TokenStream = visit_str(quote! {
      #name::try_from(value).map_err(|_| {
        ::serde::de::Error::invalid_value(::serde::de::Unexpected::Str(value), &self)
      })
    });

    let visitor: TokenStream = quote!(#visit_i32 #visit_str);
    let visitor: TokenStream = to_visitor(name, "string or integer", visitor);

    let deserialize: TokenStream = to_deserialize(
      name,
      quote! {
        #visitor
        deserializer.deserialize_any(Visitor)
      },
    );

    tokens.extend(deserialize);
  }
}

// =============================================================================
// Protobuf Enum Variant
// =============================================================================

struct ProtoVariant {
  ident: Ident,
}

impl ProtoVariant {
  fn parse(input: Variant) -> Result<Self> {
    static ERR_NAMED: &str = "Cannot use #[derive(ProtoType)] with named fields";
    static ERR_UNNAMED: &str = "Cannot use #[derive(ProtoType)] with unnamed fields";
    static ERR_DISCRIMINANT_LITERAL: &str = "Variant must have a literal discriminant";
    static ERR_DISCRIMINANT_MISSING: &str = "Variant must have a discriminant";

    match input.fields {
      Fields::Named(_) => Err(error(input.ident, ERR_NAMED)),
      Fields::Unnamed(_) => Err(error(input.ident, ERR_UNNAMED)),
      Fields::Unit => match input.discriminant {
        Some((_, Expr::Lit(_))) => Ok(Self { ident: input.ident }),
        Some((_, _)) => Err(error(input.ident, ERR_DISCRIMINANT_LITERAL)),
        None => Err(error(input.ident, ERR_DISCRIMINANT_MISSING)),
      },
    }
  }

  fn include(&self) -> bool {
    self.ident != "BitNone"
  }
}

// =============================================================================
// Protobuf Message
// =============================================================================

pub struct ProtoData {
  name: Ident,
  data: Vec<ProtoField>,
}

impl ProtoData {
  fn parse(name: Ident, fields: FieldsNamed) -> Result<Self> {
    Ok(Self {
      name,
      data: Self::parse_fields(fields)?,
    })
  }

  fn parse_fields(fields: FieldsNamed) -> Result<Vec<ProtoField>> {
    fields.named.into_iter().map(ProtoField::parse).collect()
  }

  fn param_names(&self) -> Vec<&Ident> {
    self.data.iter().map(ProtoField::ident).collect()
  }

  fn param_types(&self) -> Vec<Type> {
    self.data.iter().map(ProtoField::param_type).collect()
  }

  fn param_intos(&self) -> Vec<Expr> {
    self.data.iter().map(ProtoField::param_into).collect()
  }

  fn field_names(&self) -> Vec<String> {
    self.data.iter().map(ProtoField::name).collect()
  }

  fn field_types(&self) -> Vec<Cow<'_, Type>> {
    self.data.iter().map(ProtoField::field_type).collect()
  }

  fn field_unwraps(&self) -> Vec<Stmt> {
    self.data.iter().map(ProtoField::field_unwrap).collect()
  }

  fn serializers(&self) -> Vec<Expr> {
    self.data.iter().map(ProtoField::serializer).collect()
  }

  fn deserializers(&self) -> Vec<Expr> {
    self.data.iter().map(ProtoField::deserializer).collect()
  }
}

impl ToTokens for ProtoData {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let name: &Ident = &self.name;

    let param_names: Vec<&Ident> = self.param_names();
    let param_types: Vec<Type> = self.param_types();
    let param_intos: Vec<Expr> = self.param_intos();

    let struct_name: String = name.to_string();
    let struct_size: usize = self.data.len();

    let field_names: Vec<String> = self.field_names();
    let field_types: Vec<Cow<'_, Type>> = self.field_types();
    let field_unwrap: Vec<Stmt> = self.field_unwraps();

    let field_serializer: Vec<Expr> = self.serializers();
    let field_deserializer: Vec<Expr> = self.deserializers();

    // =========================================================================
    // Misc. Helpers
    // =========================================================================

    tokens.extend(quote! {
      impl #name {
        #[allow(clippy::too_many_arguments)]
        pub fn new(#(#param_names: #param_types),*) -> Self {
          Self {
            #(#param_names: #param_intos),*
          }
        }
      }
    });

    // =========================================================================
    // Impl Serialize
    // =========================================================================

    let serialize: TokenStream = quote! {
      let mut state: S::SerializeStruct = ::serde::Serializer::serialize_struct(
        serializer,
        #struct_name,
        #struct_size,
      )?;

      #(
        ::serde::ser::SerializeStruct::serialize_field(&mut state, #field_names, &#field_serializer)?;
      )*

      ::serde::ser::SerializeStruct::end(state)
    };

    tokens.extend(to_serialize(name, serialize));

    // =========================================================================
    // Impl Deserialize
    // =========================================================================

    let visit_map: TokenStream = visit_map(quote! {
      #(
        let mut #param_names: Option<#field_types> = None;
      )*

      while let Some(key) = access.next_key::<Field>()? {
        match key {
          #(
            Field::#param_names => {
              if #param_names.is_some() {
                return Err(::serde::de::Error::duplicate_field(#field_names));
              }

              #param_names = Some(#field_deserializer);
            }
          )*
        }
      }

      #(#field_unwrap)*

      Ok(Self::Value {
        #(#param_names),*
      })
    });

    let visitor: TokenStream = to_visitor(name, &format!("struct {struct_name}"), visit_map);

    let deserialize: TokenStream = to_deserialize(
      name,
      quote! {
        #[allow(non_camel_case_types)]
        #[derive(::serde::Deserialize)]
        #[serde(field_identifier)]
        enum Field { #(#param_names),* }

        #visitor

        const FIELDS: &'static [&'static str] = &[#(#field_names),*];
        deserializer.deserialize_struct(#struct_name, FIELDS, Visitor)
      },
    );

    tokens.extend(deserialize);
  }
}

// =============================================================================
// Protobuf Message Field
// =============================================================================

enum ProtoField {
  Type(ProtoFieldType),
  Enum(ProtoFieldEnum),
}

impl ProtoField {
  fn parse(input: Field) -> Result<Self> {
    let attr: Attributes = Attributes::parse(&input.attrs)?;
    let ident: Ident = input.ident.expect("named field without name");
    let label: Option<Label> = Label::new(&ident, &attr)?;

    if let Some(enumeration) = attr.enumeration {
      Ok(Self::Enum(ProtoFieldEnum {
        label,
        ident,
        rtype: parse_str(&enumeration)?,
      }))
    } else {
      Ok(Self::Type(ProtoFieldType {
        label,
        ident,
        rtype: input.ty,
      }))
    }
  }

  fn name(&self) -> String {
    self.ident().to_string()
  }

  const fn ident(&self) -> &Ident {
    match self {
      Self::Type(inner) => &inner.ident,
      Self::Enum(inner) => &inner.ident,
    }
  }

  fn param_type(&self) -> Type {
    match self {
      Self::Type(inner) => inner.param_type(),
      Self::Enum(inner) => inner.param_type(),
    }
  }

  fn param_into(&self) -> Expr {
    match self {
      Self::Type(inner) => inner.param_into(),
      Self::Enum(inner) => inner.param_into(),
    }
  }

  fn field_type(&self) -> Cow<'_, Type> {
    match self {
      Self::Type(inner) => inner.field_type(),
      Self::Enum(inner) => inner.field_type(),
    }
  }

  fn field_unwrap(&self) -> Stmt {
    let param_name: &Ident = self.ident();
    let field_name: String = param_name.to_string();
    let field_type: Cow<'_, Type> = self.field_type();
    let is_default: bool = field_name == "Pagination";

    if is_default {
      parse_quote! {
        let #param_name: #field_type = #param_name.unwrap_or_default();
      }
    } else {
      parse_quote! {
        let Some(#param_name): Option<#field_type> = #param_name else {
          return Err(::serde::de::Error::missing_field(#field_name));
        };
      }
    }
  }

  fn serializer(&self) -> Expr {
    match self {
      Self::Type(inner) => inner.serializer(),
      Self::Enum(inner) => inner.serializer(),
    }
  }

  fn deserializer(&self) -> Expr {
    match self {
      Self::Type(inner) => inner.deserializer(),
      Self::Enum(inner) => inner.deserializer(),
    }
  }
}

// =============================================================================
// Protobuf Field (default)
// =============================================================================

struct ProtoFieldType {
  label: Option<Label>,
  ident: Ident,
  rtype: Type,
}

impl ProtoFieldType {
  const fn etype(&self) -> ExtType<'_> {
    ExtType::new(&self.rtype)
  }

  fn param_type(&self) -> Type {
    let rtype: &Type = self.etype().trailer();

    if let Some(Label::Repeated) = self.label {
      parse_quote!(impl IntoIterator<Item = #rtype>)
    } else {
      parse_quote!(#rtype)
    }
  }

  fn param_into(&self) -> Expr {
    let ident: &Ident = &self.ident;

    if let Some(Label::Repeated) = self.label {
      parse_quote!(#ident.into_iter().collect())
    } else {
      parse_quote!(#ident)
    }
  }

  const fn field_type(&self) -> Cow<'_, Type> {
    Cow::Borrowed(&self.rtype)
  }

  fn serializer(&self) -> Expr {
    let ident: &Ident = &self.ident;
    parse_quote!(self.#ident)
  }

  fn deserializer(&self) -> Expr {
    parse_quote!(access.next_value()?)
  }
}

// =============================================================================
// Protobuf Field (enum)
// =============================================================================

struct ProtoFieldEnum {
  label: Option<Label>,
  ident: Ident,
  rtype: Type,
}

impl ProtoFieldEnum {
  const fn etype(&self) -> ExtType<'_> {
    ExtType::new(&self.rtype)
  }

  fn param_type(&self) -> Type {
    let rtype: &syn::Type = self.etype().trailer();

    if let Some(Label::Repeated) = self.label {
      parse_quote!(impl IntoIterator<Item = #rtype>)
    } else {
      parse_quote!(#rtype)
    }
  }

  fn param_into(&self) -> Expr {
    let ident: &Ident = &self.ident;

    if let Some(Label::Repeated) = self.label {
      parse_quote!(#ident.into_iter().map(Into::into).collect())
    } else {
      parse_quote!(#ident.into())
    }
  }

  fn field_type(&self) -> Cow<'_, Type> {
    if let Some(Label::Repeated) = self.label {
      Cow::Owned(parse_quote!(Vec<i32>))
    } else {
      Cow::Owned(parse_quote!(i32))
    }
  }

  fn serializer(&self) -> syn::Expr {
    let ident: &Ident = &self.ident;
    let rtype: &Type = &self.rtype;

    if let Some(Label::Repeated) = self.label {
      parse_quote! {
        self.#ident().map(|variant| variant.as_str_name()).collect::<Vec<_>>()
      }
    } else {
      parse_quote! {
        #rtype::from_i32(self.#ident).map(|variant| variant.as_str_name())
      }
    }
  }

  fn deserializer(&self) -> syn::Expr {
    let rtype: &Type = &self.rtype;

    if let Some(Label::Repeated) = self.label {
      // TODO: Avoid additional Vec<T> allocation
      parse_quote! {
        access.next_value::<Vec<#rtype>>()?.into_iter().map(Into::into).collect()
      }
    } else {
      parse_quote! {
        access.next_value::<#rtype>().map(Into::into)?
      }
    }
  }
}

// =============================================================================
// Protobuf Field Label
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Label {
  Optional,
  Repeated,
}

impl Label {
  fn new(ident: &Ident, attributes: &Attributes) -> Result<Option<Self>> {
    match (attributes.repeated, attributes.optional) {
      (true, true) => Err(error(ident, "Field cannot be repeated AND optional")),
      (true, false) => Ok(Some(Self::Repeated)),
      (false, true) => Ok(Some(Self::Optional)),
      (false, false) => Ok(None),
    }
  }
}

// =============================================================================
// Protobuf Field Attributes
// =============================================================================

struct Attributes {
  enumeration: Option<String>,
  repeated: bool,
  optional: bool,
}

impl ParseAttr for Attributes {
  const NAME: &'static str = "prost";
  const DATA: &'static [&'static str] = &["enumeration", "repeated", "optional"];

  fn parse(attributes: &[Attribute]) -> Result<Self> {
    let mut enumeration: Once<String> = Once::None;
    let mut repeated: Once<bool> = Once::None;
    let mut optional: Once<bool> = Once::None;

    Self::parse_inner(attributes, |item| match item.name() {
      "enumeration" => enumeration.try_once(|| item.parse()),
      "repeated" => repeated.try_once(|| item.parse()),
      "optional" => optional.try_once(|| item.parse()),
      _ => unreachable!(),
    })?;

    Ok(Self {
      enumeration: enumeration.into_option(),
      repeated: repeated.unwrap_or_default(),
      optional: optional.unwrap_or_default(),
    })
  }
}

// =============================================================================
// Misc. Helpers
// =============================================================================

fn to_serialize(name: &Ident, body: TokenStream) -> TokenStream {
  quote! {
    impl ::serde::Serialize for #name {
      fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
      where
        S: ::serde::Serializer,
      {
        #body
      }
    }
  }
}

fn to_deserialize(name: &Ident, body: TokenStream) -> TokenStream {
  quote! {
    impl<'de> ::serde::Deserialize<'de> for #name {
      fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
      where
        D: ::serde::Deserializer<'de>,
      {
        #body
      }
    }
  }
}

fn to_visitor(name: &Ident, expecting: &str, body: TokenStream) -> TokenStream {
  quote! {
    struct Visitor;

    impl<'de> ::serde::de::Visitor<'de> for Visitor {
      type Value = #name;

      fn expecting(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(#expecting)
      }

      #body
    }
  }
}

fn visit_i32(body: TokenStream) -> TokenStream {
  quote! {
    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
      E: ::serde::de::Error,
    {
      #body
    }
  }
}

fn visit_str(body: TokenStream) -> TokenStream {
  quote! {
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
      E: ::serde::de::Error,
    {
      #body
    }
  }
}

fn visit_map(body: TokenStream) -> TokenStream {
  quote! {
    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
      A: ::serde::de::MapAccess<'de>,
    {
      #body
    }
  }
}
