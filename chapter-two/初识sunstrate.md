# 快速启动

## 单节点

1. 下载源码
2. `cargo build —release` 编译项目
3. `./target/release/substrate —dev` 运行，dev默认单节点
4. 打开浏览器 https://polkadot.js.org/apps 并连接本地节点
5. ` ./target/release/node-template purge-chain --dev` 清除节点数据

## 多节点

1. 下载源码
2. 编译
3. `./target/release/substrate --alice --chain local` 启动第一个节点，此时是不会出块的
4. `./target/release/substrate --bob --chain local --base-path /tmp/bob` 启动第二个节点
5. 打开浏览器 https://polkadot.js.org/apps 并连接本地节点查看出块情况
6. `rm -rf /tmp/alice && rm -rf /tmp/bob` 清除节点数据

# 模块定义概览

```rust
// 导入依赖
use support::{decl_module, decl_storage, decl_event, ...};
// 定义关联类型
pub trait Trait: system::Trait {...}

// 声明Runtime的地方
// 声明存储
decl_storage! {...}
// 声明事件
decl_event! {...}
// 声明错误
decl_error! {...}
// 声明用户可以调用的runtime的方法
decl_module! {...}
// 用户不可调用，提供给当前pallet的一些工具方法
// 或者给其他模块调用的方法
impl<T: Trait> Module<T> {...}
```

## 引入和定义关联类型

`System::Trait` 是*Runtime*下基础的**pallet** 的`Trait`，在自定义`Trait`的时候几乎都会继承它。

```rust
pub trait Trait: system::Trait {
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>
}
```

在自定义的`trait`里面还可以自定义其他的关联类型，每个关联类型后面会有该类型的`trait`约束。

```rust
// From `system` pallet
pub trait Trait: `static + Eq + Clone {
  type Origin: ...
  type Call: ...
  type Index: ...
  type BlockNumber: ...
}
```

定义关联类型的目的是为了和其他模块产生交互。

## 定义存储

```rust
decl_storage! {
  trait Store for Module<T: Trait> as TemplateModule {
    // Here we are declaring a StorageValue, `Something` as an Option<u32>
    // `get(fn something)` defines a getter function
    // Getter called with `Self::thing()`
    Smoeting get(fn something): Option<u32>;
    // Here we are declaring a StorageMap `SomeMap` from an AccountId to a Hash.
    // Getter called with `Self::some_map(account_id)`
    SomeMap get(fn some_map): map hasher(identity) T::AccountId => u32;
  }
}
```



## 定义事件

```rust
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AcountId {
    /// event `SomethingStored` is declared with a parameter of the type `u32` and `AccountId`
    SomethingStored(u32, AccountId),
  }
);
```



## 定义可调用函数

```rust
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    fn deposit_event<T>() = default; // The default deposit_event definition
    
    // 声明用户可以调用的方法
    pub fn do_something(origin, something: u32) -> Result {
      let sender = ensure_signed(origin)?;
      <Something::put(something); // Put a value into a StorageValue
      Self::deposit_event(RawEvent::SomethingStored(something, who)); // Emit Event
      Ok(()) // Return Ok at the end of function
    }
  }
}
```



## 定义公共和私有函数

这里定义的时候是遵循`Rust`本身的语法的，需要其他模块用到的方法要定义为`pbu`。

```rust
impl<T :Trait> Module<T> {
  fn mint(to: T::AccountId, id: T::Hash) -> Result {...}
  fn transfer(from: T::AccountId, to: T::AccountId, id: T::Hash) -> Result {...}
}
```



# substrate-node-template

## 代码预览

### node



### Runtime





### 3

### 4

## 编译成功截图

![](./imgs/build-finished.jpg)

## 运行成功截图

![](./imgs/run-finished.jpg)

## 使用 M1 Mac 编译踩的坑



### 依赖库有问题

开始使用的是[v3.0.0+1](https://github.com/substrate-developer-hub/substrate-node-template/tree/v3.0.0+1)版本，编译会报如下错误：

```
error[E0609]: no field `__rip` on type `__darwin_arm_thread_state64`
   --> crates/runtime/src/traphandlers.rs:169:44
    |
169 |                     (*cx.uc_mcontext).__ss.__rip as *const u8
    |                                            ^^^^^ unknown field
    |
    = note: available fields are: `__x`, `__fp`, `__lr`, `__sp`, `__pc` ... and 2 others
```

一个依赖库又问题，在*cargo*下的如下路径：*.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/wasmtime-runtime-0.22.0/src/traphandlers.rs*

![](./imgs/Q-1.jpg)

需要把`__rip`改成`__pc`，如下图：

![](./imgs/A-1.jpg)

### 另一个没能解决的错误

第一个错误修改完成后，在后面的编译中遇到另一个错误，没能解决，如下图：

![](./imgs/Q-3.jpg)

## 如何运行成功的

最后又回到了官方文档，官方文档提示建议尝试最新的 substrate-node-template 代码：[latest](https://github.com/substrate-developer-hub/substrate-node-template/tree/latest)

切换到该tags下正常运行。（在前面的课程build substrate的时候是没有出现任何问题的，可能是substrate的依赖都比较新？）

# SubStrate学习网站

- Substrate 官方文档：https://substrate.dev/docs/en/
- Substrate 的 recipe ：https://substrate.dev/recipes/
- Substrate 的 rust docs ：https://substrate.dev/rustdocs/latest/sc_service/index.html
- Substrate 的 tutorial ：https://substrate.dev/en/tutorials
- Substrate Seminar：https://substrate.dev/en/seminar  (两周一次，周二 14:00)

