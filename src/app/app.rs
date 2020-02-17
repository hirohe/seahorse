use crate::Command;

#[derive(Default)]
pub struct App {
    pub name: String,
    pub display_name: String,
    pub usage: String,
    pub version: String,
    pub commands: Vec<Command>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    pub fn display_name<T: Into<String>>(mut self, display_name: T) -> Self {
        self.display_name = display_name.into();
        self
    }

    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = usage.into();
        self
    }

    pub fn version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = version.into();
        self
    }

    pub fn commands(mut self, commands: Vec<Command>) -> Self {
        self.commands = commands;
        self
    }

    pub fn run(&self, args: Vec<String>) {
        match args.len() {
            1 => {
                self.help();
                return;
            }
            _ => (),
        }

        let (cmd_v, args_v) = args[1..].split_at(1);
        let cmd = match cmd_v.first() {
            Some(c) => c,
            None => {
                self.help();
                return;
            }
        };

        match (cmd.len(), args_v.len()) {
            (0, _) | (_, 0) => {
                self.help();
                return;
            }
            _ => (),
        }

        match self.select_command(&cmd) {
            Some(command) => {
                command.run(args);
            }
            None => self.help(),
        }
    }

    fn help(&self) {
        match self.display_name.len() {
            0 => println!("Name:\n\t{}\n", self.name),
            _ => println!("Name:\n\t{}\n", self.display_name),
        }

        println!("Usage:\n\t{}\n", self.usage);
        println!("Version:\n\t{}\n", self.version);

        println!("Commands:");
        for c in &self.commands {
            println!("\t{} : {}", c.name, c.usage);

            match &c.flags {
                Some(flags) => {
                    for flag in flags {
                        println!("\t\t{}", flag.usage)
                    }
                }
                _ => ()
            }
        }
    }

    fn select_command(&self, cmd: &String) -> Option<&Command> {
        (&self.commands)
            .into_iter()
            .find(|command| &command.name == cmd)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, App, Command, Context, Flag, FlagType};

    #[test]
    fn app_test() {
        let a: Action = |c: &Context| {
            assert_eq!(true, c.bool_flag("bool"));
            match c.string_flag("string") {
                Some(flag) => assert_eq!("string".to_string(), flag),
                None => assert!(false, "string test false...")
            }
            match c.int_flag("int") {
                Some(flag) => assert_eq!(100, flag),
                None => assert!(false, "int test false...")
            }
            match c.float_flag("float") {
                Some(flag) => assert_eq!(1.23, flag),
                None => assert!(false, "float test false...")
            }
        };
        let c = Command::new()
            .name("hello")
            .usage("test hello args")
            .action(a)
            .flags(vec![
                Flag::new("bool", "test hello [args] --bool", FlagType::Bool),
                Flag::new("string", "test hello [args] --int [int value]", FlagType::String),
                Flag::new("int", "test hello [args] --int [int value]", FlagType::Int),
                Flag::new("float", "test hello [args] --int [int value]", FlagType::Float),
            ]);
        let app = App::new()
            .name("test")
            .usage("test [command] [arg]")
            .version("0.0.1")
            .commands(vec![c]);

        app.run(vec![
            "test".to_string(),
            "hello".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
            "string".to_string(),
            "--int".to_string(),
            "100".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, "test [command] [arg]".to_string());
        assert_eq!(app.version, "0.0.1".to_string());
    }
}