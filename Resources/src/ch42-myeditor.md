# My Editor

## 1 install && unerstand

### 1.1 安装并新建项目

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
rustc --version

# 构建工具
cargo --version
# 创建一个不带git的项目
cargo init test --vcs none
```

### 1.2 理解Cargo创建的文件

src 目录
  : src是我们所有源代码文件的存放地方.

`Cargo.toml` && `Cargo.lock`
  : Cargo.toml 是 Rust 项目的“项目说明书 + 依赖清单 + 构建配置”.
    : 依赖管理: 依赖树, `Cargo.toml` 可以描述当前节点依赖项 SemVer
  : Cargo.lock (Cargo build --locked) 记录实际锁定使用的依赖版本.

target 目录
  :  cargo 支持多种构建目标,常见的是 debug (cargo build) 与 release (cargo build --release); 
  : `cargo clean;cargo build;cargo build` 
  : 如果连续构建2次, 第二次构建什么都不做, cargo build 不会重复编译,如果源文件发生变更, 才会发生编译.

`Compile` && `Run`
  : `cargo run` 编译并且运行 

## 2 terminal 

> [!NOTE]
> A terminal mode is one of a set of possible states of a terminal or pseudo terminal character device in Unix-like systems and determines how characters written to the terminal are interpreted. In cooked mode data is preprocessed before being given to a program, while raw mode passes the data as-is to the program without interpreting any of the special characters.

### 2.1 in cooked mode

```rust
use std::io::{self, Read};

