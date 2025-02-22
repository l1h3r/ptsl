use ptsl_protos::bridge::CommandExt;
use ptsl_protos::result::CommandResult;
use ptsl_protos::traits::Decode;
use ptsl_protos::traits::Encode;
use ptsl_protos::traits::Message;
use ptsl_protos::types::CommandId;
use ptsl_protos::types::Request;
use ptsl_protos::types::RequestHeader;
use ptsl_protos::types::Response;
use std::cell::Cell;
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::client::Config;
use crate::client::Rpc;
use crate::client::Stream;
use crate::consts::CLIENT_VERSION_LIST;
use crate::consts::METHOD_VERSION_LIST;
use crate::consts::PTSL_VERSION;
use crate::error::Error;
use crate::error::Result;
use crate::types::VersionData;
use crate::types::VersionType;

// =============================================================================
// Client Status
// =============================================================================

/// Status of the gRPC client.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
  /// Client is connected to gRPC endpoint.
  Connected,
  /// Client is connected and server is known to be active.
  Activated,
}

// =============================================================================
// Client Core
// =============================================================================

#[derive(Debug)]
struct ClientCore {
  config: Config,
  status: Cell<Status>,
  session: OnceCell<String>,
}

impl ClientCore {
  #[inline]
  fn new(config: Config) -> Self {
    Self {
      config,
      status: Cell::new(Status::Connected),
      session: OnceCell::new(),
    }
  }
}

// =============================================================================
// Client
// =============================================================================

/// PTSL client interface.
#[derive(Clone, Debug)]
pub struct Client {
  grpc: Rpc,
  core: Rc<ClientCore>,
}

impl Client {
  /// Create a new PTSL `Client`.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if gRPC initialization fails.
  pub async fn new() -> Result<Self> {
    Self::from_config(Config::new()).await
  }

  /// Create a new PTSL `Client` from the given `config`.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if gRPC initialization fails.
  pub async fn from_config(config: Config) -> Result<Self> {
    if config.launch {
      super::launch()?;
    }

    if config.locate {
      todo!("[client]: locate server")
    }

    let mut this: Self = Self {
      grpc: Rpc::connect(&config).await?,
      core: Rc::new(ClientCore::new(config)),
    };

    activate(&mut this).await?;

    Ok(this)
  }

  /// Returns the status of the gRPC client.
  #[inline]
  pub fn status(&self) -> Status {
    self.core.status.get()
  }

  /// Returns the configuration used to initialize the client.
  #[inline]
  pub fn config(&self) -> &Config {
    &self.core.config
  }

  /// Returns the PTSL session id.
  #[inline]
  pub fn session(&self) -> Option<&str> {
    self.core.session.get().map(Deref::deref)
  }

  #[inline]
  pub(crate) fn set_status(&self, value: Status) {
    self.core.status.set(value);
  }

  #[inline]
  pub(crate) fn set_session(&mut self, value: String) {
    let _set: bool = self.core.session.set(value).is_ok();
  }

  // ===========================================================================
  // Misc. API
  // ===========================================================================

  /// Query the PTSL version.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if the gRPC request fails.
  pub async fn version(&mut self, version: VersionType) -> Result<VersionData> {
    let data: VersionData = match version {
      VersionType::Server => VersionData::Server(self.get_ptsl_version().await?.version),
      VersionType::Client => VersionData::Client(CLIENT_VERSION_LIST),
      VersionType::ClientAPI => VersionData::Client(METHOD_VERSION_LIST),
    };

    Ok(data)
  }

  // ===========================================================================
  // gRPC Utilities
  // ===========================================================================

  async fn dispatch<T>(&mut self, request: T::Send) -> Result<T::Recv>
  where
    T: Message + ?Sized,
  {
    let request: Request = RequestBuilder::new(T::TYPE)
      .request(request.encode()?)
      .session(self.session().map(ToOwned::to_owned))
      .build();

    let result: CommandResult<T::Recv> = if T::VIA_STREAM {
      self
        .send_streaming_command(T::TYPE, request, T::SEND_PINGS)
        .await?
    } else {
      self.send_command(T::TYPE, request).await?
    };

    result.into_result().map_err(Into::into)
  }

  #[allow(clippy::future_not_send)]
  async fn send_command<T>(
    &mut self,
    command: CommandId,
    request: Request,
  ) -> Result<CommandResult<T>>
  where
    T: for<'de> Decode<'de>,
  {
    let output: Response = self.grpc.send_request(request).await?;
    let result: CommandResult<T> = CommandResult::try_new(command, output)?;

    Ok(result)
  }

  #[allow(clippy::future_not_send)]
  async fn send_streaming_command<T>(
    &mut self,
    command: CommandId,
    request: Request,
    _pings: bool,
  ) -> Result<CommandResult<T>>
  where
    T: for<'de> Decode<'de>,
  {
    let mut stream: Stream = self.grpc.send_streaming_request(request).await?;
    let mut latest: CommandResult<T> = CommandResult::empty(command);

    while let Some(output) = stream.message().await? {
      latest = CommandResult::try_new(command, output)?;
    }

    Ok(latest)
  }
}

impl CommandExt for Client {
  type Error = Error;

  #[inline]
  fn init(&mut self, session_id: String) {
    self.set_session(session_id);
  }

  #[inline]
  async fn send<T>(
    &mut self,
    request: <T as Message>::Send,
  ) -> Result<<T as Message>::Recv, Self::Error>
  where
    T: Message + ?Sized,
  {
    self.dispatch::<T>(request).await
  }
}

// =============================================================================
// Check Host Status
// =============================================================================

async fn activate(client: &mut Client) -> Result<()> {
  const COMMAND: CommandId = CommandId::HostReadyCheck;

  let request: Request = RequestBuilder::new(COMMAND).build();
  let response: CommandResult<()> = client.send_command(COMMAND, request).await?;

  if response.is_completed() {
    client.set_status(Status::Activated);
  }

  Ok(())
}

// =============================================================================
// Request Builder
// =============================================================================

struct RequestBuilder {
  command: CommandId,
  session: Option<String>,
  request: Option<String>,
}

impl RequestBuilder {
  const fn new(command: CommandId) -> Self {
    Self {
      command,
      session: None,
      request: None,
    }
  }

  fn session<T: Into<Option<String>>>(mut self, value: T) -> Self {
    self.session = value.into();
    self
  }

  fn request<T: Into<Option<String>>>(mut self, value: T) -> Self {
    self.request = value.into();
    self
  }

  fn build(self) -> Request {
    Request {
      header: Some(RequestHeader {
        task_id: String::new(),
        command: self.command as i32,
        version: PTSL_VERSION,
        session_id: self.session.unwrap_or_default(),
      }),
      request_body_json: Self::format_request(self.request),
    }
  }

  fn format_request(request: Option<String>) -> String {
    static NULL: &str = "null";

    let mut request: String = request.unwrap_or_default();

    if request == NULL {
      request.clear();
    }

    request
  }
}
