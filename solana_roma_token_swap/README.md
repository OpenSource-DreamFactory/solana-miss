## 部署说明
1，下载anchor框架，anchor build，anchor run，anchor deploy
demo：
https://dev.to/edge-and-node/the-complete-guide-to-full-stack-solana-development-with-react-anchor-rust-and-phantom-3291


2，将代码导入网页编译：
https://beta.solpg.io/

编译，部署，查看区块浏览器

solana代币部署文档：
https://github.com/tm01013/how-to-make-your-own-crypto/tree/main

代币部署说明：
1,
![3c6c17f11847776c1e1ce702dabc86b0](https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/5c4d0614-6408-4c9b-9b1c-eb71a9f75196)
2，
<img width="1123" alt="image" src="https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/c3d2ab04-5560-440b-a880-cb60bb3e9eb1">
3，
<img width="981" alt="image" src="https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/50be03e8-d139-4af2-9861-a95fdd355248">
4，
<img width="791" alt="image" src="https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/c0429a8e-171b-403a-b59e-6e661b922dbb">

5，
```
ubuntu@merlin-rpc-json-node:~/how-to-make-your-own-crypto$ cat metadata.json 
{
  "name": "ROMA Token",
  "symbol": "roma",
  "description": "roma",
  "image": "https://logolook.net/wp-content/uploads/2023/09/Roma-Logo.png",
  "attributes": [],
  "properties": {
    "files": [
      {
        "uri": "https://logolook.net/wp-content/uploads/2023/09/Roma-Logo.png",
        "type": "image/png"
      }
    ]
  }
}

```

6,上传Metadata


<img width="1115" alt="image" src="https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/b8587abf-1301-43a9-ac3a-b893dd05619c">

<img width="1123" alt="image" src="https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/a8f88e0c-dbe3-467c-a20a-7ca412a10925">



ROMA Token 合约：
ECKt9UeMb6mc1QkTkgysS8tSvKYdf6DY2oMNiyoVWJxh
https://solscan.io/token/ECKt9UeMb6mc1QkTkgysS8tSvKYdf6DY2oMNiyoVWJxh?cluster=devnet

## Programming Language

Rust

## Original Requirements

合约要求约定，平台的token名字叫：ROMA 兑换合约 注意：平台的token在兑换的时候价格固定为一个设定的值，例如0.5U 。合约逻辑：用户点击兑换按钮，合约的兑换方法执行如下 1.拿到最新的sol合约对usdt的价格X（这个价格由外界调用方法更新） 2.拿到平台设定的roma代币的价格Y，例如 0.5（roma的价格也由外界调用合约方法设置） 3.拿到用户输入的要兑换的Sol的数量Z 4.根据算法  X*Z / Y 计算可以兑换得到的 ROMA的数量K  ，扣除平台手续费（手续费按照sol数量收的百分比收取，初始的时候手续费为0 ） 5.合约内部给用户转账ROMA数量K  6.合约内部把sol转移到设定的地址J （这个地址需要支持外界设置，如果不设置就是合约部署者） 7.发出兑换事件，把事件抛出来~ 抛出的事件数据（用户地址，花费sol的数量，兑换的到roma的数量） 另外，需要注意， 1.一些参数设置，需要白名单管理员权限才可以调用。 2.兑换合约要支持开关，也就是管理员可以关闭合约兑换通道

## Project Name
```
solana_roma_token_swap
```

## User Stories

- 作为一个用户，我希望能够轻松兑换ROMA代币
- 作为一个管理员，我希望能够设置和调整代币价格
- 作为一个管理员，我希望能够开启或关闭兑换功能
- 作为一个用户，我希望在兑换时能看到明确的交易详情
- 作为一个用户，我希望兑换过程中的手续费用低

## Requirement Analysis

根据需求，我们需要开发一个基于Solana的智能合约，用于ROMA代币的兑换。合约需要能够处理价格更新、兑换逻辑、手续费收取、转账操作，并且提供管理员权限以调整参数和开关兑换功能。此外，合约应当能够发出兑换事件，供外部监听和处理。

## Requirement Pool

- ['P0', '实现基于Solana的智能合约，支持ROMA代币兑换']
- ['P0', '合约能够处理价格更新、兑换逻辑、手续费收取和转账操作']
- ['P1', '提供管理员权限以调整代币价格和开关兑换功能']
- ['P2', '合约能够发出兑换事件，供外部监听和处理']
- ['P2', '优化用户兑换体验，确保手续费用低、交易明确']

## UI Design draft

由于项目为智能合约，主要交互将通过区块链界面或集成的第三方平台进行。UI设计将侧重于清晰展示兑换信息、价格更新、手续费等关键信息。


### 部署合约地址
https://explorer.solana.com/tx/2hmBCZd5h4V6cU43cyZvdRd7PGUDZKX9tKm2WyF3G6poxYEhcot4gH3armPmoLcjhoS24xZDJq13BiZqv6vtKMng?cluster=devnet
