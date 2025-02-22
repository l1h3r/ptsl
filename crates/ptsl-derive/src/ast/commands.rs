use heck::ToSnekCase;
use heck::ToTitleCase;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use syn::braced;
use syn::parenthesized;
use syn::parse::Lookahead1;
use syn::parse::Parse;
use syn::parse::ParseBuffer;
use syn::parse::ParseStream;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::token;
use syn::Attribute;
use syn::Expr;
use syn::Ident;
use syn::Result;
use syn::Token;
use syn::Type;

use crate::ast::ExtType;
use crate::attrs::ParseAttr;
use crate::attrs::Once;

type CommandArgs = Punctuated<CommandArg, Token![,]>;

fn ident(string: impl AsRef<str>) -> Ident {
  Ident::new(string.as_ref(), Span::call_site())
}

// =============================================================================
// Custom Keywords
// =============================================================================

mod kw {
  syn::custom_keyword!(command);
  syn::custom_keyword!(support);
  syn::custom_keyword!(send);
  syn::custom_keyword!(recv);
  syn::custom_keyword!(ping);
  syn::custom_keyword!(sync);
}

// =============================================================================
// Command List
// =============================================================================

/// A list of commands.
pub struct CommandList {
  commands: Vec<Command>,
}

impl CommandList {
  fn extension(&self) -> ExtensionList<'_> {
    ExtensionList(&self.commands)
  }
}

impl Parse for CommandList {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    let mut commands: Vec<Command> = Vec::new();

    while !input.is_empty() {
      commands.push(Command::parse(input)?);
    }

    Ok(Self { commands })
  }
}

impl ToTokens for CommandList {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    for command in self.commands.iter() {
      command.to_tokens(tokens);
    }

    self.extension().to_tokens(tokens);
  }
}

// =============================================================================
// Command
// =============================================================================

pub struct Command {
  attr: Attributes,
  kind: CommandType,
  name: Ident,
  head: token::Paren,
  mode: CommandMode,
  tail: token::Brace,
  args: CommandArgs,
}

impl Command {
  fn send_type(&self) -> Type {
    self.mode.send_type(&self.name)
  }

  fn recv_type(&self) -> Type {
    self.mode.recv_type(&self.name)
  }

  fn param_names(&self) -> Vec<&Ident> {
    self.args.iter().map(|arg| &arg.name).collect()
  }

  fn param_kinds(&self) -> Vec<&Type> {
    self.args.iter().map(|arg| &arg.kind).collect()
  }

  fn param_inits(&self) -> Vec<Expr> {
    self.args.iter().map(|arg| arg.init()).collect()
  }

  fn param_features(&self) -> Vec<Option<Attribute>> {
    self.args.iter().map(|arg| arg.attr.feature()).collect()
  }

  const fn is_command(&self) -> bool {
    matches!(self.kind, CommandType::Command(_))
  }

  const fn is_support(&self) -> bool {
    matches!(self.kind, CommandType::Support(_))
  }

  const fn message(&self) -> Message<'_> {
    Message(self)
  }

  const fn request(&self) -> Request<'_> {
    Request(self)
  }

  const fn builder(&self) -> Builder<'_> {
    Builder(self)
  }

  const fn extension(&self) -> Extension<'_> {
    Extension(self)
  }
}

impl Parse for Command {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    let mode: ParseBuffer<'_>;
    let args: ParseBuffer<'_>;

    Ok(Self {
      attr: Attributes::parse_outer(input)?,
      kind: input.parse()?,
      name: input.parse()?,
      head: parenthesized!(mode in input),
      mode: CommandMode::parse(&mode)?,
      tail: braced!(args in input),
      args: Punctuated::parse_terminated(&args)?,
    })
  }
}

impl ToTokens for Command {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match self.kind {
      CommandType::Command(_) => {
        self.message().to_tokens(tokens);
        self.request().to_tokens(tokens);
        self.builder().to_tokens(tokens);
      }
      CommandType::Support(_) => {
        self.builder().to_tokens(tokens);
      }
    }
  }
}

// =============================================================================
// Command Type
// =============================================================================

enum CommandType {
  Command(kw::command),
  Support(kw::support),
}

impl Parse for CommandType {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    let lookahead: Lookahead1<'_> = input.lookahead1();

