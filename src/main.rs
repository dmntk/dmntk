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

//! Entrypoint of `DMNTK`.

extern crate clap;
extern crate dmntk_common;
extern crate dmntk_evaluator;
extern crate dmntk_feel;
extern crate dmntk_feel_parser;
extern crate dmntk_recognizer;
extern crate dmntk_server;
extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;

mod actions;
mod cli;

/// Main entrypoint of `DMNTK`.
fn main() {
  actions::action()
}
