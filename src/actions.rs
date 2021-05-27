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

use {super::cli::Action, dmntk_common::Feelify, dmntk_feel::Scope};

pub fn action() {
  match crate::cli::action() {
    Action::ParseDecisionTable(dtb_file_name) => {
      //FIXME remove when implemented
      print!("Parsing decision table from file: {}", dtb_file_name);
      // parse_decision_table(&dtb_file_name);
    }
    Action::RecognizeDecisionTable(dtb_file_name) => {
      //FIXME remove when implemented
      print!("Recognizing decision table from file: {}", dtb_file_name);
      // recognize_decision_table_from_file(&dtb_file_name);
    }
    Action::EvaluateDecisionTable(dtb_file_name, ctx_file_name) => {
      //FIXME remove when implemented
      print!("Evaluating decision table from files: {}, {}", dtb_file_name, ctx_file_name);
      // evaluate_decision_table_from_file(dtb_file_name, ctx_file_name);
    }
    Action::TestDecisionTable(dtb_file_name) => {
      //FIXME remove when implemented
      print!("Testing decision table from file: {}", dtb_file_name);
      // test_decision_table_from_file(dtb_file_name);
    }
    Action::ParseFeelTextualExpression(feel_file_name, ctx_file_name) => {
      parse_textual_expression_from_file(&feel_file_name, &ctx_file_name);
    }
    Action::EvaluateFeelTextualExpression(feel_file_name, ctx_file_name) => {
      evaluate_textual_expression_from_file(&feel_file_name, &ctx_file_name);
    }
    Action::StartServer(server_config) => {
      //FIXME remove when implemented
      print!("Starting DMNTK server with configuration: {:?}", server_config);
      // server::start(server_config)
    }
    Action::NoAction => {
      // do nothing
    }
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
