项目文档

## 环境设置

### 安装Rust

Rust是一个快速、内存安全的编程语言，非常适合智能合约的开发。要安装Rust，请访问[Rust官方网站](https://www.rust-lang.org/tools/install)并按照指引进行安装。安装完成后，你可以使用以下命令确认Rust版本：

```sh
rustc --version
```

### 安装Solana CLI

Solana CLI是与Solana区块链交互的命令行工具。你可以通过以下Solana官方文档来安装Solana CLI：[Solana CLI安装指南](https://docs.solana.com/cli/install-solana-cli-tools)。

### 安装Anchor

Anchor是一个用于Solana程序开发的框架，提供了丰富的工具和库以简化智能合约的开发。安装Anchor的命令如下：

```sh
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked
```

安装完成后，可以通过下面的命令来验证Anchor的安装：

```sh
anchor --version
```

## 合约部署

### 编译合约

首先，确保你的合约代码已经准备好。使用以下Anchor命令来编译合约：

```sh
anchor build
```

编译成功后，Anchor会在`target/deploy`目录下生成一个.so文件，这是你的合约编译结果。

### 部署合约

使用以下命令设置Solana CLI默认的网络（以devnet为例）：

```sh
solana config set --url https://api.devnet.solana.com
```

确保你的钱包有足够的SOL来支付部署费用。使用以下命令部署合约：

```sh
solana program deploy target/deploy/your_contract.so
```

部署成功后，Solana CLI会返回一个合约地址，这个地址代表了你的合约在Solana网络上的位置。

## 合约交互

### 设置Solana Web3.js

为了与合约交互，你需要在你的项目中安装Solana Web3.js库：

```sh
npm install @solana/web3.js
```

### 示例代码

以下是使用Solana Web3.js与合约交互的简单示例：

```javascript
const solanaWeb3 = require('@solana/web3.js');
const connection = new solanaWeb3.Connection(solanaWeb3.clusterApiUrl('devnet'));

async function getBalance(walletAddress) {
    let balance = await connection.getBalance(new solanaWeb3.PublicKey(walletAddress));
    console.log(`Balance: ${balance}`);
}

// 替换成你的钱包地址
const walletAddress = 'YourWalletAddressHere';
getBalance(walletAddress);
```

### 更新价格和设置地址

更新价格和设置地址的功能需要通过合约提供的方法实现，具体代码取决于合约的设计。

## 事件监听

你可以使用Solana的WebSocket服务来监听合约事件。具体实现方法取决于你的合约设计以及如何触发和记录事件。


### 事件处理

在Anchor中，事件是通过Rust的结构体定义的，然后使用`#[event]`宏标记。这允许开发者在合约的特定操作点记录事件，这些事件随后可以被客户端监听和处理。事件是在智能合约执行期间生成的，并且可以包含合约状态的有用信息，例如交易细节或合约状态的变更。

### 定义事件

首先，你需要定义一个事件。这是通过创建一个包含数据字段的Rust结构体完成的，然后将其与`#[event]`宏注解：

```rust
use anchor_lang::prelude::*;

#[event]
pub struct MyEvent {
    pub data: u64,
    // 其他字段...
}
```

### 触发事件

在合约逻辑中，当特定条件满足时，可以触发事件。这是通过调用`emit!`宏完成的：

```rust
emit!(MyEvent {
    data: 42,
    // 设置其他字段...
});
```

### 监听事件

客户端监听事件通常是通过订阅特定的WebSocket端点来完成的，该端点与Solana节点或网关（如Solana的公共API或项目自己的网关）通信。客户端库（如@solana/web3.js）提供了订阅事件的功能。
这个就是合约部署的ProgramId，一般在lib.rs文件的入口
<img width="570" alt="image" src="https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/d01d8268-2360-46be-ac2f-d0717dcdc724">

以下是使用JavaScript（假设使用`@solana/web3.js`）监听Anchor定义的事件的简单示例：

```javascript
const connection = new solanaWeb3.Connection(
  solanaWeb3.clusterApiUrl('devnet'),
  'confirmed',
);

const myProgramId = new solanaWeb3.PublicKey('MyProgramPublicKey...');
const subscriptionId = connection.onLogs(
  myProgramId,
  (logs, context) => {
    for (const log of logs.logs) {
      if (log.includes('MyEvent')) {
        console.log('Event detected:', log);
        // 进一步处理日志信息...
      }
    }
  },
  'confirmed',
);

// 在适当的时候取消订阅
// connection.removeOnLogsListener(subscriptionId);
```

在这个示例中，客户端使用`onLogs`方法订阅了特定程序的日志。当日志中出现特定事件（例如，通过事件名称识别）时，它会被检测并处理。

### 注意事项

- 确保智能合约部署后的程序ID用于订阅事件。
- 在实际应用中，处理日志以识别事件时，可能需要解析日志字符串以提取事件数据。

Anchor框架和客户端库的版本更新可能会带来API的变化，因此建议查阅最新的文档以获取准确的实现细节。

https://github.com/coral-xyz/anchor

onLogs
https://solana-labs.github.io/solana-web3.js/classes/Connection.html#onLogs

or：
https://solana.com/docs/rpc/websocket/logssubscribe
