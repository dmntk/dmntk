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

//! # Command-line interface for DMNTK.
//!
//! Definitions of available command-line actions.

use dmntk_common::{Configuration, ServerConfiguration, DMNTK_COPYRIGHT, DMNTK_EXECUTABLE, DMNTK_NAME, DMNTK_VERSION};
use std::{env, path::Path};

/// Command name for recognizing decision table from text.
const CMD_RDT: &str = "rdt";

/// Command name for parsing decision table.
const CMD_PDT: &str = "pdt";

/// Command name for evaluating decision table.
const CMD_EDT: &str = "edt";

/// Command name for testing decision table.
const CMD_TDT: &str = "tdt";

/// Command name for parsing FEEL textual expression.
const CMD_PTX: &str = "ptx";

/// Command name for evaluating FEEL textual expression.
const CMD_ETX: &str = "etx";

/// Command name for starting platform as a service.
const CMD_SRV: &str = "srv";

/// Available command-line actions.
pub enum Action {
  /// Start decision table service in server mode.
  StartServer(ServerConfiguration),
  /// Recognize decision table loaded from text file.
  RecognizeDecisionTable(String),
  /// Parse decision table loaded from text file.
  ParseDecisionTable(String),
  /// Evaluate decision table loaded from `DTB` and `CTX` files.
  EvaluateDecisionTable(String, String),
  /// Test decision table loaded from `DTB` file.
  TestDecisionTable(String),
  /// Parse `FEEL` textual expression loaded from text file.
  ParseFeelTextualExpression(String, String),
  /// Evaluate `FEEL` textual expression loaded from text file.
  EvaluateFeelTextualExpression(String, String),
  /// Do nothing, no action was specified.
  NoAction,
}

/// Prints usage to standard output.
fn show_usage() {
  show_version();
  println!("USAGE:");
  println!("  {} [COMMAND] [FILES]", DMNTK_EXECUTABLE);
  println!("\nCOMMANDS:");
  println!("  {}  dtb_file             Recognize decision table defined in text file.", CMD_RDT);
  println!("  {}  dtb_file             Parse decision table defined in text file.", CMD_PDT);
  println!("  {}  dtb_file ctx_file    Evaluate decision table defined in text file.", CMD_EDT);
  println!("  {}  dtb_file             Test decision table defined in text file.", CMD_TDT);
  println!("  {}  feel_file ctx_file   Parse FEEL textual expression defined in text file.", CMD_PTX);
  println!("  {}  feel_file ctx_file   Evaluate FEEL textual expression defined in text file.", CMD_ETX);
  println!("  {}  config_file          Run DMNTK as a service.", CMD_SRV);
  println!("  -h, --help, help          Display help.");
  println!("  -v, --version, version    Display version.");
  println!();
}

/// Prints the name, version and copyright note to standard output.
fn show_version() {
  println!("\n{} v{}\n{}\n", DMNTK_NAME, DMNTK_VERSION, DMNTK_COPYRIGHT);
}

/// Prints help information to standard output.
fn show_help() {
  show_version();
  println!("Help: to be defined...\n");
}

/// Checks if there is single file name passed as an argument.
/// The specified name must be valid name of an existing file.
fn one_file(options: &[String]) -> bool {
  if options.len() == 2 {
    let path = Path::new(options[1].as_str());
    return path.exists() && path.is_file();
  }
  false
}

/// Checks if there are two file names passed as an argument.
/// The specified names have to be be valid names of existing files.
fn two_files(options: &[String]) -> bool {
  if options.len() == 3 {
    let path_one = Path::new(options[1].as_str());
    let path_two = Path::new(options[2].as_str());
    return path_one.exists() && path_one.is_file() && path_two.exists() && path_two.is_file();
  }
  false
}

/// Reads and parses configuration file.
fn get_config(config_file_name: String) -> Configuration {
  let err_read = format!("reading configuration file '{}' failed", config_file_name);
  let file_content = std::fs::read_to_string(&config_file_name).expect(&err_read);
  let err_parse = format!("parsing configuration file '{}' failed", config_file_name);
  serde_yaml::from_str(&file_content).expect(&err_parse)
}

/// Checks the list of arguments passed from the command line
/// and returns an action related to valid argument.
pub fn action() -> Action {
  let args: Vec<String> = env::args().collect::<Vec<String>>();
  if args.len() < 2 {
    show_usage();
    return Action::NoAction;
  }
  let options = &args[1..];
  match options[0].trim() {
    "-h" | "--help" | "help" => {
      show_help();
      Action::NoAction
    }
    "-v" | "--version" | "version" => {
      show_version();
      Action::NoAction
    }
    CMD_RDT if one_file(options) => Action::RecognizeDecisionTable(options[1].to_owned()),
    CMD_PDT if one_file(options) => Action::ParseDecisionTable(options[1].to_owned()),
    CMD_EDT if two_files(options) => Action::EvaluateDecisionTable(options[1].to_owned(), options[2].to_owned()),
    CMD_TDT if one_file(options) => Action::TestDecisionTable(options[1].to_owned()),
    CMD_PTX if two_files(options) => Action::ParseFeelTextualExpression(options[1].to_owned(), options[2].to_owned()),
    CMD_ETX if two_files(options) => Action::EvaluateFeelTextualExpression(options[1].to_owned(), options[2].to_owned()),
    CMD_SRV if one_file(options) => Action::StartServer(get_config(options[1].to_owned()).server),
    _ => {
      show_usage();
      Action::NoAction
    }
  }
}
