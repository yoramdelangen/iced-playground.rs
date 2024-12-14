use iced::{
    widget::{button, column, container, text},
    Element, Length,
};

mod screen {
    use iced::{
        alignment::{Horizontal, Vertical},
        widget::{button, column, row, text},
        Element,
    };

    #[derive(Default, Debug, Clone, Copy)]
    pub struct Counter {
        value: u32,
    }

    impl Counter {
        pub fn new() -> Self {
            println!("New instance");
            Self { value: 10 }
        }

        pub fn view(&self) -> Element<CounterMessage> {
            println!("Render counter: {:?}", self);
            let buttons = row![
                button("+").on_press(CounterMessage::Increment),
                text(self.value).size(50),
                button("-").on_press(CounterMessage::Decrement),
            ]
            .align_y(Vertical::Center)
            .spacing(5);

            let back = button("Back").on_press(CounterMessage::Navigate(crate::Route::Home));

            column![buttons, back]
                .align_x(Horizontal::Center)
                .padding(10)
                .into()
        }

        pub fn update(&mut self, msg: CounterMessage) -> Command<CounterMessage> {
            match msg {
                CounterMessage::Increment => self.value += 1,
                CounterMessage::Decrement => {
                    if self.value > 0 {
                        self.value -= 1
                    }
                }
                CounterMessage::Navigate(_route) => {}
            };

            msg
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum CounterMessage {
        Increment,
        Decrement,
        Navigate(crate::Route),
    }
}

#[derive(Debug, Clone, Copy)]
enum Route {
    Home,
    Counter(screen::Counter),
}

impl Default for Route {
    fn default() -> Self {
        Self::Home
    }
}

#[derive(Debug, Clone, Copy)]
enum RouteMessage {
    Home(),
    Counter(screen::CounterMessage),
}
//
// impl Default for RouteState {
//     fn default() -> Self {
//         Self::Counter(screen::Counter::new())
//     }
// }

#[derive(Debug, Clone)]
enum Message {
    Navigate(Route),

    // handle sub-routes messages
    RouteMessage(RouteMessage),
}

#[derive(Default)]
struct App {
    router: Route,
}

impl App {
    fn view(&self) -> Element<Message> {
        let navbar = container(text("Navbar"))
            .padding([5, 0])
            .center_x(Length::Fill);

        let content: Element<Message> = match &self.router {
            Route::Counter(counter) => counter
                .view()
                .map(|msg| Message::RouteMessage(RouteMessage::Counter(msg))),
            Route::Home => button("GO to router")
                .on_press(Message::Navigate(Route::Counter(screen::Counter::new())))
                .into(),
        };

        column![navbar, container(content).center_x(Length::Fill),].into()
    }

    fn update(&mut self, msg: Message) {
        // something...
        match msg {
            Message::Navigate(route) => match route {
                Route::Home => self.router = Route::Home,
                Route::Counter(counter) => {
                    println!("Navigate to counter");
                    self.router = Route::Counter(counter)
                }
            },

            Message::RouteMessage(message) => {
                match message {
                    RouteMessage::Home() => {}
                    RouteMessage::Counter(counter_message) => {
                        // get counter instance and send update
                        if let Route::Counter(mut counter) = &mut self.router {
                            println!("Counter {:?}", counter_message);
                            counter.update(counter_message);
                            println!("My message for couter {:?}", counter);
                        };
                    }
                }

            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("Counter :)", App::update, App::view)
}
