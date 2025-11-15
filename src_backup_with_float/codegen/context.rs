use std::collections::HashMap;

pub struct CodeGenContext {
    
    // mi serve a mappare un nome di variabile con l'offset dallo stackpointer
    pub variables: HashMap<String, i32>,
    
    // mi serve per tenere traccia dell'offset corrente dello stack (dovrebbe essere negativo e che sale)
    pub stack_offset: i32,
    
    // salvo le stringhe letterali da salvare in .data
    pub string_literals: Vec<String>,
    
    // contatore per le label uniche 
    pub label_counter: usize,
}

impl CodeGenContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            stack_offset: 0,
            string_literals: Vec::new(),
            label_counter: 0,
        }
    }
    
    // funzione creata per allocare una variabile sullo stack
    pub fn allocate_variable(&mut self, name: String) -> i32 {
        self.stack_offset -= 4; // sottraggo 4 che è la grandezza di una word in riscv32
        self.variables.insert(name ,self.stack_offset);
        self.stack_offset
    }
    
    pub fn get_variable_offset(&mut self, name: &str) -> Option<i32> {
        self.variables.get(name).copied()
    }
    
    pub fn add_string_literal(&mut self, s: String) -> String {
        let label = format!("str_{}", self.string_literals.len());
        self.string_literals.push(s);
        label
    }
    
    pub fn gen_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }
}