    if lookahead.peek(kw::command) {
      input.parse().map(Self::Command)
    } else if lookahead.peek(kw::support) {
      input.parse().map(Self::Support)
    } else {
      Err(lookahead.error())
    }
  }
}

// =============================================================================
// Command Mode
// =============================================================================

struct CommandMode {
  send: Option<kw::send>,
  recv: Option<kw::recv>,
  ping: Option<kw::ping>,
  sync: Option<kw::sync>,
}

impl CommandMode {
  const fn send(&self) -> bool {
    self.send.is_some()
  }

  const fn recv(&self) -> bool {
    self.recv.is_some()
  }

  const fn ping(&self) -> bool {
    self.ping.is_some()
  }

  const fn sync(&self) -> bool {
    self.sync.is_some()
  }

  fn send_name(&self, name: &Ident) -> Option<Ident> {
    let true = self.send() else {
      return None;
    };

    Some(ident(format!("{name}RequestBody")))
  }

  fn recv_name(&self, name: &Ident) -> Option<Ident> {
    let true = self.recv() else {
      return None;
    };

    Some(ident(format!("{name}ResponseBody")))
  }

  fn send_type(&self, name: &Ident) -> Type {
    self
      .send_name(name)
      .map(|base| parse_quote!(crate::types::#base))
      .unwrap_or_else(|| parse_quote!(()))
  }

  fn recv_type(&self, name: &Ident) -> Type {
    self
      .recv_name(name)
      .map(|base| parse_quote!(crate::types::#base))
      .unwrap_or_else(|| parse_quote!(()))
  }
}

impl Parse for CommandMode {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    let mut this: Self = Self {
      send: None,
      recv: None,
      ping: None,
      sync: None,
    };

    while !input.is_empty() {
      let lookahead: Lookahead1<'_> = input.lookahead1();

      if lookahead.peek(kw::ping) {
        this.ping = Some(input.parse()?);
      } else if lookahead.peek(kw::recv) {
        this.recv = Some(input.parse()?);
      } else if lookahead.peek(kw::send) {
        this.send = Some(input.parse()?);
      } else if lookahead.peek(kw::sync) {
        this.sync = Some(input.parse()?);
      } else {
        return Err(lookahead.error());
      }

      if input.is_empty() {
        break;
      }

      let _unused: Token![,] = input.parse()?;
    }

    Ok(this)
  }
}

// =============================================================================
// Command Arg
// =============================================================================

struct CommandArg {
  attr: Attributes,
  name: Ident,
  skip: Token![:],
  kind: Type,
}

impl CommandArg {
  fn init(&self) -> Expr {
    ExtType::new(&self.kind).initializer()
  }
}

impl Parse for CommandArg {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    Ok(Self {
      attr: Attributes::parse_outer(input)?,
      name: input.parse()?,
      skip: input.parse()?,
      kind: input.parse()?,
    })
  }
}

// =============================================================================
// Command Attributes
// =============================================================================

struct Attributes {
  feature: Option<String>,
}

impl Attributes {
  fn feature(&self) -> Option<Attribute> {
    if let Some(ref feature) = self.feature {
      Some(parse_quote!(#[cfg(feature = #feature)]))
    } else {
      None
    }
  }
}

impl ParseAttr for Attributes {
  const NAME: &'static str = "cfg";
  const DATA: &'static [&'static str] = &["feature"];

  fn parse(attributes: &[Attribute]) -> Result<Self> {
    let mut feature: Once<String> = Once::None;

    Self::parse_inner(attributes, |item| match item.name() {
      "feature" => feature.try_once(|| item.parse()),
      _ => unreachable!(),
    })?;

    Ok(Self {
      feature: feature.into_option(),
    })
  }
}

// =============================================================================
// Command Message
// =============================================================================

struct Message<'a>(&'a Command);

impl ToTokens for Message<'_> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let name: &Ident = &self.0.name;
    let send: Type = self.0.send_type();
    let recv: Type = self.0.recv_type();
    let ping: bool = self.0.mode.ping();
    let sync: bool = self.0.mode.sync();
    let docs: Documentation<'_> = Documentation::Message(name);
    let feat: Option<Attribute> = self.0.attr.feature();

    tokens.extend(quote! {
      #feat
      #[doc = #docs]
      #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
      pub struct #name;

      #feat
      impl crate::traits::Message for #name {
        const TYPE: crate::types::CommandId = crate::types::CommandId::#name;
        const SEND_PINGS: bool = #ping;
        const VIA_STREAM: bool = !#sync;

        type Send = #send;
        type Recv = #recv;
      }
    });
  }
}

