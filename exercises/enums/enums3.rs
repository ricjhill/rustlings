// enums3.rs
// Address all the TODOs to make the tests pass!




enum Message {
    Quit,
    Move{x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32, i32, i32)
    // TODO: implement the message variant types based on their usage below
}

struct Point {
    x: i32,
    y: i32
}

struct State {
    color: (i32, i32, i32),
    position: Point,
    quit: bool
}

impl State {
    fn change_color(&mut self, color: (i32, i32, i32)) {
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
            Message:: Quit  => {println!("The Quit variant has no data to destructure." )},
            Message:: Move{ x,y} => self.move_position(Point{ x, y})  ,
            Message:: Echo(text) => self.echo(text),
            Message:: ChangeColor( r,g,b) => self.change_color( (r, g, b) )
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
            position: Point{ x: 0, y: 15 },
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
