```plaintext
  ___  ____  ____  ____  ____  _  _ 
 / __)(  _ \(_  _)(  __)(  _ \( \/ )
( (_ \ ) __/  )(   ) _)  )   // \/ \
 \___/(__)   (__) (____)(__\_)\_)(_/
```

Lightweight cli powered by OpenAI to easily generate Linux commands through natural language inputs

---

# Introduction

How many of you have felt the tiredness of having to go back and forth from a browser to your terminal just to perform a simple task on linux just because you were not familiar with the command line tools? GPTerm is a command line tool that enables you to perform linux commands through natural language inputs. Just type,

```bash
gpterm build a new docker image with a Dockerfile located in ~/project/dockerfile/ named someone/my_awesome_image:latest
```

and you are bound to enjoy your evening with a nice cup of coffee.

# Quickstart

## Installation

### 1. Install Rust Compiler

Goto [rust website](https://www.rust-lang.org/tools/install) and install rust compiler.

### 2. Clone GPTerm repository

Run following command to download GPTerm repository in your home directory. `git` should be installed.

```bash
cd ~/ && git clone https://github.com/litcoderr/GPTerm.git && cd GPTerm/gpterm
```

### 3. Build and Install GPTerm

Run following command to install `gpterm`. `cargo` should be installed.

```bash
cargo install --path .
```

### 4. Configure API Key Environment Variable

To access OpenAI API, you need to generate an api key from [OpenAI Platform](https://platform.openai.com/docs/overview). After recieving your api key, paste following command in either `.bashrc` or `.zshrc` file depending on your shell.

```bash
export OPENAI_API_KEY="Your API key"
```

After pasting in your api key, run `source` to initialize the appended environment variable to current shell runtime.

```bash
source ~/.bashrc
```

or

```bash
source ~/.zshrc
```

## Tutorial

### 1. Query Desired Action in the Following Format

```bash
gpterm [desired action]
```

### 2. Select and Run a Desired Command

GPTerm will respond in the following format.

```bash
[1] `command 1` : description about command 1
[2] `command 2` : description about command 2
...
Enter Number (Ctrl-C to abort): 
```

After reviewing carefully the suggested commands, choose one command and enter in the number associated with it. Pressing `Enter` will run the selected command.

**Disclaimer: Some suggested commands might be destructive! Please review the commands very carefully!**