fn main() {
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
    }
}
```

### 2.2 in raw mode

`cargo add crossterm` 引入第三方依赖.

enter raw mode 参考 [kilo](https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html)

```rust
use std::io::{self, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
			disable_raw_mode().unwrap();
            break;
        }
    }
}
```

### 2.3 in raw mode display keypress

打印每个read到的字符


```rust
fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
        }
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}
```

> [!NOTE]
> [Typewriter](https://en.wikipedia.org/wiki/Typewriter)
> 三个概念——铃铛、鼓的转动以及车厢的移动,在计算机中仍然存在,但从历史上看,这些概念是通过电传打字机进入计算机而形成的。返回此处:您可能已经注意到该打印的更多内容!确实开始了一条新线路,但并未从左侧开始,仿佛在打字机上我们只旋转了鼓,却没有移动车厢。我们通过将用于运输回程的信号(实际上仍按此名称)来修复,并将其添加到上述输出中。
> Bits (`0` 或 `1`) -> Byte (`01100001`) -> lookup ascii table -> character (`lowercase a`)
> ASCII代码0-31均为不可打印字符(您还可以找到铃铛、回车以及上方的线源),127 也是一个控制字符。ASCII 代码 32-126 都是可打印的。is_control() 因此只需查看代码的范围即可
> arrow keys, Escape, page up, page down, home, end, backspace, delete, enter, ctrl + a, ctrl + b etc..


### 2.4 Error Handling

> [!NOTE]
> 在许多其他编程语言中,某个函数节点出现错误, 它将不会返回,转而向前驱节点抛出一个异常,这意味着我们要么向上抛出异常,要么给一个特殊的返回值, 这两种都需要不同的代码片段处理,很难分析产生问题的原因,在rust中,使用Result (Ok, Err)来处理; .unwrap()在这里假设的是函数节点正常返回,如果不是这中情况,程序将会停止. 这里的处理尽可能向上抛出延迟处理,直到当前节点不得不处理,最终我们会终止程序.

## 3 Raw Input and Output 

### 3.1 Idiomatic code
> [!NOTE]
> 编写惯用代码有点像这样。关键在于遵循语言中常见的规则和做法,使你的代码易于阅读和理解。通常情况下,只有当某些内容无法正常工作时,人们才会深入你的代码——要么是因为存在漏洞,要么他们想添加一些新内容。因此,让代码易于理解,不仅有助于当前的协作者,也有助于在调试或添加更多功能时,帮助你未来的自我。

在不改变代码工作方式的情况下进行重构。我们的目标并非完美,而是不断改进,使我们的代码更易于操作和改进。为了确保我们的更改不会损坏任何内容,我们将手动测试当前情况。虽然自动化测试会很棒,但远超本教程中的内容。

### 3.2 Splitting the code into multiple files

在 Rust 中,与许多编程语言一样,主方法通常只需提供应用程序的入口来启动,而不会执行更多操作。我们的目标是将代码置于逻辑合理性的位置,以便在前进过程中更容易地找到并维护。这种方法还有很多其他好处,当我们遇到它们时,我会指出这些好处。

`struct` a group of variables and functions that come together to form a meaningful whole. it's exactly what we need to keep things neat and organized

了解一些引用的第三方库crossterm的常见struct
`KeyEvent` `KeyCoke`

了解rust中的clippy:
> [!NOTE]
> clippy 既是烦人的 Microsoft Office 97 功能的名称,也是用于指出代码可能改进的工具——即 linter。你可以通过以下命令行来运行它: 
> cargo clippy
> cargo clippy -- -W clippy::all  -W clippy::pedantic
> cargo clippy --fix -- -W clippy::all  -W clippy::pedantic
> 高度警惕状态: cargo clippy -- -W clippy::all -W clippy::pedantic  -W clippy::nursery -W clippy::cargo -W clippy::restriction

在main.rs顶部加入#![warnall, clippy::pedantic]

最终,我们的目标应该是毫无预警地编写代码,向未来的开发者表明,我们已努力打造出尽可能出色的代码。每当教程步骤涉及一种剪裁会皱眉的练习时,我都会确保在文本中加以说明。

### 3.3 improve Error Handling

让我们通过阐述关于错误处理的承诺,来总结一下对惯用代码的看法。我们的目标是:

    处理本地问题:如果能够解决发生错误,我们将负责处理。
    向上传递:对于无法直接修复的错误,我们会将它们传递到链条上以进行进一步处理。
    高层处理:如果错误达到最高级别,我们将尽可能优雅地管理它。

为此,我们将把错误处理逻辑与主循环分开,并打印出一条简单的告别消息。

### 3.4 使用`ALT + Q` 去quit

有时会使用q这个字符输入,所以q退出并不理想? 希望使用`ALT + Q` 去退出.
希望优雅退出使用should_quit: bool这个变量去标记退出, 实现优雅退出.

### 3.5 Clear the screen

当我们按下按键时,如何渲染页面?
  : 开始:我们为用户搭建了舞台。
  : 每次按压后:响应用户的行为。
  : 离开前:清理我们的工作区,并留下一条告别信息。

`print!("\x1b[2J");` \x 16进制表示; `1b` 表示 27; `[2J` 表示escape sequence [vt100](https://vt100.net/docs/vt100-ug/chapter3.html#ED);

现在让我们的代码更简洁、更有条理:
  : 最初,我们启用“原始模式”并清除屏幕,并设置编辑器。
  : 最后,我们进行清理:禁用原始模式,再次清除屏幕,处理任何错误,并告别用户。
  : 在两者之间,我们的循环会读取输入、评估、更新屏幕并重复操作——为未来的自己保持清晰且易于操作。最好趁代码仍可控时开始清理。

从现在起,我们将停止呼应每款按键,保持屏幕清晰和专注。

## 4 Assignment: 

[vim doc](http://www.vim.org/)

### 4.1 Assignment 1: Tidles

在屏幕左侧画一列 tidels (`~`) , 我们将在文本编辑器的任何一行开头画一个 `~`;
    1. 画的意思是我们移动光标,在需要打印的位置打印一个 `~`
    2. 实现一个draw_rows的函数, 使用这个函数打印 `~`, 在每一行
        参考 `crossterm::cursor::MoveTo` `crossterm::cursor::size`
    
help code:
```rust
fn main() {
    let somesize = (10,20);
    dbg!(somesize);
    dbg!(somesize.0);
    dbg!(somesize.1);
    for number in 0..10 {
        dbg!(number);
    }
}
```

因为向屏幕输出是一个昂贵的操作,为了更加高效地向屏幕输出, 其中存在缓冲的概念, 我们输入的字符会先进入缓冲, 缓冲区满自动向屏幕输出,或者手动控制不满的缓冲向屏幕输出.并清空缓冲区;
`execute!()` 宏是一个确保你的输入可以被立即打印出来的宏, 正如上面提到的问题, 写入是一个昂贵的操作,我们可以先将操作放入缓冲区,在合适的时间节点进行写入到屏幕上,不必每次都进行写入;

### 4.2 Assignment 2: Improved Drawing

光标闪烁的问题? 
    为了这种情况不发生,可以考虑在刷新屏幕时先隐藏光标, 在刷新完成后再进行显示.

每次刷新前不要清理整个屏幕,而是在redraw时,优雅地清除线.

help code:
```rust
Hide, Show, ClearType, Print, execute!(), queue!(), stdout.flush()
size() 返回turple, MoveTo() 使用x, y, 是否考虑使用struct, 定义height, width使得语义清晰
```

```rust
use std::io::stdout;
use std::io::Write; //Remove this line and it won't work

fn main() {
    print!("Testing!");
    stdout().flush().unwrap();
}
```

```rust
struct Size {
    height:u16
}

fn get_size() -> Size {
    Size{height:2}
}

fn print_size(size:Size) {
    let Size{height} = size;
    println!("The height is {}", height);
}
fn print_size_assessment(size:Size) {
    let Size{height} = size;
    if height < 5 {
        println!("It's small");
    } else {
        println!("It's tall");
    }
}

fn main() {
    let size = get_size();
    print_size(size);
    print_size_assessment(size);
}
```

Move Semantics & Ownership

```rust
#[derive(Debug)]
struct Hecto;
fn main() {
    let x = Hecto{};
    println!("x is {:?}",x); // This would prints out "x is Hecto"
    let y = x;
    println!("x is {:?}, and y is {:?}",x, y); // ... and this?
}
```

```rust
use std::fmt::{Debug, Result, Formatter};
struct Hecto;
impl Debug for Hecto {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Hecto is awesome.")
    }
}
fn main() {
    let x = Hecto;
    println!("{:?}",x);
}
```

Deriving and Using Traits

```rust
#[derive(Debug, Copy, Clone)]
struct Hecto;
fn main() {
    let x = Hecto{};
    println!("x is {:?}",x); // This would prints out "x is Hecto"
    let y = x;
    println!("x is {:?}, and y is {:?}",x, y); // ... and this?
}
```

### 4.3 Assignment 3: Welcome Message

也许是时候展示一条欢迎信息了。让我们在屏幕下方显示编辑器的名称和版本号,该屏幕位于终端中央。
```rust
fn main() {
    let division = 9/3;
    dbg!(division);
    
    let longer = "hecto".repeat(3);
    dbg!(longer);
    
    let slicing = &"hecto"[..2];
    dbg!(slicing);
    
    let length = "123".len();
    dbg!(length);
    
    let x:u16 = 1;
    let result = (length as u16) - x;
    dbg!(result);
}
```

Safe arithmetics and casting
```rust
#[allow(arithmetic_overflow)]
fn main() {
    let huge : u32 = std::u32::MAX; 
    dbg!(huge);
    dbg!(huge as u16);
    let small: u16 = 0;
    dbg!(small-1);
    dbg!(9/3);
    dbg!(10/3);
}
```
解决这个问题还有其他多种方法。
* 忽略它。极不可能有人面对这种情况,因为这种情况只有在极端边缘情况下才会发生。如果有的话。
* 崩溃。我们可以尝试将 usize 转换为 u16 或 u16,如果它不起作用,就会崩溃。
* 防止它。问题的一部分在于,我们无法控制人们传递给我们的立场。我们可以重构代码以执行职位的界限。
* 记录下来,然后继续前进。/// comments yourself `cargo doc --open`

关注一些概念:
    Document, Location, Cursor (Caret), Position (Column, Row), Point

### 4.4 Assignment 4: Caret Movement

 现在让我们关注一下意见。我们希望用户能够随意移动。
我们还希望了解当前caret的所在位置,因为在接下来的章节中,我们将把护理位置逐步调整为文档中要点的位置。

```rust
use core::cmp::min;
fn main() {
    let what_is_this:u8 = min(4,8);
    dbg!(what_is_this);
    match what_is_this {
        4 | 8 => {
            println!("It's one of the two numbers!");
        }
       _ => {
            println!("It's something else!");
        }
    }
}
```

### 4.5 Assignment 5: A poor Line Viewer

在程序中正确地处理文本是一件棘手的事情, 现在假设文本是一系列的字符串,例如"hello world". rust事实上有两种方式表示字符串 `str` 和 `String`. 我们将更多地与 `&str` 进行交互, 它是内存中字节序列的指针, 我们称 `str` 为字面字符串, 称 `&str` 为字符串切片.
当我们和字符串切片交互时,我们需要了解:
* 它是一个指向位于内存中的字节序列
* 许多字节被存储
* 任何 `str` 值总是一个有效的UTF-8
* 每个字节代表一个UTF-8数据点

```rust
fn main() {
    let slice: &str = "Hello, World";  
    dbg!(slice.as_ptr());              
    dbg!(slice.len());                 
    dbg!(slice.as_bytes());            
}
```

字符串实现了一个很棒的trait(Deref<target=str>), 这个trait可以保证你可以容易将 `String` 转换成 `&str`; 同时也意味着我们永远只需要保持与 `&str` 进行交互

```rust
fn prints_str(str: &str) {
    println!("I only print &strs, and the &str I got is: {str}");
}


