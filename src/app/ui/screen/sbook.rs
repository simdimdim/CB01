use crate::{
    AppData,
    Id,
    Label,
    Message,
    ViewA,
    BLACK,
    GREEN,
    RED,
    WHITE,
    YELLOW,
};
use iced::{
    pick_list,
    text_input,
    Align,
    Column,
    Command,
    Container,
    Element,
    HorizontalAlignment,
    Length,
    PickList,
    Row,
    Space,
    Text,
    VerticalAlignment,
};

#[derive(Debug, Clone)]
pub struct SBook {
    pub titlein: text_input::State,
    pub pick:    pick_list::State<String>,
    pub err:     EBook,
    pub show:    Option<Id>,
    pub title:   String,
}
#[derive(Debug, Clone)]
pub enum ABook {
    Prev,
    Next,
    View(String),
    UpdateTitle(String),
    Rename(String),
}
#[derive(Debug, Clone)]
pub enum EBook {
    No,
}
impl SBook {
    pub fn new() -> Self {
        Self {
            titlein: text_input::State::default(),
            pick:    pick_list::State::default(),
            err:     EBook::No,
            show:    None,
            title:   "".to_owned(),
        }
    }

    pub fn view(
        &mut self, data: &mut AppData, darkmode: bool,
    ) -> Element<'_, Message> {
        let mut list = data.library.titles.find_all(self.title.as_str());

        if let Some(pos) = list.iter().position(|x| x == &self.title) {
            list.remove(pos);
        };
        let title =
            iced::TextInput::new(&mut self.titlein, "", &self.title, |s| {
                ABook::UpdateTitle(s).into()
            })
            .on_submit(ABook::Rename(self.title.clone()).into())
            .size(24);
        let mut titlerow = Row::new()
            .align_items(Align::Start)
            .width(Length::Fill)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(title);
        {
            let indicatorcnt;
            let color;
            match (self.show.is_some(), !list.is_empty()) {
                (true, true) => {
                    indicatorcnt = "✔️";
                    color = GREEN;
                }
                (true, false) => {
                    indicatorcnt = "-";
                    color = YELLOW;
                }
                (false, true) => {
                    indicatorcnt = "X";
                    color = RED;
                }
                (false, false) => {
                    indicatorcnt = "";
                    color = if darkmode { WHITE } else { BLACK };
                }
            };
            let indicator = Text::new(indicatorcnt).color(color);
            titlerow = titlerow.push(
                indicator
                    .width(Length::Shrink)
                    .vertical_alignment(VerticalAlignment::Center)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(28),
            )
        }
        titlerow = titlerow.push(Space::new(Length::Fill, Length::Shrink));
        let mut main = Column::new().align_items(Align::Center).push(titlerow);
        if !list.is_empty() {
            let select = PickList::new(&mut self.pick, list, None, |s| {
                ABook::View(s).into()
            });
            main = main.push(select);
        }
        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::Center)
            .align_y(Align::Center)
            .into()
    }

    pub fn update(
        &mut self, data: &mut AppData, message: ABook,
    ) -> Command<Message> {
        match message {
            ABook::Prev => {}
            ABook::Next => {}
            ABook::View(title) => {
                if let Some(id) = data.library.titles.id(&title.into()).copied() {
                    self.show = Some(id);
                    self.title =
                        data.library.titles.title(id).unwrap_or_default().0;
                }
            }
            ABook::UpdateTitle(t) => {
                self.title = t.clone();
                if let Some(&id) = data.library.titles.id(&Label(t)) {
                    self.show = Some(id);
                } else {
                    self.show = None
                }
            }
            ABook::Rename(_s) => {}
        };
        Command::none()
    }
}
impl From<ABook> for Message {
    fn from(a: ABook) -> Self { Message::Update(ViewA::ABook(a)) }
}
