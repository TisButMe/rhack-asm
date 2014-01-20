use std::os::args;
use parser::{Parser, LCommand, ACommand, CCommand};
use encoder::Encoder;
use symbol_table::SymbolTable;
use std::io::fs::File;
use std::path::posix::Path;
use std::io::BufferedWriter;

mod parser;
mod encoder;
mod symbol_table;

fn main() {
    let mut parser = match args().len() {
        i if i<2 => fail!("You must provide an input file"),
        _        => Parser::new(args()[1])
    };
    let mut writer = BufferedWriter::new(File::create(&Path::new(args()[1].split('.').next().unwrap() + ".hack")));

    let mut s_table = SymbolTable::new();
    update_table_with_L_symbols(&mut parser, &mut s_table);

    let mut enc = Encoder::new(&mut s_table);

    parser.rewind();

    loop {
        let line = match parser.command_type() {
            Some(LCommand) => ~"",
            Some(ACommand) => enc.encode_ACommand(parser.symbol().expect("A Command must have a symbol")).expect("A Command invalid"),
            Some(CCommand) => ~"111" + match (parser.comp(), parser.dest(), parser.jump()) {
                (Some(c), Some(d), Some(j)) => enc.encode_comp(c).expect("C bits invalid") + enc.encode_dest(d).expect("D bits invalid") 
                                                                                           + enc.encode_jump(j).expect("J bits invalid"),
                (Some(c), Some(d), None)    => enc.encode_comp(c).expect("C bits invalid") + enc.encode_dest(d).expect("D bits invalid") 
                                                                                           + enc.encode_jump(~"null").expect("J bits invalid"),
                (Some(c), None   , Some(j)) => enc.encode_comp(c).expect("C bits invalid") + enc.encode_dest(~"null").expect("D bits invalid") 
                                                                                           + enc.encode_jump(j).expect("J bits invalid"),
                _                           => fail!("Invalid command")
            },
            None           => ~""
        };

        if line.len() > 0 {
            writer.write_line(line);
        }

        match parser.advance() {
            Some(_) => {},
            None    => {println!("End of assembly"); break;}
        }
    }

    writer.flush();
}

fn update_table_with_L_symbols(parser: &mut Parser, table: &mut SymbolTable) {
    let mut line_counter = 0;

    loop {
        match parser.command_type() {
            Some(LCommand)                => {table.add_L_symbol(parser.symbol().expect("Invalid L Symbol"), line_counter);},
            Some(ACommand)|Some(CCommand) => line_counter += 1,
            _                             => {}
        }

        match parser.advance() {
            Some(_) => {},
            None    => {println!("First pass over"); break;}
        } 
    }
}