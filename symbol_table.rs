use std::hashmap::HashMap;

pub struct SymbolTable {
    table: HashMap<~str, uint>,
    sp: uint
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut predef_map = HashMap::<~str, uint>::new();
        predef_map.insert(~"R0", 0);
        predef_map.insert(~"R1", 1);
        predef_map.insert(~"R2", 2);
        predef_map.insert(~"R3", 3);
        predef_map.insert(~"R4", 4);
        predef_map.insert(~"R5", 5);
        predef_map.insert(~"R6", 6);
        predef_map.insert(~"R7", 7);
        predef_map.insert(~"R8", 8);
        predef_map.insert(~"R9", 9);
        predef_map.insert(~"R10", 10);
        predef_map.insert(~"R11", 11);
        predef_map.insert(~"R12", 12);
        predef_map.insert(~"R13", 13);
        predef_map.insert(~"R14", 14);
        predef_map.insert(~"R15", 15);
        predef_map.insert(~"SP", 0);
        predef_map.insert(~"LCL", 1);
        predef_map.insert(~"ARG", 2);
        predef_map.insert(~"THIS", 3);
        predef_map.insert(~"THAT", 4);
        predef_map.insert(~"SCREEN", 16384);
        predef_map.insert(~"KBD", 24576);

        SymbolTable {table:predef_map , sp:16}
    }

    // Will either return the val of the symbol, or add a variable to the table to link to it
    pub fn get_symbol(&mut self, symbol: ~str) -> uint {
        if self.table.find(&symbol).is_some() {
            self.table.find(&symbol).unwrap().clone()      
        } else {
            self.table.insert(symbol, self.sp); 
            let old = self.sp; 
            self.sp += 1; 
            old
        }
    }

    // Returns None if the L-Symbol is already in the table, which lets upstairs deal with double definition of L-Symbols
    pub fn add_L_symbol(&mut self, symbol: ~str, val: uint) -> Option<uint> {
        if self.table.find(&symbol).is_some() {
            None
        } else {
            self.table.insert(symbol, val);
            Some(val)
        }
    }
}