// =============================================================================
// Command Request
// =============================================================================

struct Request<'a>(&'a Command);

impl ToTokens for Request<'_> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    if !self.0.mode.send() {
      return;
    }

    let name: &Ident = &self.0.name;
    let send: Type = self.0.send_type();
    let recv: Type = self.0.recv_type();
    let feat: Option<Attribute> = self.0.attr.feature();

    let param_names: Vec<&Ident> = self.0.param_names();
    let param_kinds: Vec<&Type> = self.0.param_kinds();
    let param_feats: Vec<Option<Attribute>> = self.0.param_features();

    let send_name: Option<Ident> = self.0.mode.send_name(name);
    let docs_init: Documentation<'_> = Documentation::Request(name, send_name);

    tokens.extend(quote! {
      #feat
      impl #send {
        /// Use `client` to send the request to the PTSL server.
        #[inline]
        pub async fn send<T>(self, client: &mut T) -> ::core::result::Result<#recv, T::Error>
        where
          T: CommandExt,
        {
          client.send::<#name>(self).await
        }
      }

      #feat
      impl #name {
        #[doc = #docs_init]
        #[inline]
        pub fn request(#(#param_feats #param_names: #param_kinds),*) -> #send {
          #send::new(#(#param_feats #param_names),*)
        }
      }
    });
  }
}

// =============================================================================
// Command Builder
// =============================================================================

struct Builder<'a>(&'a Command);

impl Builder<'_> {
  fn enabled(&self) -> bool {
    // TODO
    true
  }

  fn builder_name(&self) -> Option<Ident> {
    let name: &Ident = &self.0.name;

    if self.0.is_support() {
      Some(ident(format!("{name}Builder")))
    } else if self.0.mode.send() {
      Some(ident(format!("{name}RequestBuilder")))
    } else {
      None
    }
  }

  fn builder_type(&self) -> Option<TokenStream> {
    if self.0.is_support() {
      Some(self.0.name.to_token_stream())
    } else if self.0.mode.send() {
      Some(self.0.send_type().to_token_stream())
    } else {
      None
    }
  }
}

impl ToTokens for Builder<'_> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    if !self.enabled() {
      return;
    }

    let Some(builder_name) = self.builder_name() else {
      return;
    };

    let Some(builder_type) = self.builder_type() else {
      return;
    };

    let name: &Ident = &self.0.name;
    let feat: Option<Attribute> = self.0.attr.feature();

    let param_names: Vec<&Ident> = self.0.param_names();
    let param_kinds: Vec<&Type> = self.0.param_kinds();
    let param_inits: Vec<Expr> = self.0.param_inits();
    let param_feats: Vec<Option<Attribute>> = self.0.param_features();

    let param_docs = self
      .0
      .args
      .iter()
      .map(|arg| Documentation::Builder(&arg.name));

    tokens.extend(quote! {
      #feat
      /// Convenience type for creating a new request.
      #[derive(Clone, Debug)]
      pub struct #builder_name {
        #(#param_feats #param_names: #param_kinds),*
      }

      #feat
      impl #builder_name {
        /// Create a new builder.
        #[inline]
        pub fn new() -> Self {
          Self {
            #(#param_feats #param_names: #param_inits),*
          }
        }

        #(
          #param_feats
          #[doc = #param_docs]
          #[inline]
          pub fn #param_names<T: Into<#param_kinds>>(mut self, value: T) -> Self {
            self.#param_names = value.into();
            self
          }
        )*

        /// Consume the builder and return the constructed type.
        #[inline]
        pub fn build(self) -> #builder_type {
          #builder_type::new(#(#param_feats self.#param_names),*)
        }
      }

      #feat
      impl #name {
        /// Create a new builder.
        #[inline]
        pub fn builder() -> #builder_name {
          #builder_name::new()
        }

        /// Shortcut for [`Self::builder().build()`].
        #[inline]
        pub fn create() -> #builder_type {
          Self::builder().build()
        }
      }
    });
  }
}

