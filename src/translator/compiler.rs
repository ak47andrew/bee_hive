use crate::translator::intermediate_language::IR;
use crate::translator::tokenizer::BasicType;

fn translate_ir(instruction: IR) -> String {
    match instruction {
        IR::SET_POINTER { index  } => {
            format!(">!{}", (if index > 0 {">"} else {"<"}).repeat(index.abs() as usize))
        }
        IR::LOAD_IMMEDIATE_STRING { value } => {
            value
                .chars()
                .map(|c| format!("{}<", "+".repeat(c as usize)))
                .collect::<Vec<_>>()
                .join("\n")
        }
        IR::LOAD_IMMEDIATE_INTEGER { value } => {
            format!("{}<", "+".repeat(value as usize))
        }
        IR::OUTPUT {value_type} => {
            if value_type == BasicType::Void {
                println!("Something might've gone wrong. Compiler's trying to output a `Void` type. This can lead to UB, be aware!");
            }

            if value_type != BasicType::Char {
                "[-+!#]>!>#![+-!]>>+<<.[-]>>-".to_string()
            } else {
                "[-+!#]>!>#![+-!].[-]".to_string()
            }
        }
        // TODO: this is literally the same code! We should absolutely generalise that!
        // Cell to which one you want to store a variable, pointer at a value
        IR::STORE_VARIABLE { cell } => {
            format!("[-+!#]{}[-]#![+-!]", translate_ir(IR::SET_POINTER { index: cell }))
        }
        // Cell to which load variable, pointer at a value
        IR::LOAD_VARIABLE {cell} => {
            format!("[-+!#]{}#![+-!]", translate_ir(IR::SET_POINTER { index: cell }))
        }
        IR::INPUT {cell} => {
            format!(">![-],[-+!#]{}#![+-!]", translate_ir(IR::SET_POINTER { index: cell }))
        }
        IR::WAIT_FOR_INPUT => {
            ">!>>>>[]".to_string()
        }
    }
}

pub static CODE_CHARS: &str = ".,[]<>+-!#$";

pub fn codegen(irs: Vec<IR>) -> String {
    let mut out = String::new();

    for ir in irs {
        let mut comment = format!("{:?}", ir);
        comment.retain(|c| !CODE_CHARS.contains(c));
        let code = translate_ir(ir);
        out.push_str(&code);
        out.push_str("  // ");
        out.push_str(&*comment);
        out.push('\n');
    }

    out.trim().to_string()
}
