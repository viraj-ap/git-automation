
## Installation

First of all install [Rust](https://www.rust-lang.org/tools/install) in your system.

Onced it's installed, run the following commands in the terminal.

```bash
  git clone https://github.com/viraj-ap/git-automation.git
  cd git-automate
```
Build the project

```bash
  cargo build --release
```
Specify you path (Keep this outside you main directory)
```bash
cargo install --path <your-path-of this-project>
```




    
## Example

![alt text](demo/img.png)

Change the name of the command if you want Cargo.toml.
(I have set it to git-auto)

Once you are done just type this into terminal.
```
git-auto
```
and then enter the commit messaage

```
Enter commit message:
```
enter the message and the files are now addded to git.


