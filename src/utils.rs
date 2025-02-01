pub const NAME: &str = "MLTalk";
pub const ABOUT: &str = "ML-family programming language that you can write code smoothly";
pub const VERSION: &str = "0.1.0";

pub const SPACE: [&str; 5] = [" ", "　", "\n", "\t", "\r"];
pub const BEGIN: &str = "begin";
pub const END: &str = "end";

pub const RESERVED: [&str; 12] = [
    "print", "let", "const", "if", "then", "else", "for", "in", "do", "while", "loop", "fault",
];
pub const BUILTIN: [&str; 7] = [
    "type",
    "alphaConvert",
    "input",
    "readFile",
    "load",
    "sleep",
    "exit",
];

#[macro_export]
macro_rules! ok {
    ($option_value: expr, $fault_kind: expr) => {
        if let Some(ok) = $option_value {
            Ok(ok)
        } else {
            Err($fault_kind)
        }
    };
    ($option_value: expr) => {
        if let Some(ok) = $option_value {
            Ok(ok)
        } else {
            Err(Fault::Syntax)
        }
    };
}

#[macro_export]
macro_rules! join {
    ($x:expr) => {
        $x.join(&SPACE[0].to_string())
    };
    ($x:expr, $d: expr) => {
        $x.join($d)
    };
}

#[macro_export]
macro_rules! some {
    ($result_value: expr) => {
        if let Ok(ok) = $result_value {
            Some(ok)
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! trim {
    ($token: expr, $top: expr, $end: expr) => {
        ok!($token.get($top.len()..$token.len() - $end.len()))?
    };
}

#[macro_export]
macro_rules! remove {
    ($token: expr, $to_remove: expr) => {
        $token.replacen($to_remove, "", 1)
    };
}

#[macro_export]
macro_rules! char_vec {
    ($text: expr) => {
        $text
            .chars()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
    };
}

#[macro_export]
macro_rules! index_check {
    ($list: expr, $index: expr, $obj: expr) => {
        if !(0.0 <= $index && $index < $list.len() as f64) {
            return Err(Fault::Index(Value::Num($index), $obj.to_owned()));
        }
    };
}

#[macro_export]
macro_rules! repl_print {
    ($color: ident, $value: expr) => {
        println!("{navi} {}", $value, navi = "=>".$color().bold())
    };
}

#[macro_export]
macro_rules! fault {
    ($e: expr) => {
        repl_print!(red, format!("Fault: {:?}", $e))
    };
}

#[macro_export]
macro_rules! crash {
    ($result: expr) => {
        match $result {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {e:?}", "Fault".red());
                std::process::exit(1);
            }
        }
    };
}