// =============================================================================
// Extension Trait
// =============================================================================

struct ExtensionList<'a>(&'a [Command]);

impl ToTokens for ExtensionList<'_> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let functions = self
      .0
      .iter()
      .filter(|command| command.is_command())
      .map(Command::extension);

    tokens.extend(quote! {
      type Send<T> = <T as crate::traits::Message>::Send;
      type Recv<T> = <T as crate::traits::Message>::Recv;

      /// Extension trait for simplified command execution.
      pub trait CommandExt {
        /// The type of error returned by the implemention.
        type Error;

        /// Called after a successful authentication request.
        fn init(&mut self, session_id: String);

        /// Send a gRPC request to the PTSL server.
        async fn send<T>(&mut self, request: Send<T>) -> Result<Recv<T>, Self::Error>
        where
          T: crate::traits::Message + ?Sized;

        #(#functions)*
      }
    });
  }
}

// =============================================================================
// Command Extension
// =============================================================================

struct Extension<'a>(&'a Command);

impl Extension<'_> {
  fn new_sender(&self) -> Expr {
    if self.0.mode.send() {
      let ident: &Ident = &self.0.name;
      let names: Vec<&Ident> = self.0.param_names();
      let feats: Vec<Option<Attribute>> = self.0.param_features();

      parse_quote!(#ident::request(#(#feats #names),*))
    } else {
      parse_quote!(())
    }
  }

  fn initialize(&self) -> Option<TokenStream> {
    if self.is_authentication() {
      Some(quote!(self.init(recv.session_id.clone());))
    } else {
      None
    }
  }

  fn parameters(&self) -> Option<TokenStream> {
    if self.0.mode.send() {
      let param_names: Vec<&Ident> = self.0.param_names();
      let param_kinds: Vec<&Type> = self.0.param_kinds();
      let param_feats: Vec<Option<Attribute>> = self.0.param_features();

      Some(quote!(#(#param_feats #param_names: #param_kinds),*))
    } else {
      None
    }
  }

  fn is_authentication(&self) -> bool {
    self.0.name == "RegisterConnection" || self.0.name == "AuthorizeConnection"
  }
}

impl ToTokens for Extension<'_> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let camel_name: &Ident = &self.0.name;
    let snake_name: Ident = ident(camel_name.to_string().to_snek_case());

    let new_sender: Expr = self.new_sender();
    let initialize: Option<TokenStream> = self.initialize();
    let parameters: Option<TokenStream> = self.parameters();
    let doc_string: Documentation<'_> = Documentation::Extension(camel_name);

    let feature: Option<Attribute> = self.0.attr.feature();

    tokens.extend(quote! {
      #feature
      #[doc = #doc_string]
      async fn #snake_name(&mut self, #parameters) -> Result<Recv<#camel_name>, Self::Error> {
        let send: Send<#camel_name> = #new_sender;
        let recv: Recv<#camel_name> = self.send::<#camel_name>(send).await?;

        #initialize

        Ok(recv)
      }
    });
  }
}

// =============================================================================
// Misc. Documentation
// =============================================================================

enum Documentation<'a> {
  Message(&'a Ident),
  Request(&'a Ident, Option<Ident>),
  Builder(&'a Ident),
  Extension(&'a Ident),
}

impl Documentation<'_> {
  fn string(&self) -> String {
    match self {
      Self::Message(name) => {
        format!(
          "Marker type for common behaviour of the [`{name}`][crate::types::CommandId::{name}] command.",
          name = name.to_string(),
        )
      }
      Self::Request(name, None) => {
        format!("Create a new [`{name} request`][()].", name = Format(name))
      }
      Self::Request(name, Some(send)) => {
        format!(
          "Create a new [`{name} request`][crate::types::{send}].",
          name = Format(name),
        )
      }
      Self::Builder(name) => {
        format!("Set the value of the `{name}` field.")
      }
      Self::Extension(name) => {
        format!(
          "Construct a new `{name} request` and send to the PTSL server.",
          name = Format(name),
        )
      }
    }
  }
}

impl ToTokens for Documentation<'_> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let output: String = self.string();
    tokens.extend(quote!(#output))
  }
}

struct Format<'a>(&'a Ident);

impl Display for Format<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.0.to_string().to_title_case().to_lowercase())
  }
}
