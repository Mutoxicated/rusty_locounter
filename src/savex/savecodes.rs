use lazy_static::lazy_static;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Display;

macro_rules! decodes {
    [$($k:expr => $v:ident),* ] => [
        [
            $( (String::from($k), SaveCode::$v) ),*
        ].iter().cloned().collect()
    ];
}

// macro_rules! encodes {
//     [$($k:ident => $v:expr),* ] => [
//         [
//             $( (SaveCode::$k, String::from($v)) ),*
//         ].iter().cloned().collect()
//     ];
// }

macro_rules! savecodes {
    ($($v:ident),*) => [
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum SaveCode {
            $($v),*
        }

        impl Display for SaveCode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let num = *self as usize;
                if num == 0 {
                    return f.write_str("000")
                }
        
                let mut a = (num as f32)/100.0;
                
                let numstr = num.to_string();
                let mut finished = "".to_owned();
        
                while a < 1.0 {
                    finished.push('0');
                    a *= 10.0
                }
                finished.push_str(numstr.as_str());
        
                f.write_str(finished.as_str())
            }
        }        

        lazy_static!{
            pub static ref DECODE_MAP:HashMap<String, SaveCode> = decodes!(
                $(SaveCode::$v.to_string() => $v),*
            );
    
            // pub static ref ENCODE_MAP:HashMap<SaveCode, String> = encodes!(
            //     $($v => SaveCode::$v.to_string()),*
            // );
        }
    ]
}

pub fn decode(str:&str) -> SaveCode {
    return *DECODE_MAP.get_key_value(str).unwrap().1
}

#[macro_export]
macro_rules! decoded {
    ($v:ident) => {
        decode($v.as_str())
    };
}

pub fn write_code(savecode:SaveCode) -> String {
    savecode.to_string()
}

#[macro_export]
macro_rules! code {
    ($e:tt) => {
        write_code(SaveCode::$e)
    };
}

savecodes![
    Na,
    Vec,
    WrapStart,
    WrapEnd,
    StringStart,
    StringEnd
];
