// enums3.rs
// Address all the TODOs to make the tests pass!

// I AM NOT DONE

enum Message {
    Quit,Move{x: i32, y: i32 },Echo(String),ChangeColor(i32, i32, i32)
    // TODO: implement the message variant types based on their usage below
}

struct Point {
    x: u8,
    y: u8
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message{
            Message:: Quit  => {println!("The Quit variant has no data to destructure.")},
            Message:: Move{ x,y} => { println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );} ,
            Message:: Echo(text) =>println!("Text message: {}", text),
            Message:: ChangeColor( r,g,b) => println!( "Change the color to red {}, green {}, and blue {}",r, g, b)
        }
        // TODO: create a match expression to process the different message variants
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State{
            quit: true,
            position: Point{ x: 10, y: 15 },
            color: (255, 0, 255)
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move{ x: 10, y: 15 });
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }

}
