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

#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde;
#[cfg(feature = "bin")]
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[cfg(feature = "bin")]
mod cli;

#[cfg(feature = "bin")]
use crate::cli::Action;

/// Main entrypoint of the library.
#[cfg(not(feature = "bin"))]
fn main() {
  println!("Decision Model and Notation Toolkit needs be compiled with --features=bin");
}

/// Main entrypoint of the application.
#[cfg(feature = "bin")]
fn main() {
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
    Action::ParseFeelTextualExpression(file_name) => {
      //FIXME remove when implemented
      print!("Parsing textual expression from file: {}", file_name);
      // parse_textual_expression_from_file(file_name);
    }
    Action::EvaluateFeelTextualExpression(file_name) => {
      //FIXME remove when implemented
      print!("Evaluating textual expression from file: {}", file_name);
      // evaluate_feel_expression_from_file(file_name);
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