fn main() {
    let slice: &str = "Hello!";
    prints_str(slice);
    
    let mut string: String = String::from("Hello!");
    string.pop();
    string.push_str(", World!");
    prints_str(&string);
}
```

任务如下:
* 引入一个新的 `struct` 名为 `View`
* 为 `View` 定义一个名为 `render()` 的方法
* 把之前 `draw_rows()` 的逻辑移动到 `render()` 中
* 调整逻辑使得第一行读出 `Hello world` (没有必要隐藏中间的editor的名称与版本号)
* 使用 `String` 与 `str` 的知识, 使得 `Terminal::print` 不再依赖一些 `trait`

核心概念:
1. `Editor`: 该组件主要在不同用户界面组件之间进行介导,从而在它们之间传递焦点和信息。目前,我们只有全尺寸的视图。
2. `View`: 该组件从编辑器接收每个与文本相关的事件,例如字符的按键、新行等。视图利用此信息来提升渲染效率,并将修改事件传递给缓冲区。
3. `Buffer`: 本组件将保留我们的文本。我们将从一个简单的文本表达开始,并逐步将其扩展到本教程中,以便根据需要容纳更多数据。

缓冲器是一种通用结构,它包含文本编辑器修改和显示文本文件所需的所有内容。视图与缓冲区交互以在屏幕上呈现。在许多文本编辑器中,你可以轻松地从一个缓冲区切换到下一个,从而实现并行打开多个文件。

缓冲区的实现细节因文本编辑器而异 
* Nano 采用一种直接的缓冲区结构,且文档没有其他内部表示。保存文件后,缓冲区内的内容(不包括用于语法高亮的颜色等数据)保存到磁盘。
* Vim 具有复杂的内部结构,能够高效处理操作,甚至适用于大文件,并包含一个独立的屏幕缓冲器,用于表示当前屏幕上可见的内容。

这些设计选择反映了每个编辑器的愿景:
* Nano 设计为一款小巧轻便的编辑器(因此名称!),适用于配置文件的快速更新。
* Vim 设计为一款功能齐全、功能丰富的文本编辑器,非常适合处理长文件。


什么是Vector, Victor?

```text
     我们的缓冲器需要动态调整大小——当物品被添加时增加,当物品被带走时会缩小——因此我们需要一种特殊的数据结构。这正是 Rust 中的字符串在内部使用称为 Vector 或 Vec 的结构所管理的。字符串内部有一个 Vec<u8>——一个字节的向量,本质上是一个。正如我们在第3章中所看到的,u8代表无符号的8位数,其大小基本上是字节。因此,当你在操作字符串时,实际上是在处理一个字节的向量。
     此VEC不仅存储字节,还确保其保存的任何数据都可以以“&str”形式返回,该数据引用一组UTF-8编码的Unicode字符序列。Vecs 旨在高效管理内存。例如:如果在循环中初始化一个Vec并持续添加元素,那么一个简单的方法可能涉及为每个添加的添加器分配新的内存,每次将所有现有内存都转移到此新空间中。这将是低效的
    我们需要一种数据结构,当你添加和删除项目时,它可以随之增长和缩小。字符串在引擎盖下使用此类数据结构,以修改其所保存的文本。这种数据结构称为 Vector,VEC。一个字符串在内部维护一个 u8 的 Vec,这是一种简洁的表达方式:它使用 Vec 的灵活数据结构来容纳 u8,因此没有符号的 8 位数。那是一笔麻烦。因此,基本上,它是一个字节向量,而String数据结构可确保Vec保存任意数据点,最终在需要时你会得到一个&str,该数据也会指向内存中的字节,但这些数据可确保属于UTF-8编码的Unicode点的一部分。2
    内部Vec试图确保内存效率高。考虑创建新 Vec 并持续循环添加场景。在简单的实现中,Vec 会为所有旧项和新项分配一些内存,并将旧项复制过去,但在内部,Vecs 确保此类数据复制无需频繁发生。
