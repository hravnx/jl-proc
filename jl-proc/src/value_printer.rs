use crate::ansi_color;
use serde_json::Value;
use std::io::{Result, Write};

/// Configuration for pretty printing JSON values
pub struct ValuePrinterConfig {
    /// Whether to use ANSI colors in output
    pub use_color: bool,
    /// Number of spaces per indentation level
    pub indent_size: usize,
    /// Maximum width before wrapping arrays/objects
    pub max_width: usize,
}

impl Default for ValuePrinterConfig {
    fn default() -> Self {
        Self {
            use_color: false,
            indent_size: 2,
            max_width: 80,
        }
    }
}

/// A pretty printer for serde_json::Value that supports colors and flexible formatting
pub struct ValuePrinter {
    config: ValuePrinterConfig,
    // Color codes - empty strings when color is disabled
    key_color: &'static str,
    string_color: &'static str,
    number_color: &'static str,
    boolean_color: &'static str,
    null_color: &'static str,
    punctuation_color: &'static str,
    reset_color: &'static str,
}

impl ValuePrinter {
    /// Create a new ValuePrinter with the given configuration
    pub fn new(config: ValuePrinterConfig) -> Self {
        let (
            key_color,
            string_color,
            number_color,
            boolean_color,
            null_color,
            punctuation_color,
            reset_color,
        ) = if config.use_color {
            (
                ansi_color!(fg: 33), // Blue for keys
                ansi_color!(fg: 2),  // Green for strings
                ansi_color!(fg: 11), // Yellow for numbers
                ansi_color!(fg: 9),  // Red for booleans
                ansi_color!(fg: 8),  // Gray for null
                ansi_color!(fg: 7),  // Light gray for punctuation
                ansi_color!(),       // Reset
            )
        } else {
            ("", "", "", "", "", "", "")
        };

        Self {
            config,
            key_color,
            string_color,
            number_color,
            boolean_color,
            null_color,
            punctuation_color,
            reset_color,
        }
    }

    /// Pretty print a JSON value to the given writer
    pub fn print<W: Write>(&self, writer: &mut W, value: &Value) -> Result<()> {
        self.print_value(writer, value, 0)
    }

    /// Print object contents without the top-level curly braces, with custom base indentation
    pub fn print_object_contents<W: Write>(
        &self,
        writer: &mut W,
        obj: &serde_json::Map<String, Value>,
        base_indent: usize,
    ) -> Result<()> {
        if obj.is_empty() {
            return Ok(());
        }

        // Check if we should format compactly
        let compact = self.should_format_compact_object(obj);

        let entries: Vec<_> = obj.iter().collect();

        if compact {
            self.write_indent(writer, base_indent)?;
            for (i, (key, value)) in entries.iter().enumerate() {
                if i > 0 {
                    write!(writer, "{}, {}", self.punctuation_color, self.reset_color)?;
                }
                write!(
                    writer,
                    "{}{}{}:{} ",
                    self.key_color, key, self.punctuation_color, self.reset_color
                )?;
                self.print_value(writer, value, base_indent)?;
            }
        } else {
            for (i, (key, value)) in entries.iter().enumerate() {
                self.write_indent(writer, base_indent)?;
                write!(
                    writer,
                    "{}{}{}:{} ",
                    self.key_color, key, self.punctuation_color, self.reset_color
                )?;
                self.print_value(writer, value, base_indent + 1)?;
                if i < entries.len() - 1 {
                    write!(writer, "{},{}", self.punctuation_color, self.reset_color)?;
                }
                writeln!(writer)?;
            }
        }

        Ok(())
    }

