# Test Rust

## 1. 基础练习

```bash
1. 使用cargo创建项目并运行
2. 声明多种类型变量并打印
    2.1. 尝试不同类型的变量运算
    2.2. 尝试类型转换后运算
    2.3. 尝试声名变量后不赋值然后直接使用
    2.4. 尝试对没有mut声明的变量进行修改
3. 声明一个函数并调用
    3.1. 尝试定义带参数的函数并调用
    3.2. 尝试定义带返回值的函数并调用
    3.3. 尝试函数返回值的不带return和;的方式
4. 定义一个字符串
    4.1. 尝试String.from("") || "".to_string() || format!("")
    4.2. 尝试push || push_str
    4.3. 尝试slice = &str
5. 所有权体会move, copy, borrow, borrow mutable
    5.1. move 使用函数String参数把所有权转移走后再使用
    5.2. copy 使用函数u32参数把所有权复制后再使用
    5.3. borrow 使用函数&String参数把所有权借用走后再使用
    5.4. borrow 使用函数&mut String参数把所有权借用走后clear原变量后再使用
    eg.
        String       move, not mutable
        &String      borrow, not mutable
        &str         borrow, not mutable
        &mut String  borrow, mutable eg. push || push_str
        &mut str     borrow, mutable eg. make_ascii_lowercase
6. If / Else
    6.1. 尝试 if {}
    6.2. 尝试 if {} else {}
    6.3. 尝试 if {} else if {} else {}
    6.4. 尝试 "三元表达式" let data = if { 5 } else { 6 }
7. Vec, Array, Slice
    Vec:
        7.1 Vec::new() || Vec::with_capacity(5) || v.push ||  v.len || v.capacity
        7.2 体验vec的扩容
        7.3 vec![1, 2, 3, 4, 5]
    Array:
        7.4 let array: [u32; 2] = [1, 2];
    Slice:
        7.5 定义一个函数带有&[u32]参数体验slice, 并尝试传入array试下, 尝试传入Vec试下
8. For, While, Loop
    8.1 使用for x in vec遍历Vec, 然后遍历后再尝试使用vec, 体验下所有权转移; 然后使用&vec体验下borrow
    8.2 使用 while condition {} 迭代变量 体验这种循环 break; 不修改迭代变量体验infinite loop
    8.3 使用Loop来体验infinite loop, 使用break 123; 返回循环结果并使用变量接收
9. 定义structs
    9.1 定义Struct Player {name, hp}, 声明实例并使用通过名称 player.name, player.hp
    9.2 定义Struct Position(u32, u32), 声明实例并使用通过索引 p.0, p.1
    9.3 为Struct添加行为, 定义&self的参数使用实例的状态 instance method
    9.4 为Struct添加行为, associated functions, 并返回Self类型
    9.5 为实例结构使用 let Player {pname, plocation, .. } = p; || fn test(Player {location: Position(x, y), name}: Player) {} 解构
10. Enum and Match
    10.1 使用enum State {Start, Running {hp: u32}, GameOver(Animation)} enum Animation {Running, Stopped,}定义一个枚举, 声明enum变量并使用
    10.2 使用match state { State::Start=> {} \n State::Running {ref mut hp}=>{} \n State::GameOver(Animation::Running) => {}, State::GameOver(Animation::Stopped) => {}, _ => {}} 类似switch
    10.3 可以接受match的返回值, 类似 "三元表达式" let data = match value {0..=5 => "", 6 => "", 7 => "", _ => ""};
11. Option<T>
    11.1 定义一个根据id查询name的函数, 存在查询不到的情况,用option包装 fn test(id: u32) -> Option<String> {}
    11.2 使用match处理Option, let data = match look_up(1) {Some(p) => p, None => return };
    11.3 只关注函数成功与否,会使用Option<()>类型作为函数返回值, 在函数中处理其他Option类型时可以使用?操作符简化,如果返回None,跳过后面所有指令直接返回None, 可以简化 let data = look_up(1)?; 返回也可以简化 Some(())
12. traits
    12.1 定义Struct Rect, Struct Circle, trait Drawable {fn draw(&self);} impl Drawable for Rect{}; impl Drawable for Circle{}; 使用vec![Box::new(rect), Box::new(circle)], 定义同trait的Vec<Box<dyn Drawable>>, 运行时自己推断;
    12.2 尝试在trait中定义方法默认实现
    12.3 尝试相同的struct中实现不同的trait, 且不同的trait中存在相同的方法, 尝试使用实例调用这个相同的方法, 如果不行,使用Drawable::draw(rect);调用再试试;
13. Generics
    13.1 定义struct使用泛型 struct World<T> {player:T}    struct Player {hp: u32}  impl Player {fn take_damage(&mut self, damage: u32) {}} 声明world, 并调用debug
    13.2 定义struct DebugPlayer {inner: Player} 包装Player, 并添加debug信息println
    13.3 使用泛型定义的struct作为参数时, fn use_world<T: Damage>(world: World<T>) {} 可以定义trait限制泛型的范围  修改为调用use_world
    13.4 为world添加方法时使用泛型 impl<T: Damage> World<T> {} 并再次修改调用,  修改为调用world中定义的方法
    13.5 定义多个泛型,使用common隔开 struct Thing<T, U> {a: T, b: U} trait: A, trait: B
14. Associates types
    选择时的判断标准: 
        你希望类型有调用方决定, 还是有实现方决定?  
        是否需要为同一个struct提供多种版本的实现?
        Generic => 1:N 调用方 
        Associate-type => 1:1 实现方
    不同视角的泛化:
        Generic 只用定义一个trait, 可以为一个struct提供不同具体类型的泛化
        Associate-type 只用定义一个trait, 因为每个struct只能对应一个类型, 所以同一个struct不能有多份实现, 但是不同的struct可以有不同的关联类型
```