```

理解编程中的内存管理?

```text
计算机最小的内存单元是位数,表示为0或1。例如,正如我们之前所见,ASCII 字符 a 是 011001。为表示两个(aa),它们按顺序排列:011000010110001。了解每个角色的大小和起点至关重要,因为不同角色的位数可能相邻。

这就是指针的用处。它们像地图一样,指示数据的起始点和大小,使数据检索变得高效。考虑一个示例;指针只需要起始位置和一个字符的大小即可找到每个字符。

添加更多角色,比如aaa,并不简单。如果最后一个A的内存正在使用,你需要更多的空间。这需要从操作系统请求额外的内存以容纳新字符,然后复制到前面,然后添加下一个——这个过程比单纯使用可用内存更慢。

Rust 在编译器级别解决常见的内存管理挑战:
    它会阻止超出分配内存的写入。
    它能谨慎管理指针,以避免通过一个指针修改数据以及使用另一个指针访问过时数据等错误。
    Rust 的所有权模型(包括移动语义)可有效跟踪和管理指针,从而减少过时或相互冲突的数据访问中的错误。

与此相反,C语言需要手动内存管理,而Objective-C则使用参考计数(保留和释放)来管理内存。Java 和 JavaScript 采用垃圾收集器,可自动清理未引用的对象。

