// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the contracts crate.

use thiserror::Error;

/// Errors arising from contract violations (invalid newtypes, bad formats).
#[derive(Debug, Error)]
pub enum ContractErrorChirho {
    #[error("Empty identifier for field '{field_chirho}'")]
    EmptyIdentifierChirho { field_chirho: String },

    #[error("Invalid format for '{field_chirho}': got '{value_chirho}', expected {expected_chirho}")]
    InvalidFormatChirho {
        field_chirho: String,
        value_chirho: String,
        expected_chirho: String,
    },

    #[error("Out of range: {field_chirho} = {value_chirho} (valid: {min_chirho}..={max_chirho})")]
    OutOfRangeChirho {
        field_chirho: String,
        value_chirho: i64,
        min_chirho: i64,
        max_chirho: i64,
    },
}

/// Convenience result type for contract operations.
pub type ContractResultChirho<T> = Result<T, ContractErrorChirho>;

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_empty_identifier_error_message_chirho() {
        let err_chirho = ContractErrorChirho::EmptyIdentifierChirho {
            field_chirho: "module_name".to_string(),
        };
        let msg_chirho = format!("{}", err_chirho);
        assert!(msg_chirho.contains("module_name"));
        assert!(msg_chirho.contains("Empty"));
    }

    #[test]
    fn test_invalid_format_error_message_chirho() {
        let err_chirho = ContractErrorChirho::InvalidFormatChirho {
            field_chirho: "strong_number".to_string(),
            value_chirho: "X999".to_string(),
            expected_chirho: "H/G prefix".to_string(),
        };
        let msg_chirho = format!("{}", err_chirho);
        assert!(msg_chirho.contains("X999"));
        assert!(msg_chirho.contains("strong_number"));
    }

    #[test]
    fn test_out_of_range_error_message_chirho() {
        let err_chirho = ContractErrorChirho::OutOfRangeChirho {
            field_chirho: "chapter".to_string(),
            value_chirho: 0,
            min_chirho: 1,
            max_chirho: 150,
        };
        let msg_chirho = format!("{}", err_chirho);
        assert!(msg_chirho.contains("chapter"));
        assert!(msg_chirho.contains("0"));
    }

    #[test]
    fn test_error_is_send_sync_chirho() {
        fn assert_send_sync_chirho<T: Send + Sync>() {}
        // ContractErrorChirho should be Send (for error propagation across threads)
        assert_send_sync_chirho::<ContractErrorChirho>();
    }

    #[test]
    fn test_contract_result_type_chirho() {
        let ok_chirho: ContractResultChirho<i32> = Ok(42);
        assert_eq!(ok_chirho.unwrap(), 42);

        let err_chirho: ContractResultChirho<i32> = Err(ContractErrorChirho::EmptyIdentifierChirho {
            field_chirho: "test".to_string(),
        });
        assert!(err_chirho.is_err());
    }
}
