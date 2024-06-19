use crate::{decoded, vec_node};

use super::savecodes::{decode, SaveCode};
use super::node::{Node, VecNode};

pub struct Extracter {
    current:usize,
    buf:Vec<char>
}

impl Extracter {
    pub fn new(buf:&str) -> Self {
        Self {
            buf: buf.chars().collect(),
            current:0
        }
    }

    pub fn extract(&mut self) {
        while self.current < self.buf.len() {
            let string = self.consume();
            let code = decode(string.as_str());
            if code == SaveCode::Vec {

            }
        }
    }

    fn evaluate_code<T>(&mut self, code:SaveCode) -> Node<T> {
        match code {
            SaveCode::Vec => {
                self.consume();//wrapb start
                let next = self.consume();
                if decoded!(next) != SaveCode::WrapEnd {
                    let next:Node = self.evaluate_code(decoded!(next));
                    let nextcode = next.code();

                    let contents:VecNode<T> = VecNode { 
                        wrapped_type: nextcode, 
                        contents: vec![
                            next
                        ]
                    };
                    return contents
                }
            }
        }
    }



    fn consume(&mut self) -> String {
        let mut temp:String = String::new();
        //buf[self.current] != buf.len()-1
        if self.current == self.buf.len()-1 {
            let str = self.buf[self.current].to_string();
            self.current += 1;
            return str
        }
        if self.buf[self.current].is_alphanumeric() {
            while self.buf[self.current].is_alphanumeric() {
                temp.push(self.buf[self.current]);
                self.current += 1;
            }
            temp
        } else if self.buf[self.current].is_alphabetic(){
            while self.buf[self.current].is_alphabetic() {
                temp.push(self.buf[self.current]);
                self.current += 1;
            }
            temp
        } else{
            let str = self.buf[self.current].to_string();
            self.current += 1;
            str
        }
    }
}