例如,JavaScript 使用字符串的指针数组,掩盖了开发人员的复杂性。尽管它以字符串数组的形式出现,但从技术上讲,它是指向字符串的指针数组,类似于 Rust 的 Vec<String>,但并未暴露底层指针。

继续我们讨论 Vecs 的内容,让我们来思考一下 Rust 的 Vec 如何处理数据:
如果每次将现有元素复制到新的内存位置,向 Vec 添加元素可能显得效率低下。相反,Vec 通过初始分配比当前所需的更多内存来优化此过程。这个额外的空间使Vec能够存储额外元素,而无需每次复制整个数组。只有当该容量得到充分利用时,才会分配更多空间并复制这些元素。因此,Vec 中的项数并不直接表示其使用的内存量;分配的内存通常超过当前需要,从而提升性能。该策略可平衡内存使用和处理效率。
```

在缓存中使用Vecs

```text
由于我们计划处理每个文件任意量的行,因此Vec 是我们使用的极佳数据结构。为了能够正确使用它,我们需要理解另一个小的概念和语法:
如上所述,VEC 不能保存任意数据。因此,我们谈不了一个Vec,就不能谈论它所持有的东西。我们通过在尖括号中添加该类型来表示。因此,字符串内部使用的数据结构是一个 Vec<u8> 。

fn main() {
    let mut vec: Vec<usize> = Vec::new();
    println!("The vec: {:?}",vec);
    for num in 0..100 {
        println!();
        println!("Step {}", num);
        vec.push(num);
        println!("The vec: {:?}", vec);
        println!("Its capacity: {}", vec.capacity());
        println!("Its length: {}", vec.len());
        println!("Item number {}: {}", num, vec.get(num).unwrap());
    }
}

```

### 4.6 Assignment 5:A Basic Line Viewer

错误处理:

```text
我们即将进入一个编码错误时不时发生panic的领域。这目前会产生令人讨厌的副作用。要模拟它,只需 panic!() 代码中的任意位置。panic!() 是一个有助于破坏程序的宏,当你做 Rust 不期望的事情时,就像切片超出边界一样,这种情况可能就完全一样了。

令人不安的是,当这种情况发生时,我们仍处于原始模式,这意味着我们的终端将全部陷入混乱。是时候让错误处理、禁用原始模式以及 Rust 中结构结构体的整个生命周期进行更仔细的观察。让我们先了解一下 “panic” 究竟意味着什么。
```

example 1:

```text
fn function_a() {
    function_b();
}

fn function_b() {
    function_c();
}

