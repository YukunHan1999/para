# bash beginner guide

## 1 bash and bash script

### 1.1 command shell program

shell ----command----> kernel

Describe some common shells [sh, bash, csh, tcsh, ksh]
    
### 1.2 Advantages of the Bourne Again Shell

#### 1.2.1 bash is the gnu shell
#### 1.2.2 features only found in bash
```bash
invocation
bash startup files
    login:
        /etc/profile
        ~/.bash_profile, ~/.bash_login, ~/.profile
        ~/.bash_logout
    nologin:
        ~/.bashrc  referred to ~/.bash_profile
```
#### 1.2.3 interactive shell $-
#### 1.2.4 conditionals
#### 1.2.5 shell arithmetic
#### 1.2.6 Aliases
#### 1.2.7 Arrays
#### 1.2.8 Directory stack
#### 1.2.9 the prompt
#### 1.2.10 the restricted shell

### 1.3 Executing commands

```bash
fork-and-exec mechanism:
    boot procedure -> init process(pid=1) -> fork other process   

shell built-in commands:
    bourne shell built-in commands: [:, ., break, cd, continue, eval, exec, exit, export, getopts, hash, pwd, readonly, return, set, shift, test, [, times, trap, umask and unset.]
    bash built-in commands: [alias, bind, builtin, command, declare, echo, enable, help, let, local, logout, printf, read, shopt, type, typeset, ulimit, unalias]
    special built-in commands:  posix mode [., break, continue, eval, exec, exit, export, readonly, return, set, shift, trap, unset]  
```

### 1.4 Building blocks

#### 1.4.1 shell building blocks

```bash
shell syntax
shell commands
shell functions
shell parameters
shell expansion
redirections
executing commands
shell scripts
```
    
### 1.5 develop good script

#### 1.5.1 properties of good script
#### 1.5.2 structure
#### 1.5.3 terminology
#### 1.5.4 a word on order and logic
#### 1.5.5 example

## 2 Writing and debugging scripts

### 2.1 Creating and running a script 
### 2.2 Script basics
```bash
specify shell  #!/bin/bash
add comments #
```
### 2.3 Debugging bash script
```bash
set -x 
x
set +x

bash -x script.sh
```

## 3 The Bash environment

### 3.1 Shell initialization files
#### 3.1.1 System-wide configuration files
```bash
/etc/profile
    /etc/inputrc
    /etc/profile.d/
/etc/bashrc
```
#### 3.1.2 Individual user configuration files
```bash
~/.bash_profile
~/.bash_login
~/.profile
~/.bashrc
~/.bash_logout
```

#### 3.1.3 changing shell configuration files

### 3.2 variables
#### 3.2.1 Types of variables
```bash
global variables
local variables
variables by content [string, integer, constant, array]
```
#### 3.2.2 creating variables
#### 3.2.3 exporting variables
#### 3.2.4 reserved variables
#### 3.2.5 special parameters
#### 3.2.6 script recycling with variables

### 3.3 Quoting charaters

#### 3.3.1 why
#### 3.3.2 escape character
#### 3.3.3 single quotes
#### 3.3.4 double quotes
#### 3.3.5 ansi-c quoting
#### 3.3.6 locales

### 3.4 shell expansion
#### 3.4.1 general
#### 3.4.2 brace expansion
#### 3.4.3 tidle expansion
#### 3.4.4 shell parameter and variable expansion
#### 3.4.5 command substitution
#### 3.4.6 arithmetic expansion
#### 3.4.7 process substitution
#### 3.4.8 work splitting
#### 3.4.9 file name expansion

### 3.5 aliases
### 3.6 more bash option
set -o

## 4 regular expressions

### 4.1 regular expressions

#### 4.1.1 what are regular expressions
#### 4.1.2 metacharacters
```bash
.
?
*
+
{N}
{N,}
{N,M}
-
^
$
\b
\B
\<
\>
```
#### 4.1.3 basic versus extended regular expressions

### 4.2 examples using grep

#### 4.2.1 what is grep
#### 4.2.2 grep ans regular expressions

### 4.3 pattern matching using bash features

#### 4.3.1 character ranges
#### 4.3.2 character classes

## 5 The GNU sed stream editor

### 5.1 what why how

### 5.2 interactive editing

#### 5.2.1 printing lines containing pattern
#### 5.2.2 deleting lines of input containing a pattern
#### 5.2.3 ranges of lines
#### 5.2.4 find and replace with sed

### 5.3 non-iteractive editing

#### 5.3.1 reading sed commands from a file
#### 5.3.2 writing output files

## 6 The GNU awk programing languages

### 6.1 what why how

### 6.2 the print program
#### 6.2.1 printing selected fields
#### 6.2.2 formatting fields
#### 6.2.3 the print command and regular expressions
#### 6.2.4 special patterns
#### 6.2.5 gawk scripts

### 6.3 Gawk variables

#### 6.3.1 the input field separator
#### 6.3.2 the output field separator
#### 6.3.3 the number of records
#### 6.3.4 user defined variables
#### 6.3.5 more examples
#### 6.3.6 the printf program

## 7 Conditional statements

### 7.1 if 
### 7.2 advanced if usage
### 7.3 using case statements

## 8 writing interactive scripts

#### 8.1 display user messages
#### 8.1 interactive or not
#### 8.2 using the echo built-in command

### 8.2 catching user input
#### 8.2.1 using the read built-in command
#### 8.2.2 prompting for user input
#### 8.2.3 redirections and file descriptions
#### 8.2.4 file input and output

## 9 repetitive tasks

### 9.1 the for loop
### 9.2 the while loop
### 9.3 the util loop
### 9.4 I/O redirections and loop
### 9.5 break and continue
### 9.6 making memus with the select built-in
### 9.7 the shift build-in

## 10 more on variables

### 10.1 types of variables
### 10.2 array variables
### 10.3 operations on variables

## 11 Functions

### 11.1 what why how
### 11.2 examples

## 12 catching signals

### 12.1 signals
### 12.2 traps

