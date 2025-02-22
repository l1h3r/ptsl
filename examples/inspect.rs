use ptsl_client::client::Client;
use ptsl_client::error::Result;
use ptsl_protos::bridge::CommandExt;

const APPNAME: &str = "PTSL_Example_Inspect";
const COMPANY: &str = "PTSL";

#[rustfmt::skip]
#[tokio::main]
async fn main() -> Result<()> {
  let mut client: Client = Client::new().await?;

  client.register_connection(APPNAME.into(), COMPANY.into()).await?;

  println!("[PTSL Version]:              {}", client.get_ptsl_version().await?.version);
  println!("[Playback Mode]:             {:?}", client.get_playback_mode().await?.current_settings().collect::<Vec<_>>());
  println!("[Record Mode]:               {}", client.get_record_mode().await?.current_setting());
  println!("[Session Audio Format]:      {}", client.get_session_audio_format().await?.current_setting());
  println!("[Session Audio Rate Pull]:   {}", client.get_session_audio_rate_pull_settings().await?.current_setting());
  println!("[Session Bit Depth]:         {}", client.get_session_bit_depth().await?.current_setting());
  println!("[Session Feet Frames Rate]:  {}", client.get_session_feet_frames_rate().await?.current_setting());
  println!("[Session Interleaved State]: {}", client.get_session_interleaved_state().await?.current_setting);
  println!("[Session Length]:            {}", client.get_session_length().await?.session_length);
  println!("[Session Name]:              {}", client.get_session_name().await?.session_name);

  if let Some(session_path) = client.get_session_path().await?.session_path {
    println!("[Session Path]:              {}", session_path.path);
    println!("[Session Path (online)]:     {}", session_path.info.map_or(false, |info| info.is_online));
  }

  println!("[Session Sample Rate]:       {}", client.get_session_sample_rate().await?.sample_rate());
  println!("[Session Start Time]:        {}", client.get_session_start_time().await?.session_start_time);
  println!("[Session Time Code Rate]:    {}", client.get_session_time_code_rate().await?.current_setting());
  println!("[Session Video Rate Pull]:   {}", client.get_session_video_rate_pull_settings().await?.current_setting());
  println!("[Transport Armed]:           {}", client.get_transport_armed().await?.is_transport_armed);
  println!("[Transport State]:           {}", client.get_transport_state().await?.current_setting());

  println!("[Session Edit Mode]:         {}", client.get_edit_mode().await?.current_setting());
  println!("[Session Edit Tool]:         {}", client.get_edit_tool().await?.current_setting());
  println!("[Edit Mode Options]:         {:#?}", client.get_edit_mode_options().await?.edit_mode_options);

  Ok(())
}
