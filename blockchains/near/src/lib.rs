use rust_crypto_core_chain::Chain;
use parser::ParserFactory;

mod error;
mod parser;
mod primitives;

#[cfg(test)]
use hex::FromHex;

struct Near;

impl Near {
    fn parse_formatted_data(data: &Vec<u8>) -> Result<String, String> {
        match ParserFactory::create_parser().deserialize(data) {
            Ok(parser) => parser.get_formatted_json().map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string())
        }
    }
}

impl Chain for Near {
    fn parse(data: &Vec<u8>) -> Result<String, String> {
        match ParserFactory::create_parser().deserialize(data) {
            Ok(parser) => parser.get_raw_json().map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let json = r#"{"actions":[{"Transfer":{"deposit":"200000000000000000000000"}}],"block_hash":"8L6Auh4ioZrR22vTxykBVNhEcojeDfmUoTr4tYb1QToR","hash":"ff978ce08fb7a53c3b4f91a08aa8cd10be43d972b38faa7da092b0ae92553929","nonce":93166702000009,"public_key":"ed25519:6yPGj6N27Wfaa3rDzPyWEEwyTy2ZrFa9L8N22m1hTi58","receiver_id":"demo0617.testnet","signer_id":"58bc2459804d2ed87641fbde40b0d96341cbf0313b7df4bc4f60fa2f42c602c3"}"#;
        let data = "40000000353862633234353938303464326564383736343166626465343062306439363334316362663033313362376466346263346636306661326634326336303263330058bc2459804d2ed87641fbde40b0d96341cbf0313b7df4bc4f60fa2f42c602c389772d10bc5400001000000064656d6f303631372e746573746e65746ce5b0c72ea21d29c9cf8cde859d2ddd466a70e1f8f1069742876e259fb157440100000003000000ed95c28f055a2a000000000000";
        let mut buf_message = Vec::from_hex(data).unwrap();
        assert_eq!(json, Near::parse_formatted_data(&mut buf_message).unwrap_or_else(|e| e));
    }
}

