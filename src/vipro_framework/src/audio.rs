use std::thread;
use raylib::prelude::{RaylibAudio,Sound};
pub struct AudioChannel {
    sender:flume::Sender<&'static str>,
    receiver: flume::Receiver<&'static str>,
    audio_hashmap:hashbrown::HashMap<&'static str,&'static str>
}
impl AudioChannel {
    pub fn new() -> Result<Self,Box<dyn std::error::Error>> {
        let (sender,receiver) =  flume::unbounded();
        Ok(
            Self {
                sender,receiver,
                audio_hashmap:hashbrown::HashMap::new()
            }
        )
    }
    /// Will likely be deleted soon
    pub fn get_sender(&self) -> flume::Sender<&'static str> {
        self.sender.clone()
    }
    /// Audio will need to be added before to the Audio Channel before it can be played.
    pub fn add_audio(&mut self,name:&'static str,path:&'static str) {
        self.audio_hashmap.insert(name,path);
    }
    pub fn open_audios(&self) -> Result<hashbrown::HashMap<&'static str,Sound>,()> {
        let mut audio_hashmap = hashbrown::HashMap::new();
        for entry in self.audio_hashmap.iter() {
            let file = match Sound::load_sound(entry.1) {
                Ok(sound) => sound,
                Err(_) => return Err(())
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

    }
}
