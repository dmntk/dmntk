/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * Copyright 2018-2021 Dariusz Depta Engos Software <dariusz.depta@engos.software>
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use {
  super::cli::Action,
  dmntk_common::{Feelify, Stringify},
  dmntk_feel::Scope,
};

pub fn action() {
  match crate::cli::action() {
    Action::ParseDecisionTable(dtb_file_name) => {
      // parses decision table from text file
      parse_decision_table(&dtb_file_name);
    }
    Action::RecognizeDecisionTable(dtb_file_name) => {
      // recognizes decision table loaded from text file
      recognize_decision_table_from_file(&dtb_file_name);
    }
    Action::EvaluateDecisionTable(dtb_file_name, ctx_file_name) => {
      // evaluates decision table and context loaded from file
      evaluate_decision_table_from_file(dtb_file_name, ctx_file_name);
    }
    Action::TestDecisionTable(dtb_file_name) => {
      // tests decision table loaded from file
      test_decision_table_from_file(dtb_file_name);
    }
    Action::ParseFeelTextualExpression(feel_file_name, ctx_file_name) => {
      // parses `FEEL` textual expression against the specified context
      parse_textual_expression_from_file(&feel_file_name, &ctx_file_name);
    }
    Action::EvaluateFeelTextualExpression(feel_file_name, ctx_file_name) => {
      // evaluates `FEEL` textual expression against the specified context
      evaluate_textual_expression_from_file(&feel_file_name, &ctx_file_name);
    }
    Action::StartServer(server_config) => {
      // starts REST server
      dmntk_server::start_server(server_config)
    }
    Action::NoAction => {
      // does nothing
    }
  }
}

/// Parses decision table loaded from file.
fn parse_decision_table(dtb_file_name: &str) {
  match std::fs::read_to_string(dtb_file_name) {
    Ok(text) => match dmntk_recognizer::scan(&text) {
      Ok(mut canvas) => {
        canvas.display_text_layer();
        canvas.display_thin_layer();
        canvas.display_body_layer();
        canvas.display_grid_layer();
        match canvas.plane() {
          Ok(plane) => println!("PLANE\n{}", plane),
          Err(reason) => println!("ERROR: {}", reason),
        };
      }
      Err(reason) => println!("ERROR: {}", reason),
    },
    Err(reason) => println!("loading decision table file `{}` failed with reason: {}", dtb_file_name, reason),
  }
}

/// Recognizes the decision table loaded from text file.
fn recognize_decision_table_from_file(dtb_file_name: &str) {
  match std::fs::read_to_string(dtb_file_name) {
    Ok(ref text) => match dmntk_recognizer::Recognizer::recognize(&text) {
      Ok(recognizer) => {
        recognizer.trace();
      }
      Err(reason) => println!("ERROR: {}", reason),
    },
    Err(reason) => println!("loading decision table file `{}` failed with reason: {}", dtb_file_name, reason),
  }
}

/// Evaluates context and decision table loaded from files.
fn evaluate_decision_table_from_file(dtb_file_name: String, ctx_file_name: String) {
  // read the context input from file
  match std::fs::read_to_string(ctx_file_name.as_str()) {
    Ok(ref context_input) => {
      // read decision table input from file
      match std::fs::read_to_string(dtb_file_name.as_str()) {
        Ok(ref decision_table_input) => {
          match dmntk_evaluator::evaluate_context_and_decision_table(decision_table_input, context_input) {
            Ok(value) => {
              // print the result
              println!("{}", value.stringify())
            }
            Err(reason) => println!("{}", reason),
          }
        }
        Err(reason) => println!("loading decision table file `{}` failed with reason: {}", dtb_file_name, reason),
      }
    }
    Err(reason) => println!("loading context file `{}` failed with reason: {}", ctx_file_name, reason),
  }
}

/// Tests decision table loaded from file.
fn test_decision_table_from_file(dtb_file_name: String) {
  match std::fs::read_to_string(dtb_file_name.as_str()) {
    Ok(dtb_input) => match dmntk_evaluator::evaluate_decision_table_test(&dtb_input, "%") {
      Ok((result, expected, actual)) => {
        if !result {
          println!("FAILURE");
          println!("Expected: {}", expected.stringify());
          println!("  Actual: {}", actual.stringify());
        } else {
          println!("SUCCESS!");
        }
      }
      Err(reason) => println!("{:?}", reason),
    },
    Err(reason) => println!("loading decision table file `{}` failed with reason: {}", dtb_file_name, reason),
  }
}

/// Parses `FEEL` textual expression loaded from file and prints the parsed AST to standard output.
fn parse_textual_expression_from_file(feel_file_name: &str, ctx_file_name: &str) {
  match std::fs::read_to_string(feel_file_name) {
    Ok(textual_expression) => match std::fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dmntk_evaluator::evaluate_context(&Scope::default(), &context_definition) {
        Ok(ctx) => match dmntk_feel_parser::parse_textual_expression(&ctx.into(), &textual_expression, false) {
          Ok(ast_root_node) => {
            println!("    AST:{}", ast_root_node.to_string().trim_end());
          }
          Err(reason) => println!("parsing textual expression failed with reason: {}", reason),
        },
        Err(reason) => println!("evaluating context failed with reason: {}", reason),
      },
      Err(reason) => println!("loading context file `{}` failed with reason: {:?}", ctx_file_name, reason),
    },
    Err(reason) => println!("loading textual expression file `{}` failed with reason: {:?}", feel_file_name, reason),
  }
}

/// Evaluates `FEEL` textual expression loaded from file and prints the result to standard output.
fn evaluate_textual_expression_from_file(feel_file_name: &str, ctx_file_name: &str) {
  match std::fs::read_to_string(feel_file_name) {
    Ok(textual_expression) => match std::fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dmntk_evaluator::evaluate_context(&Scope::default(), &context_definition) {
        Ok(ctx) => match dmntk_feel_parser::parse_textual_expression(&ctx.clone().into(), &textual_expression, false) {
          Ok(ast_root_node) => match dmntk_evaluator::evaluate(&ctx.into(), &ast_root_node) {
            Ok(result) => {
              println!("{}", result.feelify());
            }
            Err(reason) => println!("evaluating textual expression failed with reason: {}", reason),
          },
          Err(reason) => println!("parsing textual expression failed with reason: {}", reason),
        },
        Err(reason) => println!("evaluating context failed with reason: {}", reason),
      },
      Err(reason) => println!("loading context file `{}` failed with reason: {:?}", ctx_file_name, reason),
    },
    Err(reason) => println!("loading textual expression file `{}` failed with reason: {:?}", feel_file_name, reason),
  }
}
