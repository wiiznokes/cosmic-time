use cosmic::{
    app::{Core, Settings},
    desktop::IconSource,
    executor,
    iced::{Alignment, Event, Length, Subscription},
    widget::{button, column, icon::Handle, text},
    Command, Element,
};
use cosmic_time::{
    self, anim, chain, container, id, once_cell::sync::Lazy, Duration, Instant, Timeline,
};

static CONTAINER: Lazy<id::Container> = Lazy::new(id::Container::unique);

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(cosmic::app::run::<Counter>(Settings::default(), ())?)
}

struct Counter {
    core: Core,
    value: i32,
    timeline: Timeline,
    enable: bool,
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
        let mut timeline = Timeline::new();
        let animation = chain![
            CONTAINER,
            container(Duration::ZERO).width(0.).height(100.),
            container(Duration::from_secs(2)).width(200.).height(100.),
            container(Duration::from_secs(2))
                .width(200.)
                .height(300.)
                .padding([0, 0, 0, 0]),
            container(Duration::from_secs(2))
                .width(700.)
                .height(300.)
                .padding([0, 0, 0, 500]),
            container(Duration::from_secs(2))
                .width(150.)
                .height(150.)
                .padding([0, 0, 0, 0]),
        ];

        //timeline.set_chain(animation).start();

        let state = Self {
            core,
            value: 0,
            timeline,
            enable: true,
            icon: IconSource::Name("firefox".to_string()),
        };

        (state, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<cosmic::app::Message<Message>> {
        match message {
            Message::Tick(now) => self.timeline.now(now),
            Message::Pressed => {}
            Message::Released => {}
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
        let icon = self.icon.as_cosmic_icon().size(30);

        let button = cosmic::widget::button::button(icon).on_press(Message::DoNothing);

        let button = cosmic::widget::mouse_area(button)
            .on_press(Message::Pressed)
            .on_release(Message::Released);

        anim!(CONTAINER, &self.timeline, button).into()
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

static BLUETOOTH_ENABLED: Lazy<id::Toggler> = Lazy::new(id::Toggler::unique);
