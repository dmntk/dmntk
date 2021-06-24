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

pub extern crate dmntk_server;
extern crate serde;
#[cfg(feature = "bin")]
extern crate serde_derive;
extern crate serde_yaml;

#[cfg(feature = "bin")]
mod cli;

#[cfg(feature = "bin")]
mod actions;

#[cfg(not(feature = "bin"))]
fn main() {
  println!("Decision Model and Notation Toolkit needs be compiled with --features=bin");
}

/// Main entrypoint of the application.
#[cfg(feature = "bin")]
fn main() {
  actions::action()
}
