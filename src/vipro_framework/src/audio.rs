use std::thread;
use anyhow::Result;
use raylib::prelude::{RaylibAudio,Sound};
#[derive(Debug)]
pub struct AudioChannel {
    sender:flume::Sender<&'static str>,
    receiver: flume::Receiver<&'static str>,
    audio_hashmap:hashbrown::HashMap<&'static str,&'static str>
}
#[derive(Debug,Clone)]
pub struct AudioSender {
    sender:flume::Sender<&'static str>
}
impl AudioSender {
    /// Ends the Audio Channel thread
    pub fn stop(self) -> Result<()>  {
        self.sender.send("drop")?;
        Ok(())
    }
    /// Ends the Audio Channel thread asynchronously
    pub async fn stop_async(self) -> Result<()>  {
        self.sender.send_async("drop").await?;
        Ok(())
    }
    pub fn try_stop(self) -> Result<()> {
        self.sender.try_send("drop")?;
        Ok(())
    }
    /// Sends a name of a sound to be played
    pub fn play_sound(&self,name:impl Into<&'static str>) -> Result<()>
    {
        self.sender.send(name.into())?;
        Ok(())
    }
    /// Sends a name of a sound to be played asynchronously
    pub async fn play_sound_async(&self,name:impl Into<&'static str>) -> Result<()> {
        self.sender.send_async(name.into()).await?;
        Ok(())
    }
    /// Attempts to send a name of a sound to be played
    pub fn try_play_sound(&self,name:impl Into<&'static str>) -> Result<()>
    {
        self.sender.try_send(name.into())?;
        Ok(())
    }
}
impl AudioChannel {
    pub fn new() -> Result<Self> {
        let (sender,receiver) =  flume::unbounded();
        Ok(
            Self {
                sender,receiver,
                audio_hashmap:hashbrown::HashMap::new()
            }
        )
    }
    /// Will likely be deleted soon
    pub fn get_sender(&self) ->  AudioSender {
        AudioSender { sender: self.sender.clone() }
    }
    /// Audio will need to be added before to the Audio Channel before it can be played.
    pub fn add_audio(&mut self,name:&'static str,path:&'static str) {
        if name == "drop" { panic!("Use of reserved name 'drop'") }
        self.audio_hashmap.insert(name,path);
    }
    pub fn open_audios(&self) -> Result<hashbrown::HashMap<&'static str,Sound>,String> {
        let mut audio_hashmap = hashbrown::HashMap::new();
        for entry in self.audio_hashmap.iter() {
            let file = match Sound::load_sound(entry.1) {
                Ok(sound) => sound,
                Err(_) => {
                    return Err(format!("Could not open file {} this could be because it is invalid, doesn't exist, or you do not have the right permissions.",entry.1));
                }
            };
            audio_hashmap.insert(*entry.0, file);
        }
        Ok(audio_hashmap)
    }
    //// Listens on a new thread for a &str and will play it if the name is in the audio hashmap
    /// If an invalide name is given (a name to an audio file that doesn't exist) it will be ignored.
    /// Use audio_listener_abort() to abort on invalid name.
    pub fn audio_listener(self) {
        thread::spawn(move || {
            let mut stream = RaylibAudio::init_audio_device();
            let audio_hash = self.open_audios().expect("could not one or more of the sound files");
            while let Ok(msg) = self.receiver.recv() {
                // the drop message will be sent once the window and everything else closes
                if msg == "drop" { break; }
                if let Some(audio) = audio_hash.get(msg) {
                    stream.play_sound(audio);
                }
            }
        });
    }
    /// Same thing as audio_listener() however this panics if
    /// an unknown audio name is given
    pub fn audio_listener_abort(self) {
        thread::spawn(move || {
            let mut stream = RaylibAudio::init_audio_device();
            let audio_hash = self.open_audios().expect("could not one or more of the sound files");
            while let Ok(msg) = self.receiver.recv() {
                // the drop message will be sent once the window and everything else closes
                if msg == "drop" { break; }
                if let Some(audio) = audio_hash.get(msg) {
                    stream.play_sound(audio);
                } else {
                    panic!("Encounted a sound that is not it the hashmap name: {msg} hashmap:{audio_hash:#?}");
                }
            }
        });

    }
}
