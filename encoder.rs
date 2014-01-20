use std::hashmap::HashMap;
use std::from_str::from_str;
use symbol_table::SymbolTable;

mod symbol_table;

pub struct Encoder<'a> {
    comp_map: HashMap<~str, ~[int]>,
    dest_map: HashMap<~str, ~[int]>,
    jump_map: HashMap<~str, ~[int]>,
    table: &'a mut SymbolTable
}

impl<'a> Encoder<'a> {
    pub fn new<'a>(table: &'a mut SymbolTable) -> Encoder<'a>{
        let mut comp_map = HashMap::<~str, ~[int]>::new(); 
        comp_map.insert(~"0",   ~[0, 1, 0, 1, 0, 1, 0]);
        comp_map.insert(~"1",   ~[0, 1, 1, 1, 1, 1, 1]);
        comp_map.insert(~"-1",  ~[0, 1, 1, 1, 0, 1, 0]);   
        comp_map.insert(~"D",   ~[0, 0, 0, 1, 1, 0, 0]);    
        comp_map.insert(~"A",   ~[0, 1, 1, 0, 0, 0, 0]);   
        comp_map.insert(~"!D",  ~[0, 0, 0, 1, 1, 0, 1]);
        comp_map.insert(~"!A",  ~[0, 1, 1, 0, 0, 0, 1]);   
        comp_map.insert(~"-D",  ~[0, 0, 0, 1, 1, 1, 1]);   
        comp_map.insert(~"-A",  ~[0, 1, 1, 0, 0, 1, 1]);   
        comp_map.insert(~"D+1", ~[0, 0, 1, 1, 1, 1, 1]);
        comp_map.insert(~"A+1", ~[0, 1, 1, 0, 1, 1, 1]);   
        comp_map.insert(~"D-1", ~[0, 0, 0, 1, 1, 1, 0]);    
        comp_map.insert(~"A-1", ~[0, 1, 1, 0, 0, 1, 0]);   
        comp_map.insert(~"D+A", ~[0, 0, 0, 0, 0, 1, 0]);
        comp_map.insert(~"D-A", ~[0, 0, 1, 0, 0, 1, 1]);   
        comp_map.insert(~"A-D", ~[0, 0, 0, 0, 1, 1, 1]);  
        comp_map.insert(~"D&A", ~[0, 0, 0, 0, 0, 0, 0]);   
        comp_map.insert(~"D|A", ~[0, 0, 1, 0, 1, 0, 1]);
        comp_map.insert(~"M",   ~[1, 1, 1, 0, 0, 0, 0]);   
        comp_map.insert(~"!M",  ~[1, 1, 1, 0, 0, 0, 1]);    
        comp_map.insert(~"-M",  ~[1, 1, 1, 0, 0, 1, 1]);   
        comp_map.insert(~"M+1", ~[1, 1, 1, 0, 1, 1, 1]);
        comp_map.insert(~"M-1", ~[1, 1, 1, 0, 0, 1, 0]);   
        comp_map.insert(~"D+M", ~[1, 0, 0, 0, 0, 1, 0]);   
        comp_map.insert(~"D-M", ~[1, 0, 1, 0, 0, 1, 1]);   
        comp_map.insert(~"M-D", ~[1, 0, 0, 0, 1, 1, 1]);
        comp_map.insert(~"D&M", ~[1, 0, 0, 0, 0, 0, 0]);   
        comp_map.insert(~"D|M", ~[1, 0, 1, 0, 1, 0, 1]);   

        let mut dest_map = HashMap::<~str, ~[int]>::new();
        dest_map.insert(~"null", ~[0, 0, 0]);   
        dest_map.insert(~"M",    ~[0, 0, 1]);   
        dest_map.insert(~"D",    ~[0, 1, 0]);   
        dest_map.insert(~"MD",   ~[0, 1, 1]);   
        dest_map.insert(~"A",    ~[1, 0, 0]);   
        dest_map.insert(~"AM",   ~[1, 0, 1]);   
        dest_map.insert(~"AD",   ~[1, 1, 0]);   
        dest_map.insert(~"AMD",  ~[1, 1, 1]);  

        let mut jump_map = HashMap::<~str, ~[int]>::new();
        jump_map.insert(~"null", ~[0, 0, 0]);   
        jump_map.insert(~"JGT",  ~[0, 0, 1]);   
        jump_map.insert(~"JEQ",  ~[0, 1, 0]);   
        jump_map.insert(~"JGE",  ~[0, 1, 1]);   
        jump_map.insert(~"JLT",  ~[1, 0, 0]);   
        jump_map.insert(~"JNE",  ~[1, 0, 1]);   
        jump_map.insert(~"JLE",  ~[1, 1, 0]);   
        jump_map.insert(~"JMP",  ~[1, 1, 1]);

        Encoder{comp_map: comp_map, dest_map: dest_map, jump_map: jump_map, table: table}
    }

    pub fn encode_comp(&self, comp: ~str) -> Option<~str> {
        match self.comp_map.find(&comp) {
            Some(v) => Some(v.iter().fold(~"", |mut acc, d| {acc = acc + d.to_str(); acc})),
            None    => None
        }
    }

    pub fn encode_dest(&self, dest: ~str) -> Option<~str> {
        match self.dest_map.find(&dest) {
            Some(v) => Some(v.iter().fold(~"", |mut acc, d| {acc = acc + d.to_str(); acc})),
            None    => None
        }
    }

    pub fn encode_jump(&self, jump: ~str) -> Option<~str> {
        match self.jump_map.find(&jump) {
            Some(v) => Some(v.iter().fold(~"", |mut acc, d| {acc = acc + d.to_str(); acc})),
            None    => None
        }
    }

    // address is a decimal number
    pub fn encode_ACommand(&mut self, address: ~str) -> Option<~str> {
        let mut b_str = match from_str::<int>(address) {
            Some(nb) => format!("{:t}", nb),
            None     => format!("{:t}", self.table.get_symbol(address))
        };

        if b_str.len() > 15 {
            return None;
        }

        while b_str.len() < 16 {
            b_str = ~"0" + b_str;
        }

        Some(b_str)
    }
}

mod tests {
    #[test]
    fn test_encode_comp() {
        let enc = Encoder::new();
        assert!(enc.encode_comp(~"M+1") == Some(~"1110111"));
    }

    #[test]
    fn test_encode_dest() {
        let enc = Encoder::new();
        assert!(enc.encode_dest(~"AMD") == Some(~"111"));
    }

    #[test]
    fn test_encode_jump() {
        let enc = Encoder::new();
        assert!(enc.encode_jump(~"JLT") == Some(~"100"));
    }

    #[test]
    fn test_encode_ACommand() {
        let enc = Encoder::new();
        assert!(enc.encode_ACommand(~"15") == Some(~"0000000000001111"));  
        assert!(enc.encode_ACommand(~"131072") == None);  
    }
}