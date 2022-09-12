pub mod parser;
pub mod protocol_defs;
pub use parser::{ParseResult,ParsedFrame,BBBParse};

#[cfg(test)]
mod dev_tests;


#[cfg(test)]
mod tests {
use crate::protocol_defs::{self,methods::*,errors::*,CR}; 
    #[test]
    pub fn test_bbb_parse(){
        let mut underlying_data = Vec::<u8>::with_capacity(4096);
        let mut bbb_parse = crate::parser::BBBParse::new();
        
        // Test with full data      --SUCCESS
        let register_service:[u8;8] = [protocol_defs::START,REG_SERVICE,'T' as u8,'E' as u8,'S' as u8,'T' as u8,CR,CR];
        let register_caller:[u8;10] = [protocol_defs::START,REG_CALLER,'T' as u8,'E' as u8,'S' as u8,'T' as u8,CR,CR,0,0];
        let callresp:[u8;14]  = [protocol_defs::START,CALLRESP,0x20,0x20,4,CR,0xDE,0xAD,0xBE,0xEF,CR,CR,0,0]; 
        let call:[u8;12]  = [protocol_defs::START,CALL,4,CR,0xDE,0xAD,0xBE,0xEF,CR,CR,0,0]; 
        let stop_service:[u8;4] =[protocol_defs::START,STOP_SERVICE,CR,CR];
        let stop_caller:[u8;4] =[protocol_defs::START,STOP_CALLER,CR,CR]; 
        for m in 1..12{
            let output  = bbb_parse.parse(&callresp[..m]);
            println!("{:#?}",output) ;   
        }
    }
}