fn function_c() {
    println!("Inside function_c");
}

fn main() {
    function_a();
}

在此示例中,主调用函数_a,然后调用 function_b,从而导致函数_c输出一条消息。每个函数调用在调用栈上创建一个新帧,当每个函数完成后,其帧被移除,工作回到主帧。换句话说:当我们从main()转到function_c()时,堆栈会先通过一张纸来生成,描述main()的上下文,然后是function_a(),然后是function_b(),然后由println!生成的任何函数;在文本打印出来后,每张纸都逐一从堆栈中移除,直到我们返回main(),然后结束程序。

rust中使用panic!() 对于它不知道如何处理的错误。默认操作是开始解压堆栈,这意味着在重新向上链(或通过文件堆叠)进行清理。清理工作的一部分是打印出有关调用栈(在我们看来,它显示为一个堆栈跟踪)的信息,并处理变量和结构体。
或者,您也可以在发生恐慌时立即将 Rust 设置为求死执行,这可能会让程序变小,因为它会跳过展开。你在 Cargo.toml 中设置了这个,惊慌失措 = “流产”。这种方法速度更快,但要小心——可能会导致一些资源(如打开文件或网络连接)被挂断。
```



```text
现在是作业:
    创建一个名为 Buffer 的新结构,它具有 Vec<String> 键。
    确保此步骤中,Buffer 会保留一个包含 Hello,World 的字符串!
    将视图扩展为 buffer。
    使用此视图后,您需要将“视图”作为编辑器的一部分,并将渲染从关联函数更改为方法(即用于渲染(和自我)
    这反过来可能要求你实现或推导出“默认视图”和“缓冲区”。
    在渲染中更改逻辑:缓冲区中的每个条目都更改该条目。屏幕上每排空行,仍会渲染一个 ~。(您仍然可以保留欢迎信息,我们很快就会妥善处理)
```

### 4.7 Assignment 5: A poor Line Viewer

读取命令行的参数作为文件名称读取案例:

```rust
fn main() {
 let args: Vec<String> = std::env::args().collect();
 if let Some(first_arg) = args.get(1) {
    println!("Do something with the argument");
 } else {
    println!("No arg given");
 }
}
```

了解迭代器

```rust
pub fn main() {
    let range = 1..10;
    
    println!("First range");
    println!("{:?}", range);
    for n in range {
        println!("{:?}", n);
    }

    let second_range = 1..10;
     println!("Second range");
      println!("{:?}", second_range);
    for n in second_range.take(3) {
        println!("{:?}", n);
    }
     let third_range = 1..10;
    println!("Third range");
    println!("{:?}", third_range);
    for n in third_range.skip(2) {
        println!("{:?}", n);
    }
    
    let fourth_range = 1..10;
    let vec: Vec<u8> = fourth_range.skip(2).take(3).collect();
    println!("Vector");
    println!("{:?}", vec);
     for n in vec {
        println!("{:?}", n);
    }
}
```

任务如下:
```text
    如果调用 hecto 时没有任何命令行参数,则应显示之前的“欢迎屏幕”:左侧列中填充 ~,包名和版本以屏幕为中心。
    编辑器应检查文件名并将其传递给视图,例如到一种称为加载的方法。此方法应打开文件并将其内容加载到缓冲区中。(删除已用硬编码的“你好,世界!”)。
    你的缓冲器应该会暴露一种叫做 is_empty 的方法,如果缓冲区为空且不为正则返回。
    如果缓冲区为空,则应显示欢迎消息,否则应隐藏。
    空的线条仍应呈现为 ~。
```

### 4.8 Assignment 5: 

我们将在此作业中修复所有这些问题:

```text
    更新渲染策略以适当调整护理功能,从而直接去除打印效果 \r\n 。
    确保仅渲染当前行的子字符串。可以使用一个切片(即 current_row[0.width]),也可以利用以下事实:即 get 接受一个范围(即 current_row.get(0.width))——或者使用 String 的 truncate 函数,这是我们之前用于构建 wackland_message 的函数。
    在视图中引入一个需要的标志。如果属实,则需要重新绘制视图。如果为假,则无需渲染。将此标志设置在合适的位置,以确保仅在需要时才能进行渲染。
    奖励分配:即使终端尺寸调整,也需要对调整大小事件做出反应,并确保视图能够适当响应。
```








