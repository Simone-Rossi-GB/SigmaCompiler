use std::collections::HashMap;
use crate::parser::ast::Type;

pub struct CodeGenContext {

    // mi serve a mappare un nome di variabile con l'offset dallo stackpointer
    pub variables: HashMap<String, i32>,

    // mi serve per tracciare il tipo di ogni variabile
    pub variable_types: HashMap<String, Type>,

    // mi serve per tenere traccia dell'offset corrente dello stack (dovrebbe essere negativo e che sale)
    pub stack_offset: i32,

    // salvo le stringhe letterali da salvare in .data
    pub string_literals: Vec<String>,

    // contatore per le label uniche
    pub label_counter: usize,

    // stack di label di fine loop per gestire break
    pub loop_stack: Vec<String>,
}

impl CodeGenContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            variable_types: HashMap::new(),
            stack_offset: 0,
            string_literals: Vec::new(),
            label_counter: 0,
            loop_stack: Vec::new(),
        }
    }
    
    // funzione creata per allocare una variabile sullo stack
    // Ritorna offset POSITIVO dall'inizio del frame (dopo il prologo)
    pub fn allocate_variable(&mut self, name: String, var_type: Type) -> i32 {
        let offset = self.stack_offset;
        self.stack_offset += 4; // incremento 4 che è la grandezza di una word in riscv32
        self.variables.insert(name.clone(), offset);
        self.variable_types.insert(name, var_type);
        offset
    }

    pub fn get_variable_offset(&mut self, name: &str) -> Option<i32> {
        self.variables.get(name).copied()
    }

    pub fn get_variable_type(&self, name: &str) -> Option<&Type> {
        self.variable_types.get(name)
    }
    
    pub fn add_string_literal(&mut self, s: String) -> String {
        let label = format!("str_{}", self.string_literals.len());
        self.string_literals.push(s);
        label
    }
    
    pub fn generate_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    // Gestione loop per break (implementati da me porcodio)
    pub fn enter_loop(&mut self, end_label: String) {
        self.loop_stack.push(end_label);
    }

    pub fn exit_loop(&mut self) {
        self.loop_stack.pop();
    }

    pub fn current_loop_end(&self) -> Option<&String> {
        self.loop_stack.last()
    }
}