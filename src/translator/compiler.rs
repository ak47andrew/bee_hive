use crate::translator::intermediate_language::IR;
use crate::translator::memory_manager::MemoryManager;

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
            format!("{}<]", "+".repeat(value as usize))
        }
        IR::OUTPUT => {
            "[-+!#]>!>#![+-!].[-]".to_string()
        }
        IR::STORE_VARIABLE { .. } => {todo!()}
        IR::LOAD_VARIABLE => {todo!()}
    }
}

static CODE_CHARS: &str = ".,[]<>+-!#$";

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

    out.strip_suffix("\n\n").unwrap_or(out.as_str()).to_string()
}
