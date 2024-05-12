use cosmic::{
    app::{Core, Settings}, desktop::IconSource, executor, iced::{Alignment, Event, Length, Subscription}, widget::{button, column, icon::Handle, text, Column, Container, Space}, Command, Element
};
use cosmic_time::{
    self, anim, chain, container, id, once_cell::sync::Lazy, Duration, Instant, Timeline,
};

static CONTAINER: Lazy<id::Container> = Lazy::new(id::Container::unique);

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(cosmic::app::run::<Counter>(Settings::default(), ())?)
}


const TAILLE_ICON: u16 = 30;

const TAILLE_CONTAINER: u16 = TAILLE_ICON + 10;

const DELTA: u16 = 5;

struct Counter {
    core: Core,
    timeline: Timeline,
    icon: IconSource,
}

#[derive(Debug, Clone)]
enum Message {
    Pressed,
    Released,
    DoNothing,
    Tick(Instant),
}

impl cosmic::Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    const APP_ID: &'static str = "hello";

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::app::Core,
        _flags: Self::Flags,
    ) -> (
        Self,
        cosmic::iced::Command<cosmic::app::Message<Self::Message>>,
    ) {

        let state = Self {
            core,
            timeline: Timeline::new(),
            icon: IconSource::Name("firefox".to_string()),
        };

        (state, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<cosmic::app::Message<Message>> {


        match message {
            Message::Tick(now) => self.timeline.now(now),
            Message::Pressed => {
                println!("Pressed");

                let final_size = TAILLE_CONTAINER - DELTA;
                
                let animation = chain![
                    CONTAINER,
                    container(Duration::ZERO).width(TAILLE_CONTAINER).height(TAILLE_CONTAINER),
                    container(Duration::from_millis(100)).width(final_size).height(final_size),
                ];

                self.timeline.set_chain(animation).start();
            }
            Message::Released => {
                println!("Released");

                
                let animation = chain![
                    CONTAINER,
                    container(Duration::from_millis(100)).width(TAILLE_CONTAINER).height(TAILLE_CONTAINER),
                ];

                self.timeline.set_chain(animation).start();
            }
            Message::DoNothing => {}
            /*
            Message::ToggleBluetooth(chain, enabled) => {
                if self.enable == enabled {
                    return Command::none();
                }
                self.timeline.set_chain(chain).start();
                self.enable = enabled;
            }
             */
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let icon = self.icon.as_cosmic_icon().size(TAILLE_ICON);

        let button = cosmic::widget::button::button(icon);//.on_press(Message::DoNothing);

        let button = cosmic::widget::mouse_area(button)
            .on_press(Message::Pressed)
            .on_release(Message::Released);

        let e = anim!(CONTAINER, &self.timeline, button);
        
        let content = Column::new()
            .push(e)
            .push(Space::with_height(100))
            .push(self.icon.as_cosmic_icon().size(TAILLE_ICON));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        // This is the magic that lets us animaate. Cosmic-time looks
        // at what timeline you have built and decides for you how often your
        // application should redraw for you! When the animation is done idle
        // or finished, cosmic-time will keep your applicaiton idle!
        self.timeline
            .as_subscription()
            .map(|(_, instant)| Message::Tick(instant))
    }
}