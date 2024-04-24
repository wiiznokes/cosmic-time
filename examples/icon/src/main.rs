use cosmic::{
    app::{Core, Settings}, executor, iced::{Alignment, Event, Length, Subscription}, widget::{button, column, text}, Command, Element
};
use cosmic_time::{self, anim, chain, id, once_cell::sync::Lazy, reexports::iced, Duration, Instant, Timeline, container};

static CONTAINER: Lazy<id::Container> = Lazy::new(id::Container::unique);

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(cosmic::app::run::<Counter>(Settings::default(), ())?)
}

struct Counter {
    core: Core,
    value: i32,
    timeline: Timeline,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
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
        flags: Self::Flags,
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

        timeline.set_chain(animation).start();

        let state = Self {
            core,
            value: 0,
            timeline,
        };

        (state, Command::none())
    }

    fn subscription(&self) -> Subscription<Message> {
        // This is the magic that lets us animaate. Cosmic-time looks
        // at what timeline you have built and decides for you how often your
        // application should redraw for you! When the animation is done idle
        // or finished, cosmic-time will keep your applicaiton idle!
        self.timeline.as_subscription().map(|(_, instant)| Message::Tick(instant))
    }

    fn update(&mut self, message: Self::Message) ->  Command<cosmic::app::Message<Message>> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::Tick(now) => self.timeline.now(now),
        }
        Command::none()
    }
    

    fn view(&self) -> Element<Message> {
        let column = column()
            .push(
                button("Increment")
                    .on_press(Message::IncrementPressed)
                    .width(Length::Fill),
            )
            .push(text(self.value.to_string()).size(50).height(Length::Fill))
            .push(
                button("Decrement")
                    .on_press(Message::DecrementPressed)
                    .width(Length::Fill),
            )
            .padding(20)
            .align_items(Alignment::Center);

        anim!(CONTAINER, &self.timeline, column).into()
    }
}
