use std::io::fs::File;
use std::io::BufferedReader;

pub enum CommandType {
    ACommand,
    CCommand,
    LCommand
}

struct Command {
    c_type: CommandType,
    symbol: Option<~str>,
    dest:   Option<~str>,
    comp:   Option<~str>,
    jump:   Option<~str>
}

pub struct Parser {
    asm_reader: BufferedReader<File>,
    cur_command: Option<Command>
}

impl Parser {
    pub fn new(path: &str) -> Parser {
        let file = File::open(&Path::new(path));
        let mut reader = match file {
            Some(f) => BufferedReader::new(f),
            None    => fail!("File at " + path + " couldn't be found.")
        };


        let first_command = Command::from_str(reader.read_line().expect("ASM file is empty"));
        Parser{asm_reader:reader, cur_command: first_command}
    }

    pub fn advance(&mut self) -> Option<bool> {
        let next_line = match self.asm_reader.read_line() {
            Some(l) => l,
            None    => return None 
        };

        self.cur_command = Command::from_str(next_line);
        Some(true)
    }

    pub fn command_type(&self) -> Option<CommandType> {
        match self.cur_command {
            Some(ref c) => Some(c.c_type),
            None        => None
        }
    }

    pub fn symbol(&self) -> Option<~str> {
        match self.cur_command {
            Some(ref c) => c.symbol.clone(),
            None        => None
        }
    }

    pub fn dest(&self) -> Option<~str> {
        match self.cur_command {
            Some(ref c) => c.dest.clone(),
            None        => None
        }
    }

    pub fn comp(&self) -> Option<~str> {
        match self.cur_command {
            Some(ref c) => c.comp.clone(),
            None        => None
        }
    }

    pub fn jump(&self) -> Option<~str> {
        match self.cur_command {
            Some(ref c) => c.jump.clone(),
            None        => None
        }
    }

    pub fn rewind(&mut self) {
        let path = self.asm_reader.get_ref().path().clone();
        self.asm_reader = BufferedReader::new(File::open(&path).unwrap());
        self.cur_command = Command::from_str(self.asm_reader.read_line().unwrap());
    }
}

impl Command {
    fn from_str(mut cmd: &str) -> Option<Command> {
        // First we look for comments, and we remove them.
        match cmd.find_str("//") {
            Some(ind) => cmd = cmd.slice_to(ind),
            None      => {}
        }

        // We remove extra chars
        cmd = cmd.trim_chars(& &[' ', '\t', '\n', '\r']);

        // Then if the cmd is not empty, we find out its type. If it is, we return None.
        let cmd_type = match cmd.len() {
            0 => return None,
            _ => match cmd.char_at(0) {
                '('       => LCommand,
                '@'       => ACommand,
                _         => CCommand
            }
        };

        // Then we do the reading
        let (symbol, dest, comp, jump) = match cmd_type {
            LCommand => (Some(cmd.trim_chars(& &['(', ')']).to_owned()), None, None, None),
            ACommand => (Some(cmd.trim_chars(&'@').to_owned()), None, None, None),
            CCommand => Command::extractCCommand(cmd),
        };

        Some(Command{c_type: cmd_type, symbol: symbol, dest: dest, comp: comp, jump: jump})
    }

    fn extractCCommand(mut cmd: &str) -> (Option<~str>, Option<~str>, Option<~str>, Option<~str>) {
        cmd = cmd.trim_chars(& &['\n', '\r']);
        let dest_end_index = cmd.find_str("=");
        let jmp_start_index = cmd.find_str(";");
        let symbol = None;

        // We look for = (which means there is a dest), and if there is, we extract it
        let dest = match dest_end_index {
            Some(ind) => Some(cmd.slice_to(ind).to_owned()),
            None      => None 
        };

        // Same thing with ; and Jmp.
        let jump = match jmp_start_index {
            Some(ind) => Some(cmd.slice_from(ind+1).to_owned()),
            None      => None
        };

        // Then with the comp part, which is between = and ;
        let comp = match (dest_end_index, jmp_start_index) {
            (Some(d), Some(i)) => Some(cmd.slice(d+1, i).to_owned()),
            (Some(d), None)    => Some(cmd.slice_from(d+1).to_owned()),
            (None, Some(i))    => Some(cmd.slice_to(i).to_owned()),
            (None, None)       => Some(cmd.to_owned())
        };

        (symbol, dest, comp, jump)
    }
}

mod tests {
    #[test]
    fn test_cmd_from_str(){
        let test_cases = ["@154", "@test", "(Loop)", "M", "M=M+1", "A=D;JMP", "D;JEQ", "(Loop)//Test comment", "D=M+1;JEQ //test"];
        let test_results = [
        (&Some(~"154"), &None,          &None,         &None),
        (&Some(~"test"),&None,          &None,         &None),
        (&Some(~"Loop"),&None,          &None,         &None),
        (&None,         &None,          &Some(~"M"),   &None),
        (&None,         &Some(~"M"),    &Some(~"M+1"), &None),
        (&None,         &Some(~"A"),    &Some(~"D"),   &Some(~"JMP")),
        (&None,         &None,          &Some(~"D"),   &Some(~"JEQ")),
        (&Some(~"Loop"),&None,          &None,         &None),
        (&None,         &Some(~"D"),    &Some(~"M+1"), &Some(~"JEQ"))
        ];

        for (&test, &result) in test_cases.iter().zip(test_results.iter()) {
            let test_cmd = Command::from_str(test).unwrap();
            assert!((&test_cmd.symbol, &test_cmd.dest, &test_cmd.comp, &test_cmd.jump) == result);
        }
    }
}
