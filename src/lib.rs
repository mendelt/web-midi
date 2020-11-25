//! Wrap web-sys webmidi calls in a more rusty API

pub mod error;

use js_sys::Array;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{MidiOptions, MidiPortConnectionState, MidiPortDeviceState};

/// This is the main entry point to the web midi library and gives access to midi inputs and outputs
pub struct MidiAccess {
    access: web_sys::MidiAccess,
}

impl MidiAccess {
    /// Open a midi MidiAccess instance from a Navigator
    pub async fn get_access(navigator: web_sys::Navigator) -> Self {
        // let window = web_sys::window().expect("no global `window` exists");

        let access: web_sys::MidiAccess = JsFuture::from(
            navigator
                .request_midi_access_with_options(MidiOptions::new().sysex(true).software(true))
                .unwrap(),
        )
        .await
        .unwrap()
        .into();

        Self { access }
    }

    /// Get all available Midi inputs
    pub fn inputs(&self) -> Vec<MidiInput> {
        js_sys::try_iter(&self.access.inputs())
            .unwrap()
            .unwrap()
            .map(|entry| {
                let array: Array = entry.unwrap().into();
                MidiInput {
                    input: array.get(1).into(),
                }
            })
            .collect()
    }

    /// Get all available Midi outputs
    pub fn outputs(&self) -> Vec<MidiOutput> {
        js_sys::try_iter(&self.access.outputs())
            .unwrap()
            .unwrap()
            .map(|entry| {
                let array: Array = entry.unwrap().into();
                MidiOutput {
                    output: array.get(1).into(),
                }
            })
            .collect()
    }

    /// Return true if sysex is enabled for this MidiAccess
    pub fn sysex_enabled(&self) -> bool {
        self.access.sysex_enabled()
    }

    // pub fn onstatechange(&self) -> ... {
    //     todo!()
    // }

    // pub fn set_onstatechange(&self) -> ... {
    //     todo!()
    // }
}

pub struct MidiInput {
    input: web_sys::MidiInput,
}

impl MidiInput {
    // pub fn onmidimessage(&self) -> ... {
    //     todo!()
    // }

    // pub fn set_onmidimessage(&self) -> ... {
    //     todo!()
    // }

    pub fn id(&self) -> String {
        self.input.id()
    }

    pub fn manufacturer(&self) -> Option<String> {
        self.input.manufacturer()
    }

    pub fn name(&self) -> Option<String> {
        self.input.name()
    }

    pub fn version(&self) -> Option<String> {
        self.input.version()
    }

    pub fn state(&self) -> MidiPortDeviceState {
        self.input.state()
    }

    pub fn connection(&self) -> MidiPortConnectionState {
        self.input.connection()
    }

    // pub fn onstatechange(&self) -> ... {
    //     todo!()
    // }

    // pub fn set_onstatechange(&self) -> ... {
    //     todo!()
    // }

    pub async fn open(&self) -> &Self {
        JsFuture::from(self.input.open()).await.unwrap();
        self
    }

    pub async fn close(&self) -> &Self {
        JsFuture::from(self.input.close()).await.unwrap();
        self
    }
}

pub struct MidiOutput {
    output: web_sys::MidiOutput,
}

impl MidiOutput {
    pub fn clear(&self) {
        self.output.clear()
    }

    // TODO: fix input and output types to be something sensible
    pub fn send(&self, data: &JsValue) -> Result<(), JsValue> {
        self.output.send(data)
    }

    // TODO: fix input and output types to be something sensible
    pub fn send_with_timestamp(&self, data: &JsValue, timestamp: f64) -> Result<(), JsValue> {
        self.output.send_with_timestamp(data, timestamp)
    }

    pub fn id(&self) -> String {
        self.output.id()
    }

    pub fn manufacturer(&self) -> Option<String> {
        self.output.manufacturer()
    }

    pub fn name(&self) -> Option<String> {
        self.output.name()
    }

    pub fn version(&self) -> Option<String> {
        self.output.version()
    }

    pub fn state(&self) -> MidiPortDeviceState {
        self.output.state()
    }

    pub fn connection(&self) -> MidiPortConnectionState {
        self.output.connection()
    }

    // pub fn onstatechange(&self) -> ... {
    //     todo!()
    // }

    // pub fn set_onstatechange(&self) -> ... {
    //     todo!()
    // }

    pub async fn open(&self) -> &Self {
        JsFuture::from(self.output.open()).await.unwrap();
        self
    }

    pub async fn close(&self) -> &Self {
        JsFuture::from(self.output.close()).await.unwrap();
        self
    }
}
