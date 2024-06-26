## 部署说明

将代码导入：
https://beta.solpg.io/

编译，部署，查看区块浏览器

![10dac726a4c5664fb5bfc4ab625f5c91](https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/696f8e86-b014-4a98-b7e3-5a4bf27bf5f1)

![d07efa87f7e5394628e2bce5346a3f4f](https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/ebd42afa-7436-4276-b8d6-38b55e4f661a)

![cddbc45ddddf0c0beb8a066e586c618f](https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/3f95155f-2d11-4102-8287-1756b4a9cbb5)

![ccd48decf502699ee091dd71a41c4af2](https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/39e1e961-867b-47bd-ae87-21e4eaf9b5de)

![404fc6a1917c00a51c8193d0f725692b](https://github.com/OpenSource-DreamFactory/solana-miss/assets/3462559/72e3fb23-3a49-4201-a19b-151819cf3a4f)


Rust

##

为投票合约设置 Roma 代币合约：如果你有一个使用 Roma 代币的投票系统，确保通过调用一个专门的函数（例如 set_roma_token_for_voting）来设置 Roma 代币合约地址。

## Requirements

投票合约:投票合约的作用是，在用户有了ROMA币之后。可以给自己喜欢的女明星刷票。 

合约的逻辑： 输入参数：miss在中心化服务器的ID，给miss投票所花费的roma代币数量。 

1.拿到miss在中心化服务器的用户ID 

2.拿到用户愿意给miss支付的roma 

3.根据支付的roma计算出来投票数 

4.给miss定义一个数据结构（miss的中心化服务器id，miss钱包，miss的票数），然后给票数加上3中计算出来的数据。 

5.抛出投票事件（miss的ID，miss地址，投票人地址，投票人支付的roma数量，投票数） 

另外，需要注意， 
1.一些参数设置，需要白名单管理员权限才可以调用。 

2.兑换合约要支持开关，也就是管理员可以关闭合约投票通道 

3.每一个roma代币代表多少个投票数，支持设置。默认一个roma一票 

4.提供根据miss的ID查询miss的票数的接口

## Project Name

```
solana_vote_contract
```

## User Stories

- 作为一个用户，我希望能够轻松地给我喜欢的女明星投票，以表达我的支持
- 作为一个管理员，我希望能够设置和调整投票规则，如关闭投票通道或设置ROMA币与票数的兑换比例
- 作为一个开发者，我希望能够通过接口查询特定女明星的票数，以便进行数据分析和展示

## Requirement Analysis

根据需求，我们需要实现一个在Solana区块链上运行的智能合约，允许用户使用ROMA币给喜欢的女明星投票。合约需要处理投票逻辑、票数计算、事件抛出等功能，并提供管理员权限设置和查询接口。考虑到安全性和用户体验，合约还应支持开关功能和兑换比例设置。

## Requirement Pool

- ['P0', '实现基本的投票逻辑和票数计算']
- ['P0', '设计并实现数据结构来存储女明星信息和票数']
- ['P1', '实现管理员权限设置，包括合约开关和兑换比例设置']
- ['P1', '提供查询特定女明星票数的接口']
- ['P2', '优化合约代码，确保安全性和效率']

## UI Design draft

由于项目为智能合约，不涉及前端UI设计。