    /// Internal method to print a value at a given indentation level
    fn print_value<W: Write>(&self, writer: &mut W, value: &Value, indent: usize) -> Result<()> {
        match value {
            Value::Null => {
                write!(writer, "{}null{}", self.null_color, self.reset_color)?;
            }
            Value::Bool(b) => {
                write!(writer, "{}{}{}", self.boolean_color, b, self.reset_color)?;
            }
            Value::Number(n) => {
                write!(writer, "{}{}{}", self.number_color, n, self.reset_color)?;
            }
            Value::String(s) => {
                write!(writer, "{}\"{}\"{}", self.string_color, s, self.reset_color)?;
            }
            Value::Array(arr) => {
                self.print_array(writer, arr, indent)?;
            }
            Value::Object(obj) => {
                self.print_object(writer, obj, indent)?;
            }
        }
        Ok(())
    }

    /// Print a JSON array
    fn print_array<W: Write>(&self, writer: &mut W, arr: &[Value], indent: usize) -> Result<()> {
        write!(writer, "{}[{}", self.punctuation_color, self.reset_color)?;

        if arr.is_empty() {
            write!(writer, "{}]{}", self.punctuation_color, self.reset_color)?;
            return Ok(());
        }

        // Check if we should format compactly
        let compact = self.should_format_compact_array(arr);

        if compact {
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    write!(writer, "{}, {}", self.punctuation_color, self.reset_color)?;
                }
                self.print_value(writer, item, indent)?;
            }
        } else {
            for (i, item) in arr.iter().enumerate() {
                writeln!(writer)?;
                self.write_indent(writer, indent + 1)?;
                self.print_value(writer, item, indent + 1)?;
                if i < arr.len() - 1 {
                    write!(writer, "{},{}", self.punctuation_color, self.reset_color)?;
                }
            }
            writeln!(writer)?;
            self.write_indent(writer, indent)?;
        }

        write!(writer, "{}]{}", self.punctuation_color, self.reset_color)?;
        Ok(())
    }

    /// Print a JSON object
    fn print_object<W: Write>(
        &self,
        writer: &mut W,
        obj: &serde_json::Map<String, Value>,
        indent: usize,
    ) -> Result<()> {
        write!(writer, "{}{{{}", self.punctuation_color, self.reset_color)?;

        if obj.is_empty() {
            write!(writer, "{}}}{}", self.punctuation_color, self.reset_color)?;
            return Ok(());
        }

        // Check if we should format compactly
        let compact = self.should_format_compact_object(obj);

        let entries: Vec<_> = obj.iter().collect();

        if compact {
            for (i, (key, value)) in entries.iter().enumerate() {
                if i > 0 {
                    write!(writer, "{}, {}", self.punctuation_color, self.reset_color)?;
                }
                write!(
                    writer,
                    "{}{}{}:{} ",
                    self.key_color, key, self.punctuation_color, self.reset_color
                )?;
                self.print_value(writer, value, indent)?;
            }
        } else {
            for (i, (key, value)) in entries.iter().enumerate() {
                writeln!(writer)?;
                self.write_indent(writer, indent + 1)?;
                write!(
                    writer,
                    "{}{}{}:{} ",
                    self.key_color, key, self.punctuation_color, self.reset_color
                )?;
                self.print_value(writer, value, indent + 1)?;
                if i < entries.len() - 1 {
                    write!(writer, "{},{}", self.punctuation_color, self.reset_color)?;
                }
            }
            writeln!(writer)?;
            self.write_indent(writer, indent)?;
        }

        write!(writer, "{}}}{}", self.punctuation_color, self.reset_color)?;
        Ok(())
    }

    /// Write indentation spaces
    fn write_indent<W: Write>(&self, writer: &mut W, level: usize) -> Result<()> {
        for _ in 0..(level * self.config.indent_size) {
            write!(writer, " ")?;
        }
        Ok(())
    }

    /// Determine if an array should be formatted compactly (on one line)
    fn should_format_compact_array(&self, arr: &[Value]) -> bool {
        // Compact if empty or all elements are simple (non-container) values
        arr.is_empty()
            || arr.iter().all(|v| {
                matches!(
                    v,
                    Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_)
                )
            })
    }

    /// Determine if an object should be formatted compactly (on one line)
    fn should_format_compact_object(&self, obj: &serde_json::Map<String, Value>) -> bool {
        // Compact if empty or all values are simple and the total estimated length is reasonable
        if obj.is_empty() {
            return true;
        }

        let all_simple = obj.values().all(|v| {
            matches!(
                v,
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_)
            )
        });

        if !all_simple {
            return false;
        }

        // Rough estimate of output length
        let estimated_len: usize = obj
            .iter()
            .map(|(k, v)| k.len() + self.estimate_value_length(v) + 5) // +5 for quotes, colon, comma, spaces
            .sum();

        estimated_len < self.config.max_width
    }

    /// Estimate the display length of a value (for compact formatting decisions)
    fn estimate_value_length(&self, value: &Value) -> usize {
        match value {
            Value::Null => 4,        // "null"
            Value::Bool(true) => 4,  // "true"
            Value::Bool(false) => 5, // "false"
            Value::Number(n) => n.to_string().len(),
            Value::String(s) => s.len() + 2,      // +2 for quotes
            Value::Array(arr) => arr.len() * 10,  // rough estimate
            Value::Object(obj) => obj.len() * 20, // rough estimate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_values() {
        let printer = ValuePrinter::new(ValuePrinterConfig::default());
        let mut output = Vec::new();

        // Test null
        printer.print(&mut output, &Value::Null).unwrap();
        assert_eq!(String::from_utf8(output.clone()).unwrap(), "null");
        output.clear();

        // Test boolean
        printer.print(&mut output, &json!(true)).unwrap();
        assert_eq!(String::from_utf8(output.clone()).unwrap(), "true");
        output.clear();

        // Test number
        printer.print(&mut output, &json!(42)).unwrap();
        assert_eq!(String::from_utf8(output.clone()).unwrap(), "42");
        output.clear();

        // Test string
        printer.print(&mut output, &json!("hello")).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "\"hello\"");
    }

    #[test]
    fn test_simple_array() {
        let printer = ValuePrinter::new(ValuePrinterConfig::default());
        let mut output = Vec::new();

        printer.print(&mut output, &json!([1, 2, 3])).unwrap();
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "[1, 2, 3]");
    }

    #[test]
    fn test_simple_object() {
        let printer = ValuePrinter::new(ValuePrinterConfig::default());
        let mut output = Vec::new();

        printer
            .print(&mut output, &json!({"a": 1, "b": 2}))
            .unwrap();
        let result = String::from_utf8(output).unwrap();
        // Note: HashMap iteration order is not guaranteed, so we check for both possibilities
        assert!(result == "{a: 1, b: 2}" || result == "{b: 2, a: 1}");
    }

    #[test]
    fn test_nested_structure() {
        let printer = ValuePrinter::new(ValuePrinterConfig::default());
        let mut output = Vec::new();

        let nested = json!({
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25}
            ],
            "count": 2
        });

        printer.print(&mut output, &nested).unwrap();
        let result = String::from_utf8(output).unwrap();

        // Should have multi-line formatting for complex structures
        assert!(result.contains('\n'));
        assert!(result.contains("users"));
        assert!(result.contains("Alice"));
    }

    #[test]
    fn test_object_contents_without_braces() {
        let printer = ValuePrinter::new(ValuePrinterConfig::default());
        let mut output = Vec::new();

        let obj = json!({"key1": "value1", "key2": 42})
            .as_object()
            .unwrap()
            .clone();

        printer.print_object_contents(&mut output, &obj, 2).unwrap();
        let result = String::from_utf8(output).unwrap();

        // Should not contain curly braces
        assert!(!result.contains('{'));
        assert!(!result.contains('}'));
        // Should have proper indentation (4 spaces = 2 * indent_size)
        assert!(result.starts_with("    "));
        // Should contain the key-value pairs
        assert!(result.contains("key1"));
        assert!(result.contains("value1"));
        assert!(result.contains("key2"));
        assert!(result.contains("42"));
    }
}
