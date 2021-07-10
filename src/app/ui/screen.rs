use crate::{settings::AppState, AppData, AppSettings, Message};
use iced::Element;

pub mod slib;
pub mod sread;
pub mod sset;

pub use self::{slib::*, sread::*, sset::*};

#[derive(Debug)]
pub struct Screens {
    pub sset:  SSet,
    pub slib:  SLib,
    pub sread: SRead,
    pub state: AppState,
}
#[derive(Debug, Clone, Copy)]
pub enum ViewA {
    Switch(AppState),
    ASet(ASet),
    ARead(ARead),
    ALib(ALib),
}

impl Screens {
    pub fn new() -> Self {
        Self {
            sset:  SSet::new(),
            slib:  SLib::new(),
            sread: SRead::new(),
            state: AppState::Reader,
        }
    }

    pub fn view<'a>(
        &'a mut self, data: &'a mut AppData, settings: &AppSettings,
    ) -> Element<Message> {
        match self.state {
            AppState::Settings => self.sset.view(),
            AppState::Reader => self.sread.view(data, settings),
            AppState::Library => self.slib.view(),
        }
    }

    pub fn update(&mut self, message: ViewA) {
        match message {
            ViewA::Switch(s) => self.state = s,
            ViewA::ASet(a) => self.sset.update(a),
            ViewA::ARead(a) => self.sread.update(a),
            ViewA::ALib(a) => self.slib.update(a),
        }
    }
}
