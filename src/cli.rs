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

use clap::{load_yaml, App, AppSettings};
use dmntk_server::ServerConfiguration;

const DMNTK_NAME: &str = env!("CARGO_PKG_NAME");
const DMNTK_VERSION: &str = env!("CARGO_PKG_VERSION");
const DMNTK_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DMNTK_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Available command-line actions.
pub enum Action {
  /// Start decision table service in server mode.
  StartServer(ServerConfiguration),
  /// Recognize decision table loaded from text file.
  //RecognizeDecisionTable(String),
  /// Parse decision table loaded from text file.
  //ParseDecisionTable(String),
  /// Evaluate decision table loaded from `DTB` and `CTX` files.
  //EvaluateDecisionTable(String, String),
  /// Test decision table loaded from `DTB` file.
  //TestDecisionTable(String),
  /// Parse `FEEL` expression loaded from text file.
  ParseFeelExpression(String, String),
  /// Evaluate `FEEL` expression loaded from text file.
  EvaluateFeelExpression(String, String),
  /// Do nothing, no action was specified.
  DoNothing,
}

/// Checks the list of arguments passed from the command line
/// and returns an action related to valid argument.
pub fn action() -> Action {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml)
    .name(DMNTK_NAME)
    .version(DMNTK_VERSION)
    .author(DMNTK_AUTHORS.replace(":", "\n").as_str())
    .about(DMNTK_DESCRIPTION)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .get_matches();
  // parse FEEL expression subcommand
  if let Some(matches) = matches.subcommand_matches("pfl") {
    let context_file_name = matches.value_of("CONTEXT_FILE").unwrap();
    let expression_file_name = matches.value_of("EXPRESSION_FILE").unwrap();
    return Action::ParseFeelExpression(context_file_name.to_string(), expression_file_name.to_string());
  }
  // evaluate FEEL expression subcommand
  if let Some(matches) = matches.subcommand_matches("efl") {
    let context_file_name = matches.value_of("CONTEXT_FILE").unwrap();
    let expression_file_name = matches.value_of("EXPRESSION_FILE").unwrap();
    return Action::EvaluateFeelExpression(context_file_name.to_string(), expression_file_name.to_string());
  }
  // start server subcommand
  if let Some(matches) = matches.subcommand_matches("run") {
    let host = matches.value_of("host");
    let port = matches.value_of("port");
    let server_configuration = ServerConfiguration::new(host, port);
    return Action::StartServer(server_configuration);
  }
  Action::DoNothing
}
